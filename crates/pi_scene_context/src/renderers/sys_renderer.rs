//! # 渲染管线核心系统
//!
//! 此模块实现了 pi_3d 渲染引擎的核心渲染管线，负责将场景数据转换为GPU可执行的渲染命令。
//!
//! ## 架构概述
//!
//! 渲染管线采用现代图形API最佳实践，通过ECS架构实现了高度模块化和可扩展的3D渲染系统。
//! 整个管线分为8个主要阶段，每个阶段由独立的ECS系统负责处理。
//!
//! ## 核心特性
//!
//! - **增量更新系统**: 使用脏标志机制避免不必要的资源重建
//! - **资源缓存**: 着色器、管线、绑定组的智能缓存机制
//! - **多Pass渲染**: 支持不透明、透明等不同渲染通道
//! - **实例化优化**: 高效的实例化渲染批处理
//! - **ECS集成**: 完全基于Bevy ECS框架设计
//!
//! ## 渲染管线流程
//!
//! ```text
//! 场景数据 → 绑定组创建 → 着色器编译 → 管线创建 → 绘制状态 → 对象收集 → GPU执行
//!    ↓          ↓           ↓          ↓         ↓          ↓         ↓
//! Pass      BindGroups   Shader   Pipeline  DrawState  DrawList  Render
//! ```
//!
//! ## 8个核心系统
//!
//! 1. **sys_pass_bind_groups** - GPU绑定组管理（4个set：场景/模型/材质/纹理）
//! 2. **sys_pass_shader_request_by_model** - 模型变化触发的着色器请求
//! 3. **sys_pass_shader** - 着色器编译与缓存
//! 4. **sys_pass_pipeline_request_by_renderer** - 渲染器变化触发的管线请求
//! 5. **sys_pass_pipeline** - 渲染管线创建与缓存
//! 6. **sys_pass_draw_modify_by_model** - 模型绘制状态管理
//! 7. **sys_pass_draw_modify_by_pass** - 最终绘制决策
//! 8. **sys_renderer_draws_modify** - 渲染对象收集、排序与批次优化

use std::{hash::Hasher, ops::Range, sync::Arc};
use pi_slotmap::Key;

use pi_scene_shell::{prelude::*, run_stage::EngineCustomPlugins};
use crate::{
    bindgroup::*, flags::*, geometry::{instance::instanced_buffer::*, prelude::*}, materials::prelude::*, meshes::prelude::*, pass::*, scene::prelude::*, skeleton::prelude::*, transforms::prelude::*, viewer::prelude::*
};

use super::{
    _set0_modify, _set1_modify, base::*, render_depth_and_stencil::*, render_object::RenderState, render_primitive::*, render_sort::*, render_target_state::*, renderer::*
};

/// GPU绑定组管理系统 - 渲染管线第一阶段
///
/// # 功能职责
/// 为每个渲染Pass创建和管理4个GPU绑定组（BindGroup），这些绑定组定义了着色器可以访问的数据：
/// - **set0**: 场景级数据（相机矩阵、灯光、环境贴图等）
/// - **set1**: 模型级数据（变换矩阵、骨骼动画、材质参数等）
/// - **set2**: 材质参数绑定组（用户自定义Uniform参数）
/// - **set3**: 纹理采样器绑定组（漫反射、法线、金属度等纹理）
///
/// # 触发条件
/// 监听 `PassBindGroupsDirty` 组件变化，在以下情况下触发重新绑定：
/// - 渲染Pass首次创建
/// - 材质发生变化
/// - 模型发生变化
/// - 场景环境发生变化
///
/// # 性能优化
/// - 绑定组缓存：基于内容哈希避免重复创建
/// - 增量更新：只重新创建受影响的绑定组
/// - 资源复用：通过资源管理器复用GPU资源
pub fn sys_pass_bind_groups(
    addeds: ComponentAdded<PassBindGroupsDirty>,   // 新增的绑定组脏标记
    changes: ComponentChanged<PassBindGroupsDirty>, // 变化的绑定组脏标记
    mut passes: Query<                               // 查询需要更新的Pass
        (ObjectID, &PassModelID, &PassMaterialID, &PassRendererID, &mut PassBindGroups, &mut PassFlagShader, &PassTag)
    >,
    renderers: Query<(&SceneID, &ViewerID)>,        // 渲染器查询（获取场景和视口ID）
    materials: Query<( &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &EffectTextureSamplersComp)>, // 材质资源查询
    models: Query<( &BindModel, &BindSkinValue, &SkeletonID, &ModelLightingIndexs, &ModelBindDefines )>, // 模型资源查询
    targets: Res<CustomRenderTargets>,                // 自定义渲染目标
    viewers: Query<&BindViewer>,                      // 视口绑定数据
    scenes: Query<(&BindSceneEffect, &SceneLightingInfos, &BRDFTexture, &BRDFSampler, &MainCameraOpaqueTarget, &MainCameraDepthTarget, &SceneShadowRenderTarget, Option<&SceneShadowInfos>, &EnvTexture, &EnvIrradiance, &EnvSampler)>, // 场景资源查询
    device: Res<PiRenderDevice>,                      // GPU设备
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>, // 绑定组布局管理器
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,            // 绑定组管理器
    bindpassindexs: Res<BindPassIndexPool>,           // Pass索引池
    mut errors: ResMut<ResErrorRecord>,               // 错误记录
    entitysets: Res<EntityFilterForComponentChanged>, // 实体变更过滤器
    // mut performance: ResMut<Performance>,           // 性能监控（已注释）
) {

    // 获取实体过滤器，避免重复处理同一实体
    let mut entities = entitysets.pop();

    // 遍历所有发生绑定组变化的Pass
    changes.iter().chain(addeds.iter()).for_each(|entity| {
        // 跳过已经处理过的实体
        if !entities.insert(entity) { return; }

        // 获取Pass的核心组件数据
        if let Ok((_id_pass, idmodel, idmat, idrenderer, mut bindgroups, mut flag, passidx)) = passes.get_mut(*entity) {

            let idmodel = idmodel.0;

            // 通过渲染器ID获取关联的场景和视口ID
            let (idscene, idviewer) = if let Ok((idscene, idviewer)) = renderers.get(idrenderer.0) {
                (idscene.0, idviewer.0)
            } else {
                // log::warn!("Bindgroups viewer Fail {:?}", idmodel);
                return;
            };

            // 获取Pass索引绑定信息
            let bind_passindex = if let Some(bindpassindex) = bindpassindexs.get(passidx.index()) {
                bindpassindex
            } else {
                // log::warn!("Bindgroups bind_passindex Fail {:?}", idmodel);
                return;
            };

            // 准备共享引用，避免闭包捕获问题
            let scenes = &scenes;
            let device = &device;
            let asset_mgr_bindgroup_layout = &asset_mgr_bindgroup_layout;
            let asset_mgr_bindgroup = &asset_mgr_bindgroup;
            let targets = &targets;
            let errors = &mut errors;
            let viewers = &viewers;
            let models = &models;

            // 获取材质相关资源
            if let Ok((effect_key, meta, bind, textures)) = materials.get(idmat.0) {
                // 准备着色器效果和纹理绑定
                let (bindtextures, effect) = _pass_effect_ready(
                    effect_key, textures, meta
                );

                // 处理有效的着色器效果
                if let Some((key_meta, meta)) = &effect {
                    let _need_set0 = true;

                    // 根据着色器元数据确定需要的绑定组
                    let need_set1 = BindDefines::need_bind_group_set1(meta.binddefines); // 模型绑定组
                    let need_set2 = bind.0.is_some();                                   // 材质参数绑定组
                    let need_set3 = meta.textures.len() > 0;                            // 纹理绑定组

                    // 创建set0：场景级绑定组（总是需要）
                    let set0 = {
                        let temp = _set0_modify(
                            idmodel, idscene, idviewer, meta,
                            viewers, scenes, device,
                            asset_mgr_bindgroup_layout, asset_mgr_bindgroup, targets,
                            bind_passindex,
                            errors
                        );
                        // 如果set0创建失败，重置绑定组并返回
                        if temp.is_none() {
                            if bindgroups.val().is_some() {
                                *bindgroups = PassBindGroups::new(None);
                                *flag = PassFlagShader;
                            }
                            return;
                        }
                        temp
                    };

                    let set1 = if need_set1 {
                        let temp = _set1_modify(
                            idmodel, key_meta, meta,
                            models, device, asset_mgr_bindgroup_layout, asset_mgr_bindgroup,
                            errors
                        );
                        if temp.is_none() {
                            if bindgroups.val().is_some() {
                                *bindgroups = PassBindGroups::new(None);
                                *flag = PassFlagShader;
                            }
                            // log::warn!("Bindgroups Fail set1 {:?}", idmodel);
                            return;
                        }
                        temp
                    } else { None };
                    
                    let set2 = if need_set2 {
                        let item = &bind.0.as_ref().unwrap().bind;
                        let key_bind_group = item.key_bind_group();
                        if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                            Some(Arc::new(BindGroupMaterial::new(BindGroupUsage::new(key_bind_group, bind_group), item.clone())))
                        } else {
                            errors.record(idmodel.index(), ErrorRecord::ERROR_PASS_SET2_FAIL);
                            // log::warn!("Bindgroups Fail set2 {:?}", idmodel);
                            return;
                        }
                    } else { None };

                    let set3 = if need_set3 {
                        if let Some(effect_texture_samplers) = bindtextures {
                            let temp = _set3_modify(
                                key_meta, meta, effect_texture_samplers,
                                device, asset_mgr_bindgroup_layout, asset_mgr_bindgroup 
                            );
                            if temp.is_none() {
                                if bindgroups.val().is_some() {
                                    *bindgroups = PassBindGroups::new(None);
                                    *flag = PassFlagShader;
                                }
                                errors.record(idmodel.index(), ErrorRecord::ERROR_PASS_SET3_FAIL);
                                return;
                            }
                            temp
                        } else {
                            if bindgroups.val().is_some() {
                                *bindgroups = PassBindGroups::new(None);
                                *flag = PassFlagShader;
                            }
                            return;
                        }
                    } else { None };

                    // log::warn!("Bindgroups Sccess {:?}", idmodel);
                    let data = BindGroups3D::create(set0, set1, set2, set3);
                    *bindgroups = PassBindGroups::new(Some(data));
                    *flag = PassFlagShader;
                } else {
                    // log::warn!("Bindgroups Fail effect {:?}", idmodel);
                }
            } else {
                // log::warn!("Bindgroups Fail materials {:?}", idmodel);
                if bindgroups.val().is_some() {
                    *bindgroups = PassBindGroups::new(None);
                    *flag = PassFlagShader;
                }
            }
        } else {
            // log::warn!("Bindgroups Fail Pass", );
        }
    });
    entitysets.push(entities);
}

/// 着色器请求触发系统 - 当模型几何体变化时触发的着色器重新编译请求
///
/// # 功能职责
/// 监听模型几何体相关组件的变化，当检测到以下变化时触发着色器重新编译：
/// - `GeometryID`: 几何体资源变化（网格替换、顶点数据更新等）
/// - `RenderAlignment`: 渲染对齐方式变化（影响顶点着色器逻辑）
///
/// # 工作流程
/// 1. 监听相关组件的变化事件
/// 2. 遍历所有使用该几何体的渲染Pass
/// 3. 标记这些Pass的几何体ID为脏
/// 4. 设置管线状态和着色器标志，触发后续的着色器重新编译
///
/// # 性能优化
/// - 只在几何体实际变化时触发重新编译
/// - 避免无关的Pass被重新处理
/// - 通过脏标志系统实现增量更新
pub fn sys_pass_shader_request_by_model(
    addeds0: ComponentAdded<GeometryID>,         // 新增的几何体组件
    changes0: ComponentChanged<GeometryID>,       // 变化的几何体组件
    addeds1: ComponentAdded<RenderAlignment>,    // 新增的渲染对齐组件
    changes1: ComponentChanged<RenderAlignment>,  // 变化的渲染对齐组件
    // 查询使用几何体的模型 几何体ID和关联的Pass列表
    models: Query<(&GeometryID, &PassIDs)>,
    geoaddeds: ComponentAdded<VertexBufferLayoutsComp>,  // 新增的顶点布局组件
    geochanges: ComponentChanged<VertexBufferLayoutsComp>,// 变化的顶点布局组件
    geometrys: Query<(Entity, &MeshID)>,               // 几何体实体查询
    mut passes: Query<(&mut PassGeometryID, &mut PassPipelineStateDirty, &mut PassFlagShader)>, // 需要更新的Pass
    entitysets: Res<EntityFilterForComponentChanged>,   // 实体变更过滤器
) {
    // 性能计时代码（已注释）
    // let time1 = pi_time::Instant::now();

    // 第一轮：处理模型组件的变化
    let mut entities = entitysets.pop();
    addeds0.iter().chain(addeds1.iter()).chain(changes0.iter()).chain(changes1.iter()).for_each(|entity| {
        // 避免重复处理同一实体
        if !entities.insert(entity) { return; }

        // 获取发生变化模型的几何体ID和关联的Pass列表
        if let Ok((id_geo, passids)) = models.get(*entity) {
            // 遍历所有使用该几何体的Pass
            passids.0.iter().for_each(|id| {
                if let Ok((mut idgeometry, mut flagpipeline, mut flagshader)) = passes.get_mut(*id) {
                    // 更新Pass的几何体ID
                    *idgeometry = PassGeometryID(id_geo.0);
                    // 标记管线状态需要更新
                    *flagpipeline = PassPipelineStateDirty;
                    // 标记着色器需要重新编译
                    *flagshader = PassFlagShader;
                }
            });
        }
    });

    // 第二轮：处理顶点布局组件的变化
    let mut entities = entitysets.pop();
    geoaddeds.iter().chain(geochanges.iter()).for_each(|entity| {
        if !entities.insert(entity) { return; }

        // 获取变化的几何体实体
        if let Ok((entity, idmesh)) = geometrys.get(*entity) {
            // 验证该几何体确实被模型使用
            if let Ok((id_geo, passids)) = models.get(idmesh.0) {
                if entity == id_geo.0 {
                    // 更新所有使用该几何体的Pass
                    passids.0.iter().for_each(|id| {
                        if let Ok((mut idgeometry, mut flagpipeline, mut flagshader)) = passes.get_mut(*id) {
                            *idgeometry = PassGeometryID(id_geo.0);
                            *flagpipeline = PassPipelineStateDirty;
                            *flagshader = PassFlagShader;
                        }
                    });
                }
            }
        }
    });

    entitysets.push(entities);
    // 性能日志输出（已注释）
    // log::debug!("SysPassShaderRequestByModel: {:?}", pi_time::Instant::now() - time1);
}

/// 着色器编译与缓存系统 - 渲染管线第三阶段
///
/// # 功能职责
/// 根据Pass标志重新编译或缓存着色器程序，这是渲染管线的关键阶段：
/// - 解析着色器源代码并编译为GPU可执行程序
/// - 构建着色器缓存键以避免重复编译
/// - 处理材质变体和几何体布局的不同组合
/// - 管理着色器资源的生命周期
///
/// # 编译流程
/// 1. 根据材质和几何体构建着色器哈希键
/// 2. 查找缓存，未命中则触发编译
/// 3. 处理编译错误并记录日志
/// 4. 缓存编译成功的着色器程序
/// 5. 标记管线状态为脏，触发后续管线创建
///
/// # 性能优化
/// - 智能缓存系统避免重复编译相同着色器
/// - 异步编译减少主线程阻塞
/// - 错误处理机制避免编译失败影响其他Pass
pub fn sys_pass_shader(
    addeds: ComponentAdded<PassFlagShader>,
    changes: ComponentChanged<PassFlagShader>,
    models: Query<&RenderAlignment>,
    geometrys: Query<&VertexBufferLayoutsComp>, 
    materials: Query<( &AssetKeyShaderEffect, &AssetResShaderEffectMeta )>,
    mut passes: Query<
        (ObjectID, &DisposeReady, &PassModelID, &PassGeometryID, &PassMaterialID, &PassBindGroups, &mut PassShader, &mut PassPipelineStateDirty),
    >,
    assets: Res<ShareAssetMgr<Shader3D>>,
    device: Res<PiRenderDevice>,
    engineopt: Res<EngineCustomPlugins>,
    entitysets: Res<EntityFilterForComponentChanged>,
) {
    // let time1 = pi_time::Instant::now();
    let mut entities = entitysets.pop();
    changes.iter().chain(addeds.iter()).for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok((id_pass, disposeready, id_model, id_geo, idmat, bindgroups, mut old_shader, mut flagpipeline)) = passes.get_mut(*entity) {

            if disposeready.0 == true { return; }
        
            if let Ok((effect_key, meta)) = materials.get(idmat.0) {
                let key_meta = &effect_key.0;
                let meta = &meta.0;
                if let (Some(meta), Some(bindgroups)) = (meta, bindgroups.val()) {
                    match (models.get(id_model.0), geometrys.get(id_geo.0)) {
                        (Ok(renderalignment), Ok(vb)) => {
    
                            let limit = device.0.limits();
                            if vb.0.attrcount as u32 <= limit.max_vertex_attributes && vb.0.desccount as u32 <= limit.max_vertex_buffers {
                                let renderalignment = renderalignment.shader_tag(false);
                                if let Ok(shader) = shader(
                                    id_pass, meta, key_meta, vb, bindgroups, renderalignment, &assets, &device, &engineopt
                                ) {
                                    if let Some(old) = &old_shader.0 {
                                        if old.key() != shader.key() {
                                            *old_shader = PassShader(Some(shader));
                                            *flagpipeline = PassPipelineStateDirty;
                                        }
                                    } else {
                                        *old_shader = PassShader(Some(shader));
                                        *flagpipeline = PassPipelineStateDirty;
                                    }
                                    return;
                                }
                            }
                        },
                        _ => {
                            log::warn!("Shader Fail Geometry");
                        }
                    };
                    if old_shader.0.is_some() {
                        *old_shader = PassShader(None);
                        *flagpipeline = PassPipelineStateDirty;
                    }
                }
            }
        }
    });
    entitysets.push(entities);

    // log::debug!("SysPassShaderRequestByPass: {:?}", pi_time::Instant::now() - time1);
}

/// 渲染管线请求触发系统 - 渲染器参数变化时的管线重建请求
///
/// # 功能职责
/// 监听渲染器相关组件的变化，触发对应的渲染管线重建请求：
/// - **PassRendererID变化**: 渲染通道与渲染器关联关系变化
/// - **RenderState变化**: 渲染状态配置变化
/// - **FlagRendererParamForPipeline变化**: 管线参数标志变化
///
/// # 触发机制
/// 系统通过两轮处理来覆盖所有可能的变化场景：
///
/// ## 第一轮：直接相关变化处理
/// 处理直接影响渲染管线的组件变化：
/// - PassRendererID组件新增或变更
/// - RenderState组件新增或变更
/// - 直接标记对应的Pass为脏状态
///
/// ## 第二轮：渲染器参数变化处理
/// 处理渲染器参数变化对管线的影响：
/// - 渲染器参数标志位变化
/// - 遍历渲染器关联的所有模型
/// - 通过PassTag确定影响的渲染通道
/// - 批量触发对应通道的管线重建
///
/// # 模型过滤逻辑
/// 系统通过双重模型列表进行精确过滤：
/// - **ModelList**: 视景体正常包含的模型列表
/// - **ForceIncludeModelList**: 强制包含的模型列表（不受视景体裁剪影响）
/// - 只要模型在任一列表中，就会触发管线重建
///
/// # 性能优化
/// - 使用实体过滤器避免重复处理同一实体
/// - 批量处理减少单独调用开销
/// - 只处理启用的渲染器（param.enable.0检查）
pub fn sys_pass_pipeline_request_by_renderer(
    passaddeds: ComponentAdded<PassRendererID>,             // PassRendererID组件新增事件
    passaddeds2: ComponentAdded<RenderState>,               // RenderState组件新增事件
    passchanges: ComponentChanged<PassRendererID>,          // PassRendererID组件变更事件
    passchanges2: ComponentChanged<RenderState>,            // RenderState组件变更事件
    changes0: ComponentAdded<FlagRendererParamForPipeline>, // 管线参数标志新增事件
    changes: ComponentChanged<FlagRendererParamForPipeline>,// 管线参数标志变更事件
    renderers: Query<(&RendererParam, &ViewerID, &PassTag)>,// 渲染器查询：参数、视景体ID、通道标签
    viewers: Query<(&ModelList, &ForceIncludeModelList)>,   // 视景体查询：正常模型列表、强制包含模型列表
    modelspass: Query<(Entity, &PassIDs)>,                 // 模型通道关联查询
    mut passes: Query<&mut PassPipelineStateDirty>,         // 管线脏标记查询（可变）
    entitysets: Res<EntityFilterForComponentChanged>,       // 实体变更过滤器资源
) {
    // 第一轮：处理直接相关组件的变化
    // 合并所有相关组件的变更事件，统一处理
    let mut entities = entitysets.pop();
    passaddeds.iter().chain(passaddeds2.iter()).chain(passchanges.iter()).chain(passchanges2.iter()).for_each(|entity| {
        // 避免重复处理同一实体
        if !entities.insert(entity) { return; }
        // 直接将对应的Pass标记为管线脏状态，触发重建
        if let Ok(mut flag) = passes.get_mut(*entity) {
            *flag = PassPipelineStateDirty;
        }
    });

    // 第二轮：处理渲染器参数变化
    let mut entities = entitysets.pop();
    changes.iter().chain(changes0.iter()).for_each(|entity| {
        // 避免重复处理同一实体
        if !entities.insert(entity) { return; }

        // 获取渲染器的配置信息
        if let Ok((param, idviewer, passtag)) = renderers.get(*entity) {
            // 只处理启用的渲染器
            if param.enable.0 {
                // 获取视景体的模型列表
                if let Ok((modellist, forcemodels)) = viewers.get(idviewer.0) {
                    // 遍历所有模型，查找通过该渲染器渲染的模型
                    modelspass.iter().for_each(|(idmodel, passid)| {
                        // 检查模型是否在视景体中或强制包含列表中
                        if modellist.0.contains(&idmodel) || forcemodels.0.contains(&idmodel) {
                            // 触发对应渲染通道的管线重建
                            _pass_pipeline_request_by_renderer(passid.0[passtag.index()], &mut passes);
                        }
                    });
                }
            }
        }
    });
    entitysets.push(entities);
}

/// 渲染管线请求辅助函数 - 标记指定Pass为管线脏状态
///
/// # 功能职责
/// 这是一个内部辅助函数，用于标记指定的渲染通道需要重建管线：
/// - 直接设置PassPipelineStateDirty标志
/// - 触发后续的管线重建流程
/// - 统一管线脏标记的设置接口
///
/// # 使用场景
/// 在以下情况下会调用此函数：
/// - 渲染器参数变化时
/// - 渲染状态配置变化时
/// - 几何体绑定关系变化时
///
/// # 参数说明
/// - `passid`: 需要标记管线脏状态的渲染通道实体ID
/// - `passes`: 可变的PassPipelineStateDirty组件查询
fn _pass_pipeline_request_by_renderer(
    passid: Entity,
    passes: &mut Query<&mut PassPipelineStateDirty>,
) {
    // 尝试获取指定Pass的可变引用并设置脏标记
    if let Ok( mut flag ) = passes.get_mut(passid) {
        *flag = PassPipelineStateDirty;
    }
}

    /// 渲染管线创建系统 - 渲染管线第五阶段
///
/// # 功能职责
/// 创建和管理GPU渲染管线对象，这是渲染管线的核心配置阶段：
/// - 根据着色器、绑定组、几何体布局创建完整的渲染管线
/// - 处理渲染状态（深度测试、模板测试、混合模式等）
/// - 缓存管线对象以避免重复创建
/// - 适配不同的几何体顶点布局
///
/// # 管线组成
/// 渲染管线包含以下关键组件：
/// - **着色器程序**: 顶点着色器和片段着色器
/// - **绑定组布局**: 定义GPU资源绑定方式
/// - **顶点布局**: 定义几何体顶点数据结构
/// - **渲染状态**: 深度测试、模板、混合、裁剪等设置
/// - **多采样配置**: MSAA抗锯齿设置
///
/// # 缓存机制
/// 通过管线哈希键实现智能缓存：
/// - 相同配置的管线复用，减少GPU资源创建
/// - 增量更新只重建变化的管线
/// - 支持管线热重载和调试
pub fn sys_pass_pipeline(
    addeds: ComponentAdded<PassPipelineStateDirty>,    // 新增的管线脏标记
    changes: ComponentChanged<PassPipelineStateDirty>,  // 变化的管线脏标记
    renderers: Query<&RendererParam>,
    models: Query<&GeometryID>,
    geometrys: Query<&VertexBufferLayoutsComp>,
    mut passes: Query<
        (
            ObjectID, &DisposeReady, &PassModelID, &PassBindGroups, &PassShader, &mut PassPipeline, &PassRendererID,
            &RenderState, &mut PassDrawDirty
        )
    >,
    assets: ResMut<ShareAssetMgr<Pipeline3D>>,
    device: Res<PiRenderDevice>,
    mut errors: ResMut<ResErrorRecord>,
    entitysets: Res<EntityFilterForComponentChanged>,
) {
    // let time1 = pi_time::Instant::now();
    let mut entities = entitysets.pop();
    changes.iter().chain(addeds.iter()).for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok((
            id_pass, disposeready, id_model, bindgroups, shader, mut oldpipeline, idrenderer,
            renderstate, mut flag
        )) = passes.get_mut(*entity) {
            if disposeready.0 == true { return; }
            // log::warn!("SysPipeline: 0 Pass");
            if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                // log::warn!("SysPipeline: 1 Pass");
                if let Ok(id_geo) = models.get(id_model.0) {
                    // log::warn!("SysPipeline: 2 Pass {:?}", (geometrys.get(id_geo.0).is_ok(), renderers.get(idrenderer.0).is_ok()));
                    match (geometrys.get(id_geo.0), renderers.get(idrenderer.0)) {
                        (Ok(vb), Ok(param)) => {
                            let blend = renderstate.blend.clone(); // if blendenable.0 { blend.clone() } else { ModelBlend::default() };
                            if let Ok(pipeline) = pipeline(
                                shader, bindgroups, vb, &param.colorformat.0, &param.depthstencilformat.0,
                                blend, &renderstate.depth, &renderstate.stencil,
                                &renderstate.primitive,
                                id_pass, & assets, &device
                            ) {
                                // log::warn!("SysPipeline: {:?}", (passtag, idrenderer.0));
                                // *oldpipeline = PassPipeline(Some(pipeline));
                                if let Some(old) = &oldpipeline.0 {
                                    if old.key() != pipeline.key() {
                                        *oldpipeline = PassPipeline(Some(pipeline));
                                        *flag = PassDrawDirty;
                                    }
                                } else {
                                    *oldpipeline = PassPipeline(Some(pipeline));
                                    *flag = PassDrawDirty;
                                }
                                return;
                            } else {
                                errors.record(id_model.0.index(), ErrorRecord::ERROR_PASS_PIPELINE_FAIL);
                            }
                        },
                        _ => {}
                    }
                } else {}
            }
            
            if oldpipeline.0.is_some() {
                *oldpipeline = PassPipeline(None);
                *flag = PassDrawDirty;
            }
        }
    });
    entitysets.push(entities);
    // // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
}


/// 模型渲染状态变更触发系统 - 渲染管线第六阶段
///
/// # 功能职责
/// 监听模型渲染相关组件的变化，触发对应渲染通道的绘制状态更新：
/// - **RenderGeometryEable变化**: 模型渲染启用/禁用状态变化
/// - **IndiceRenderRange变化**: 索引渲染范围变化
/// - **VertexRenderRange变化**: 顶点渲染范围变化
///
/// # 变更监听机制
/// 系统监听三种关键的渲染状态变化：
///
/// ## 渲染启用状态变化
/// RenderGeometryEable组件控制模型是否参与渲染：
/// - true: 模型启用渲染
/// - false: 模型禁用渲染
/// - 变化时需要重新评估对应的PassDraw状态
///
/// ## 渲染范围变化
/// 索引和顶点渲染范围定义了实际参与渲染的数据范围：
/// - IndiceRenderRange: 控制索引缓冲区的渲染范围
/// - VertexRenderRange: 控制顶点缓冲区的渲染范围
/// - 范围变化会影响实际的绘制调用
///
/// # 影响传播机制
/// 当模型的渲染状态发生变化时：
/// 1. 查找该模型关联的所有渲染通道（PassIDs）
/// 2. 将所有相关通道标记为绘制脏状态（PassDrawDirty）
/// 3. 触发后续的绘制数据重新收集
///
/// # 性能优化
/// - 使用实体过滤器避免重复处理同一实体
/// - 批量处理减少单独调用开销
/// - 链式迭代器高效合并多种变更事件
pub fn sys_pass_draw_modify_by_model(
    models: Query<&PassIDs>,                              // 模型关联的渲染通道ID查询
    changes0: ComponentChanged<RenderGeometryEable>,      // 渲染启用状态变更事件
    changes1: ComponentChanged<IndiceRenderRange>,        // 索引渲染范围变更事件
    changes2: ComponentChanged<VertexRenderRange>,        // 顶点渲染范围变更事件
    mut passes: Query<&mut PassDrawDirty>,                // 绘制脏标记查询（可变）
    entitysets: Res<EntityFilterForComponentChanged>,     // 实体变更过滤器资源
) {
    // 从过滤器获取当前帧的实体集合
    let mut entities = entitysets.pop();

    // 合并处理所有类型的变更事件
    // 使用链式迭代器高效合并三种不同的组件变更事件
    changes0.iter().chain(changes1.iter()).chain(changes2.iter()).for_each(|entity| {
        // 避免重复处理同一实体
        if !entities.insert(entity) { return; }

        // 获取模型关联的所有渲染通道
        if let Ok(passids) = models.get(*entity) {
            // 遍历所有相关通道，标记为绘制脏状态
            passids.0.iter().for_each(|id| {
                if let Ok(mut drawdirty) = passes.get_mut(*id) {
                    drawdirty.set_changed();
                }
            });
        }
    });

    // 将处理过的实体集合重新放回过滤器
    entitysets.push(entities);
}

/// 渲染通道绘制状态验证系统 - 渲染管线第七阶段
///
/// # 功能职责
/// 验证和更新渲染通道的实际绘制状态，确保只有完全准备好的通道才会参与最终渲染：
/// - **资源完整性检查**: 验证绑定组和管线是否准备就绪
/// - **模型状态验证**: 检查模型渲染启用和销毁状态
/// - **几何体可用性检查**: 验证几何体数据是否有效
/// - **PassDraw状态更新**: 根据检查结果更新最终绘制状态
///
/// # 检查验证机制
/// 系统采用多层验证机制确保绘制状态的准确性：
///
/// ## 第一层：资源准备状态检查
/// 验证渲染通道的基础资源是否准备就绪：
/// - PassBindGroups: GPU绑定组是否创建完成
/// - PassPipeline: 渲染管线是否创建完成
/// - 任一资源未就绪时，禁用该通道的绘制
///
/// ## 第二层：模型状态检查
/// 验证关联模型的渲染状态：
/// - RenderGeometryEable: 模型是否启用渲染
/// - DisposeReady: 模型是否准备销毁
/// - 模型禁用或待销毁时，禁用绘制
///
/// ## 第三层：几何体数据检查
/// 验证几何体数据的实际可用性：
/// - 检查RenderGeometryComp是否存在且有效
/// - 调用isok()方法验证几何体数据完整性
/// - 几何体数据无效时禁用绘制
///
/// # 状态更新逻辑
/// 系统采用严格的布尔逻辑更新PassDraw状态：
/// - 所有必要条件满足时：设置PassDraw(true)
/// - 任一条件不满足时：设置PassDraw(false)
/// - 只有状态真正变化时才更新，避免不必要的状态变更
///
/// # 性能优化
/// - 使用实体过滤器避免重复处理
/// - 早期返回机制，条件不满足时立即跳过后续检查
/// - 短路求值逻辑，减少不必要的查询操作
pub fn sys_pass_draw_modify_by_pass(
    models: Query<(&GeometryID, &RenderGeometryEable, &DisposeReady)>,     // 模型几何体ID、渲染启用状态、销毁准备状态查询
    geometrys: Query<&RenderGeometryComp>,                                 // 几何体组件查询
    changes: ComponentChanged<PassDrawDirty>,                               // 绘制脏标记变更事件
    mut passes: Query<(&PassModelID, &PassBindGroups, &PassPipeline, &mut PassDraw)>, // 渲染通道完整状态查询
    entitysets: Res<EntityFilterForComponentChanged>,                      // 实体变更过滤器资源
) {
    // 获取当前帧待处理的实体集合
    let mut entities = entitysets.pop();

    // 遍历所有发生变化的渲染通道
    changes.iter().for_each(|entity| {
        // 避免重复处理同一实体
        if !entities.insert(entity) { return; }

        // 获取渲染通道的完整状态信息
        if let Ok((id_model, bindgroups, pipeline, mut old_draw)) = passes.get_mut(*entity) {
            // 第一层检查：基础资源准备状态
            // 确保绑定组和管线都已经准备就绪
            if let (Some(_), Some(_)) = (bindgroups.val(), pipeline.val()) {
                // 第二层检查：模型状态
                if let Ok((id_geo, geoenable, disposed)) = models.get(id_model.0) {
                    // 检查模型是否启用渲染且未准备销毁
                    if geoenable.0 == false || disposed.0 == true {
                        // 模型禁用或待销毁时，禁用绘制（仅在当前状态为启用时更新）
                        if old_draw.val() {
                            *old_draw = PassDraw(false);
                        };
                        return; // 早期返回，无需进行几何体检查
                    }

                    // 第三层检查：几何体数据可用性
                    if let Ok(RenderGeometryComp(Some(rendergeo))) = geometrys.get(id_geo.0.clone()) {
                        // 检查几何体数据是否完整有效
                        if rendergeo.isok() {
                            // 所有条件满足，启用绘制
                            *old_draw = PassDraw(true);
                        } else {
                            // 几何体数据无效，禁用绘制
                            *old_draw = PassDraw(false);
                        }
                    } else {
                        // 几何体组件不存在，禁用绘制
                        *old_draw = PassDraw(false);
                    }
                }
            } else {
                // 基础资源未就绪，禁用绘制
                *old_draw = PassDraw(false);
            }
        }
    });

    // 将处理过的实体集合重新放回过滤器
    entitysets.push(entities);
}

/// 渲染对象收集与排序系统 - 渲染管线第八阶段（最终阶段）
///
/// # 功能职责
/// 这是渲染管线的最终阶段，负责：
/// - 收集所有可见的渲染对象（视口剔除后）
/// - 按渲染队列参数和透明度进行排序
/// - 实例化数据合并与批次优化
/// - 生成最终的GPU绘制命令列表
///
/// # 处理流程
/// 1. **对象收集**: 从视口剔除结果中收集可见对象
/// 2. **分类排序**: 分离透明和不透明对象，按距离和队列参数排序
/// 3. **实例化合并**: 合并相同网格的实例，减少Draw Call数量
/// 4. **批处理优化**: 根材质和几何体进行批次分组
/// 5. **命令生成**: 生成最终的绘制命令列表供GPU执行
///
/// # 性能特性
/// - **深度排序**: 正确处理透明物体的渲染顺序
/// - **实例化优化**: 大幅减少GPU状态切换
/// - **批处理**: 合并多个小对象为单个批次
/// - **视口剔除**: 只处理摄像机可见的对象
pub fn sys_renderer_draws_modify(
    mut renderers: Query< ( ObjectID, &SceneID, &ViewerID, &mut Renderer, &PassTag, &RendererParam, &RendererRenderTargetKey, Option<&mut CrossDrawList> ) >, // 渲染器查询
    viewers: Query< (&ModelListAfterCulling, &ViewerGlobalPosition, &ViewerDirection, &DisposeReady, &ViewerDistanceCompute), >, // 视口和剔除结果
    scenes: Query< (&BatchParamOpaque, &BatchParamTransparent) >, // 批处理参数
    models: Query<                                              // 模型组件查询
        (
            &GlobalEnable, &GlobalMatrix, &RenderQueueSortParam, &InstancedMeshTransparentSortCollection,
            &PassIDs, &GeometryID, &IndiceRenderRange, &VertexRenderRange, &RenderGeometryEable,
        )
    >,
    passes: Query<
        (&PassRendererID, &PassBindGroups, &PassPipeline)
    >,
    geometrys: Query<&RenderGeometryComp>,
    mut performance: ResMut<Performance>,
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
    mut combinebuffer: ResMut<CombineBuffer>,
    engineopt: Res<EngineCustomPlugins>,
    mut rendertargets: ResMut<CustomRenderTargets>,
) {
    // 性能监控：开始计时（如果启用调试模式）
    if performance.debug { performance.t_drawobjs = pi_time::Instant::now(); }

    // 初始化绘制对象列表 - 预分配容量以避免动态扩容
    let mut opaque_list: Vec<DrawTmpRef> = Vec::with_capacity(4096);      // 不透明对象列表
    let mut transparent_list: Vec<DrawTmpRef> = Vec::with_capacity(4096);  // 透明对象列表

    // 实例化批处理相关变量
    let mut lastinsdata: EVerteicesInstance = EVerteicesInstance::default(); // 上一次的实例化数据
    let mut lastdraw: Option<DrawTmpRef> = None;                            // 上一次的绘制对象引用

    // 重置性能计数器
    performance.drawcalls = 0;

    // 遍历所有渲染器，每个渲染器处理一个渲染通道
    renderers.iter_mut().for_each(|(_id_renderer, idscene, id_viewer, mut renderer, passtag, param, rendertargetkey, mut crossdraws)| {
        // 第一阶段：渲染器初始化和清理
        // 清理上一帧的渲染目标资源
        if let Some(rendertargetkey) = rendertargetkey.0 {
            rendertargets.delete(rendertargetkey);
        }

        // 清理渲染器的绘制列表
        renderer.clear();

        // 配置交叉绘制列表（如果存在）- 用于跨渲染器的特殊绘制需求
        if let Some(crossdraws) = &mut crossdraws {
            crossdraws.draw_list.list.clear();

            // 配置自动清除选项
            crossdraws.clear_color   = if param.auto_clear_color.0   { Some(param.color_clear.color())  } else { None };
            crossdraws.clear_depth   = if param.auto_clear_depth.0   { Some(param.depth_clear.0)        } else { None };
            crossdraws.clear_stencil = if param.auto_clear_stencil.0 { Some(param.stencil_clear.0)      } else { None };
        }

        // 检查渲染器是否启用 - 禁用的渲染器跳过处理
        if param.enable.0 == false { return; }

        // 初始化当前渲染器的处理变量
        let mut count_vertex = 0;      // 顶点计数器
        let mut countmesh = 0;         // 网格计数器
        let mut clear_draw: Option<DrawObj> = None; // 清除绘制对象（用于mesh_as_clear功能）
        opaque_list.clear();           // 清空不透明对象列表
        transparent_list.clear();      // 清空透明对象列表
        // 第二阶段：视景体和场景数据获取
        // 获取视景体剔除后的模型列表和场景批处理参数
        if let (Ok((list_model, viewposition, viewdirection, disposed, distancecomp)), Ok((batchopaque, batchtransparent))) = (viewers.get(id_viewer.0), scenes.get(idscene.0)) {
            // 检查视景体是否已销毁 - 销毁的视景体跳过处理
            if disposed.0 { return; }

            // 重置网格计数器
            countmesh = 0;

            // 转换摄像机位置和方向为元组格式，便于距离计算
            let viewposition  = (viewposition.0.x , viewposition.0.y , viewposition.0.z  );
            let viewdirection = (viewdirection.0.x, viewdirection.0.y, viewdirection.0.z );

            // 第三阶段：遍历视景体裁剪后的可见模型
            for id_obj in list_model.0.iter() {
                // 获取模型的所有渲染相关组件
                if let Ok((
                    globalenable, nodeposition, rendersort, instancessortinfo,
                    passids, idgeometry, indicerange, vertexrange, geoenable
                )) = models.get(id_obj.clone()) {
                    let passids = &passids.0;

                    // 第四阶段：模型可见性和渲染通道过滤
                    // 检查模型是否启用渲染且全局启用
                    if geoenable.0 && globalenable.0 == true && passtag.index() < passids.len() {
                        let idpass = passids[passtag.index()];

                        // 检查几何体数据是否存在且有效
                        if let Ok(RenderGeometryComp(Some(rendergeo))) = geometrys.get(idgeometry.0) {
                            let index = 0;
                            let is_transparent = param.blend.0; // 根据渲染器参数确定是否为透明渲染

                            // 第五阶段：渲染通道资源验证
                            // 验证渲染通道的资源是否准备就绪
                            if let Ok((passrendererid, bindgroups, pipeline)) = passes.get(idpass) {
                                // 检查渲染通道是否属于当前渲染器且资源已准备好
                                if let (true, Some(bindgroups), Some(pipeline)) = (passrendererid.0 == _id_renderer, bindgroups.val(), pipeline.val()) {
                                    // 第六阶段：距离计算（用于排序优化）
                                    // 根据透明度类型和批处理配置决定是否计算距离
                                    let mut distance = 0.;
                                    if is_transparent {
                                        // 透明对象：如果批处理配置要求距离排序，则计算到摄像机的距离
                                        if batchtransparent.0.distance {
                                            distance = distancecomp.distance(&viewposition, &viewdirection, &nodeposition.xyz());
                                        }
                                    } else {
                                        // 不透明对象：如果批处理配置要求距离排序，则计算距离
                                        if batchopaque.0.distance {
                                            distance = distancecomp.distance(&viewposition, &viewdirection, &nodeposition.xyz());
                                        }
                                    }

                                    // 第七阶段：绘制对象分类收集
                                    // 检查当前模型是否为特殊清除网格（mesh_as_clear功能）
                                    if *id_obj != renderer.mesh_as_clear {
                                        // 普通渲染对象：调用collect_draw函数进行分类收集
                                        collect_draw(
                                            is_transparent, index, rendergeo, bindgroups, pipeline, indicerange, vertexrange, distance,
                                            rendersort, &mut opaque_list, &mut transparent_list, &instancessortinfo, distancecomp, &viewposition, &viewdirection
                                        );
                                    } else {
                                        // 特殊清除网格：创建用于屏幕清除的绘制对象
                                        let vertex = vertexrange.apply(rendergeo);
                                        let indices = indicerange.apply(rendergeo);
                                        let vertexcount = if let Some(indices) = &indices {
                                            indices.value_range().end - indices.value_range().start
                                        } else { vertex.end - vertex.start };

                                        // 只有顶点数量大于0时才创建清除对象
                                        if vertexcount == 0 { } else {
                                            clear_draw = Some(DrawObj {
                                                pipeline:   Some(pipeline.clone()),
                                                bindgroups: bindgroups.groups(),
                                                vertices:   rendergeo.vertices(),
                                                instances:  rendergeo.instances(),
                                                vertex:     vertexrange.apply(rendergeo),
                                                indices,
                                            });
                                        }
                                    }
                                } else {
                                    // log::warn!("PassDraw Renderer Error {:?}", (passtag));
                                }
                            } else {
                                // log::warn!("PassDraw Error {:?}", (passtag));
                            }
                            countmesh += 1;
                            // log::warn!("OK {:?}", id_obj);
                        } else {
                            // log::warn!("Fail rendergeo {:?}", (idgeometry.0, id_obj));
                        }
                    } else {
                        // log::warn!("Fail rendergeo {:?}", (geoenable.0, globalenable.0));
                    }
                } else {
                    // log::warn!("models.get Fail {:?}", id_obj);
                }
            }

            // 第八阶段：绘制列表配置和清除对象处理
            // 确定使用的绘制列表：交叉绘制列表或渲染器默认绘制列表
            let draws = if let Some(crossdraws) = &mut crossdraws {
                &mut crossdraws.draw_list
            } else {
                &mut renderer.draws
            };

            // 设置视口参数
            draws.viewport = param.viewport.val();

            // 如果存在清除绘制对象，优先添加到绘制列表开头
            if let Some(draw) = clear_draw {
                arr_push(&mut draws.list, Arc::new(draw));
            }

            // 第九阶段：对象排序优化
            // 不透明对象：按渲染队列参数和状态进行排序（优化GPU状态切换）
            opaque_list.sort_by(|a, b| DrawTmpRef::cmp_opaque(a, b));
            // 透明对象：按距离和透明度排序（确保正确的混合顺序）
            transparent_list.sort_by(|a, b| DrawTmpRef::cmp_transparent(a, b));

            // 重置批处理状态变量
            lastinsdata.reset();
            lastdraw = None;

            // 第十阶段：不透明对象批处理
            // 遍历不透明对象列表，进行实例化批处理优化
            opaque_list.drain(..).for_each(|drawinfo| {
                if let Some(tempdraw) = lastdraw.take() {
                    // 批处理条件检查
                    let batchcount_ok   = tempdraw.instancecount() + drawinfo.instancecount() < engineopt.max_instance_batch_count;     // 实例数量限制
                    let batchmem_ok     = tempdraw.can_batch_instance_memory(&drawinfo, true);                                           // 内存布局兼容性
                    let batchmaxsize_ok = combinebuffer.usedsize() + drawinfo.instancedatasize() < combinebuffer.maxcombinesize;      // 合并缓冲区大小限制
                    let batchcomb_ok    = combinebuffer.combinecommon(drawinfo.instancedatasize());                                      // 合并操作可行性

                    if batchcount_ok && batchmem_ok && batchmaxsize_ok && batchcomb_ok {
                        // 所有条件满足：可以与当前批次合并
                        _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                        lastdraw = Some(tempdraw);
                    } else {
                        // 条件不满足：完成当前批次，开始新批次
                        collect_draw_batch(&mut combinebuffer, tempdraw, &lastinsdata, draws, &mut allocator, &device, &queue, &mut count_vertex);
                        lastinsdata.reset();
                        lastinsdata.data.start = combinebuffer.usedsize();
                        _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                        lastdraw = Some(drawinfo);
                    }
                } else {
                    // 第一个对象：开始新批次
                    lastinsdata.reset();
                    lastinsdata.data.start = combinebuffer.usedsize();
                    _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                    lastdraw = Some(drawinfo);
                }
            });

            // 完成不透明对象的最后一个批次
            if let Some(tempdraw) = lastdraw.take() {
                collect_draw_batch(&mut combinebuffer, tempdraw, &lastinsdata, draws, &mut allocator, &device, &queue, &mut count_vertex);
                lastinsdata.reset();
                lastinsdata.data.start = combinebuffer.usedsize();
                lastdraw = None;
            }

            // 第十一阶段：透明对象批处理
            // 透明对象需要保持正确的渲染顺序，批处理策略略有不同
            transparent_list.drain(..).for_each(|drawinfo| {
                if let Some(tempdraw) = lastdraw.take() {
                    // 透明对象的批处理条件检查（通常更严格）
                    let batchcount_ok   = tempdraw.instancecount() + drawinfo.instancecount() < engineopt.max_instance_batch_count;
                    let batchmem_ok     = tempdraw.can_batch_instance_memory(&drawinfo, true);
                    let batchmaxsize_ok = combinebuffer.usedsize() + drawinfo.instancedatasize() < combinebuffer.maxcombinesize;
                    let batchcomb_ok    = combinebuffer.combinecommon(drawinfo.instancedatasize());

                    // log::error!("{:?}", (batchcount_ok, batchmem_ok, batchmaxsize_ok, batchcomb_ok, combinebuffer.usedsize(), drawinfo.instancedatasize(), combinebuffer.maxcombinesize));

                    if batchcount_ok && batchmem_ok && batchmaxsize_ok && batchcomb_ok {
                        // 可以合并：保持透明对象的正确渲染顺序
                        _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                        lastdraw = Some(tempdraw);
                    } else {
                        // 不能合并：完成当前批次，开始新批次
                        collect_draw_batch(&mut combinebuffer, tempdraw, &lastinsdata, draws, &mut allocator, &device, &queue, &mut count_vertex);
                        lastinsdata.reset();
                        lastinsdata.data.start = combinebuffer.usedsize();
                        _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                        lastdraw = Some(drawinfo);
                    }
                } else {
                    // 第一个透明对象：开始新批次
                    lastinsdata.reset();
                    lastinsdata.data.start = combinebuffer.usedsize();
                    _combine_instance( &mut combinebuffer, &mut lastinsdata, &drawinfo);
                    lastdraw = Some(drawinfo);
                }
            });

            // 完成透明对象的最后一个批次
            if let Some(tempdraw) = lastdraw.take() {
                collect_draw_batch(&mut combinebuffer, tempdraw, &lastinsdata, draws, &mut allocator, &device, &queue, &mut count_vertex);
                lastinsdata.reset();
                lastinsdata.data.start = combinebuffer.usedsize();
            }

            // if draws.list.len() > 0 {
            //     log::error!("Draws: {:?}", draws.list.len());
            // }
            // 第十二阶段：性能统计和收尾
            // 统计当前渲染器的绘制调用数量
            performance.drawcalls += draws.list.len() as u32;
        } else {
            // 视景体或场景数据获取失败的情况
        }

        // 记录当前渲染器处理的顶点数量
        renderer.vertexs = count_vertex;
    });

    // 第十三阶段：GPU缓冲区更新
    // 将所有合并的实例化数据上传到GPU
    combinebuffer.apply(&queue);

    // 性能监控：结束计时
    if performance.debug { performance.drawobjs = (pi_time::Instant::now() - performance.t_drawobjs).as_micros() as u32; }
}


/// 着色器创建和缓存函数
///
/// # 功能职责
/// 根据渲染配置创建或获取缓存的着色器程序：
/// - **着色器缓存**: 通过KeyShader3D实现智能缓存和复用
/// - **绑定组处理**: 处理场景、模型、材质值、纹理四个绑定组的着色器集成
/// - **代码生成**: 动态生成顶点和片段着色器的GLSL代码
/// - **资源管理**: 管理着色器资源的生命周期和依赖关系
///
/// # 着色器缓存机制
///
/// ## 缓存键生成
/// 基于以下因素生成唯一的着色器缓存键：
/// - **元数据键**: key_meta - 着色器效果元数据标识
/// - **绑定定义**: meta.binddefines - 绑定相关的预处理定义
/// - **顶点属性**: key_attributes - 顶点缓冲区布局信息
/// - **渲染对齐**: renderalignment - 渲染对齐方式
/// - **绑定组哈希**: 所有绑定组的哈希值组合
///
/// ## 缓存查找和创建
/// 1. 首先尝试从资源管理器获取已缓存的着色器
/// 2. 如果未找到，则创建新的着色器并加入缓存
/// 3. 返回着色器句柄供后续使用
///
/// # 着色器代码生成流程
///
/// ## 代码片段收集
/// 按顺序收集和处理各种代码片段：
/// - **vs_defined_snippets**: 顶点着色器定义代码
/// - **fs_defined_snippets**: 片段着色器定义代码
/// - **vs_extend_varying**: 顶点着色器扩展varying变量
/// - **fs_extend_varying**: 片段着色器扩展varying变量
/// - **运行时代码片段**: 模型、属性、效果相关的运行时代码
///
/// ## 绑定组处理顺序
/// 严格按照set0到set3的顺序处理绑定组：
/// - **set0**: 场景绑定组 - 全局场景数据
/// - **set1**: 模型绑定组 - 模型变换和蒙皮数据
/// - **set2**: 材质值绑定组 - 材质参数和uniform
/// - **set3**: 纹理绑定组 - 纹理采样器和绑定信息
///
/// # 性能优化
/// - **哈希缓存**: 避免重复创建相同配置的着色器
/// - **代码复用**: 通过模块化设计实现代码片段复用
/// - **延迟编译**: 只在需要时才进行着色器编译
fn shader(
    _id_pass: Entity,                                    // 渲染通道实体ID（用于调试）
    meta: &Handle<ShaderEffectMeta>,                     // 着色器效果元数据句柄
    key_meta: &Atom,                                     // 着色器元数据键
    vb: &VertexBufferLayoutsComp,                        // 顶点缓冲区布局组件
    bindgroups: &BindGroups3D,                           // GPU绑定组集合
    renderalignment: ERenderAlignmentForShader,          // 渲染对齐方式
    assets: & ShareAssetMgr<Shader3D>,                  // 着色器资源管理器
    device: &RenderDevice,                              // GPU设备引用
    engineopt: &EngineCustomPlugins,                    // 引擎自定义插件配置
) -> Result<Handle<Shader3D>, Shader3D> {
    // 获取顶点属性布局信息
    let key_attributes = &vb.1;

    // 分解绑定组为四个独立的绑定组引用
    let (set0, set1, set2, set3) = (&bindgroups.scene, &bindgroups.model, bindgroups.matvalues.as_ref(), bindgroups.textures.as_ref());

    // 生成绑定组的组合哈希值，用于着色器缓存键
    let mut hash = DefaultHasher::default();
    if let Some(set) = set0 { set.hash_for_shader(&mut hash); }  // 场景绑定组哈希
    if let Some(set) = set1 { set.hash_for_shader(&mut hash); }  // 模型绑定组哈希
    if let Some(set) = set2 { set.hash_for_shader(&mut hash); }  // 材质值绑定组哈希
    if let Some(set) = set3 { set.hash_for_shader(&mut hash); }  // 纹理绑定组哈希

    // 构建着色器缓存键
    let key_shader = KeyShader3D {
        key_meta: key_meta.clone(),                    // 着色器元数据键
        bind_defines: meta.binddefines,                // 绑定定义
        key_attributes: key_attributes.clone(),        // 顶点属性布局
        renderalignment: renderalignment,              // 渲染对齐方式
        bindgroups_for_shader: hash.finish(),          // 绑定组组合哈希
    };

    // 尝试从缓存获取已存在的着色器
    if let Some(shader) = assets.get(&key_shader) {
        Ok(shader)
    } else {
        let mut setidx = 0;
        let mut vs_defined_snippets = vec![];
        let mut fs_defined_snippets = vec![];
        let mut vs_extend_varying = String::from("");
        let mut fs_extend_varying = String::from("");
        let mut vs_running_model_snippets = vec![];
        let mut vs_running_attribute_snippets = vec![];
        let vs_running_after_effect_snippets = vec![];
        let vs_running_before_effect_snippets = vec![];
        let mut fs_running_before_effect_snippets = vec![];
        let fs_running_after_effect_snippets = vec![];
    
        // log::error!("Shader: {:?}", key_meta);
        // log::error!("{:?}", key_attributes);
        // log::error!("{:?}", key_attributes.vs_define_code());
    
        arr_push(&mut vs_defined_snippets, key_attributes.vs_define_code());
        vs_extend_varying += &key_attributes.vs_varying_code(meta.varyings.0.len() as u32, meta);
        fs_extend_varying += &key_attributes.fs_varying_code(meta.varyings.0.len() as u32, meta);
    
        // 处理set0：场景绑定组
        if let Some(set) = set0 {
            // 添加场景绑定组的定义代码到顶点和片段着色器
            arr_push(&mut vs_defined_snippets, set.vs_define_code(setidx));
            arr_push(&mut fs_defined_snippets, set.fs_define_code(setidx));
            setidx += 1;
        }

        // 处理set1：模型绑定组
        if let Some(set) = set1 {
            let skin = set.key().key.skin;  // 获取蒙皮信息
            arr_push(&mut vs_defined_snippets, set.vs_define_code(setidx));
            arr_push(&mut fs_defined_snippets, set.fs_define_code(setidx));

            // 添加模型相关的运行时代码片段
            arr_push(&mut vs_running_attribute_snippets, set.vs_running_model_snippet(meta));
            arr_push(&mut vs_running_model_snippets, skin.running_code());           // 蒙皮动画代码
            arr_push(&mut vs_running_model_snippets, renderalignment.running_code());  // 渲染对齐代码

            // 添加渲染对齐的定义代码
            arr_push(&mut vs_defined_snippets, renderalignment.define_code());

            setidx += 1;
        }

        // 添加顶点属性相关的运行时代码
        arr_push(&mut vs_running_attribute_snippets, key_attributes.vs_running_code());
        arr_push(&mut fs_running_before_effect_snippets, key_attributes.fs_running_code(meta));

        // 处理set2：材质值绑定组
        if let Some(set) = set2 {
            arr_push(&mut vs_defined_snippets, set.vs_define_code(setidx, meta, engineopt));
            arr_push(&mut fs_defined_snippets, set.fs_define_code(setidx, meta, engineopt));
            setidx += 1;
        }

        // 处理set3：纹理绑定组
        if let Some(set) = set3 {
            arr_push(&mut vs_defined_snippets, set.vs_define_code(setidx, meta, engineopt));
            arr_push(&mut fs_defined_snippets, set.fs_define_code(setidx, meta, engineopt));
            setidx += 1;
        }

        // 使用收集的所有代码片段构建最终的着色器
        let shader = meta.build_2(
            &device,
            &key_meta,
            &vs_defined_snippets,                              // 顶点着色器定义代码
            &vs_extend_varying,                                 // 顶点着色器扩展varying
            &fs_extend_varying,                                 // 片段着色器扩展varying
            &vs_running_attribute_snippets,                     // 顶点属性运行时代码
            &vs_running_model_snippets,                         // 顶点模型运行时代码
            &vs_running_before_effect_snippets, &vs_running_after_effect_snippets, // 顶点效果前/后处理代码
            &fs_defined_snippets,                              // 片段着色器定义代码
            &fs_running_before_effect_snippets, &fs_running_after_effect_snippets, // 片段效果前/后处理代码
            engineopt                                           // 引擎自定义选项
        );

        // 将新创建的着色器插入资源管理器缓存
        assets.insert(key_shader, shader)
    }
}

/// 渲染管线创建和缓存函数
///
/// # 功能职责
/// 根据渲染配置创建或获取缓存的GPU渲染管线：
/// - **管线缓存**: 通过KeyPipeline3D实现智能缓存和复用
/// - **状态配置**: 设置深度测试、模板测试、混合模式等渲染状态
/// - **布局管理**: 处理绑定组布局和顶点缓冲区布局
/// - **格式适配**: 支持不同的颜色和深度模板格式
///
/// # 管线缓存机制
///
/// ## 缓存键生成
/// 基于以下因素生成唯一的管线缓存键：
/// - **着色器**: shader.key() - 着色器程序标识
/// - **绑定组布局**: bindgroups.key_bindgroup_layouts() - GPU资源绑定布局
/// - **顶点布局**: vb.0 - 顶点缓冲区布局信息
/// - **渲染状态**: 深度、模板、混合、光栅化状态
/// - **目标格式**: 颜色和深度模板缓冲区格式
///
/// ## 缓存查找和创建
/// 1. 首先尝试从资源管理器获取已缓存的管线
/// 2. 如果未找到，则创建新的GPU管线并加入缓存
/// 3. 返回管线句柄供后续渲染使用
///
/// # 渲染状态配置
///
/// ## 深度模板状态
/// - **深度测试**: 基于depth_state配置深度比较函数和写掩码
/// - **模板测试**: 基于stencil_state配置模板操作
/// - **格式适配**: 自动适配不同的深度模板格式
///
/// ## 混合状态
/// - **颜色混合**: 基于ModelBlend配置源因子和目标因子
/// - **目标格式**: 支持多种颜色缓冲区格式
/// - **透明处理**: 正确处理透明物体的混合模式
///
/// ## 光栅化状态
/// - **图元拓扑**: 基于cull配置正面/背面剔除
/// - **多重采样**: 配置MSAA抗锯齿参数
/// - **多边形填充**: 控制多边形填充模式
///
/// # 性能优化
/// - **管线复用**: 相同配置的管线可以共享
/// - **状态预计算**: 提前计算和缓存管线状态
/// - **批量创建**: 支持批量创建多个相似管线
fn pipeline(
    shader: &Handle<Shader3D>,                          // 着色器程序句柄
    bindgroups: &BindGroups3D,                         // GPU绑定组集合
    vb: &VertexBufferLayoutsComp,                      // 顶点缓冲区布局组件
    colorformat: &ColorFormat,                         // 颜色缓冲区格式
    depthstencilformat: &DepthStencilFormat,            // 深度模板缓冲区格式
    blend: ModelBlend,                                 // 混合模式配置
    depth_state: &DepthState, stencil_state: &StencilState,  // 深度和模板状态
    cull: &PrimitiveState,                             // 图元状态（剔除等）
    _id_pass: Entity,                                  // 渲染通道实体ID（用于调试）
    assets: &ShareAssetMgr<Pipeline3D>,               // 管线资源管理器
    device: &RenderDevice,                            // GPU设备引用
) -> Result<Handle<Pipeline3D>, Pipeline3D> {
    // 获取着色器的键值，用于管线缓存
    let key_shader = shader.key().clone();

    // 获取绑定组布局信息
    let bind_group_layouts = bindgroups.bind_group_layouts();
    let key_bindgroup_layouts = KeyPipelineFromBindGroup(bindgroups.key_bindgroup_layouts());

    // 获取顶点缓冲区布局信息
    let key_vertex_layouts = vb.0.as_key_pipeline_from_vertex_layout();

    // 获取渲染目标格式
    let pass_color_format = colorformat.val();
    let pass_depth_format = depthstencilformat.val();

    // 配置深度模板状态
    let depth_stencil = if let Some(pass_depth_format) = pass_depth_format {
        Some(
            depth_stencil_state(
                pass_depth_format,    // 深度格式
                depth_state,          // 深度状态配置
                stencil_state         // 模板状态配置
            )
        )
    } else {
        None  // 无深度模板缓冲区
    };

    // 配置渲染目标状态（颜色混合等）
    let targets = RenderTargetState::color_target(pass_color_format, &blend);

    // 构建完整的渲染管线状态
    let key_state = KeyRenderPipelineState {
        primitive: cull.state(),                                    // 图元状态（剔除模式等）
        target_state: targets[0].clone(),                           // 渲染目标状态
        depth_stencil: depth_stencil,                               // 深度模板状态
        multisample: wgpu::MultisampleState {                      // 多重采样状态
            count: 1,                                               // 采样次数
            mask: !0,                                               // 采样掩码
            alpha_to_coverage_enabled: false                        // Alpha覆盖禁用
        }
    };

    // 构建最终的管线缓存键
    let key_pipeline = KeyPipeline3D {
        key_state,               // 渲染状态
        key_shader,              // 着色器键
        key_bindgroup_layouts,   // 绑定组布局
        key_vertex_layouts,      // 顶点布局
    };

    // 将管线键转换为64位哈希值用于缓存
    let key_u64 = key_pipeline.to_u64();

    // 尝试从缓存获取已存在的管线
    if let Some(pipeline) = assets.get(&key_u64) {
        Ok(pipeline)
    } else {
        // 缓存未命中，创建新的GPU渲染管线
        let pipeline = KeyPipeline3D::create(
            key_pipeline,           // 管线配置
            shader.clone(),          // 着色器程序
            bind_group_layouts,      // 绑定组布局
            &device                 // GPU设备
        );

        // 将新创建的管线插入资源管理器缓存
        assets.insert(key_u64, pipeline)
    }
}

/// 绘制对象收集和分类函数
///
/// # 功能职责
/// 将单个渲染对象转换为DrawTmpRef结构并根据透明度进行分类收集：
/// - **实例化对象处理**: 处理包含实例数据的几何体，支持透明度排序
/// - **普通对象处理**: 处理非实例化的单一几何体
/// - **透明度分类**: 根据对象类型分配到不透明或透明列表
/// - **距离计算**: 为透明对象计算精确的距离值
///
/// # 处理流程
///
/// ## 实例化对象处理路径
/// 当几何体包含实例数据时：
/// 1. 检查实例排序信息中的范围列表
/// 2. 为每个实例范围计算到摄像机的距离
/// 3. 创建DrawTmpRef结构，包含实例范围信息
/// 4. 透明对象使用alpha索引作为排序键
/// 5. 添加到对应的透明或不透明列表
///
/// ## 普通对象处理路径
/// 当几何体不包含实例数据时：
/// 1. 创建单一DrawTmpRef结构，实例范围设为[0,0]
/// 2. 使用传入的距离值进行排序
/// 3. 直接添加到对应的列表
///
/// # 透明度排序策略
/// - **不透明对象**: 使用原始的RenderQueueSortParam进行排序
/// - **透明对象**: 使用alpha索引重排sort_param.index，确保透明度正确混合
///
/// # 性能优化
/// - 使用范围检查避免无效实例数据处理
/// - 通过arr_push宏优化向量操作
/// - 复用传入的sort_param减少克隆开销
fn collect_draw<'w>(
    is_transparent: bool,                                          // 是否为透明对象
    pass: u8,                                                     // 渲染通道索引
    rendergeo: &'w RenderGeometry,                                // 几何体数据引用
    bindgroups: &'w BindGroups3D,                                 // GPU绑定组引用
    pipeline: &'w Pipeline3DUsage,                                // 渲染管线引用
    indicerange: &'w IndiceRenderRange,                           // 索引渲染范围
    vertexrange: &'w VertexRenderRange,                           // 顶点渲染范围
    distance: f32,                                                // 对象到摄像机的距离
    sort_param: &'w RenderQueueSortParam,                         // 渲染队列排序参数
    opaque_list: & mut Vec<DrawTmpRef<'w>>,                      // 不透明对象列表（输出）
    transparent_list: & mut Vec<DrawTmpRef<'w>>,                  // 透明对象列表（输出）
    instancessortinfo: &'w InstancedMeshTransparentSortCollection, // 实例化排序信息
    distancecomp: &ViewerDistanceCompute,                         // 距离计算器
    viewposition: &(Number, Number, Number),                      // 摄像机位置
    viewdirection: &(Number, Number, Number),                      // 摄像机方向
) {
    // 判断几何体是否包含实例化数据
    if rendergeo.instance_slot.is_some() {
        // 实例化对象处理路径
        if instancessortinfo.ranges.len() > 0 {
            // 遍历所有实例范围（按透明度分组）
            instancessortinfo.ranges.iter().for_each(|(alphaindex, range, center)| {
                // 重新计算该实例组的中心点到摄像机的距离
                let distance = distancecomp.distance(&viewposition, &viewdirection, center);

                // 验证实例范围的有效性
                if range.start < range.end && range.end <= instancessortinfo.count as u32 {
                    // 创建DrawTmpRef结构，包含所有渲染所需信息
                    let mut draw = DrawTmpRef {
                        rendergeo,                                                 // 几何体引用
                        pipeline,                                                  // 渲染管线引用
                        bindgroups,                                                // GPU绑定组引用
                        indicerange,                                               // 索引范围
                        vertexrange,                                               // 顶点范围
                        inscombinerange: range.clone(),                            // 当前处理的实例范围
                        instancessortinfo,                                         // 实例化排序信息引用
                        pass,                                                      // 渲染通道索引
                        distance,                                                  // 计算得到的距离
                        queue: sort_param.clone(),                                 // 复制排序参数
                    };

                    // 根据透明度类型分配到对应列表
                    if is_transparent == false {
                        // 不透明对象：直接添加到不透明列表
                        arr_push(opaque_list, draw);
                    } else {
                        // 透明对象：使用alpha索引作为排序键，确保正确混合顺序
                        draw.queue.index = *alphaindex;
                        arr_push(transparent_list, draw);
                    }
                } else {
                    // 实例范围无效的情况（通常不会发生）
                }
            });
        } else {
            // 实例排序信息为空的情况（注释掉的旧代码保留用于参考）
        }
    } else {
        // 普通对象处理路径（非实例化）
        let draw = DrawTmpRef {
            rendergeo,                                                 // 几何体引用
            pipeline,                                                  // 渲染管线引用
            bindgroups,                                                // GPU绑定组引用
            indicerange,                                               // 索引范围
            vertexrange,                                               // 顶点范围
            inscombinerange: Range { start: 0, end: 0 },            // 非实例化对象使用空范围
            instancessortinfo,                                         // 实例化排序信息引用（空）
            pass,                                                      // 渲染通道索引
            distance,                                                  // 使用传入的距离值
            queue: sort_param.clone(),                                 // 复制排序参数
        };

        // 根据透明度类型分配到对应列表
        if is_transparent == false {
            arr_push(opaque_list, draw);
        } else {
            arr_push(transparent_list, draw);
        }
    }
}

/// 实例化数据合并函数
///
/// # 功能职责
/// 将新的实例化数据合并到合并缓冲区中，实现批处理优化：
/// - **数据范围计算**: 根据实例范围计算数据在原始缓冲区中的字节位置
/// - **数据复制**: 将实例数据复制到合并缓冲区的连续内存中
/// - **状态更新**: 更新合并状态信息和实例计数
/// - **内存管理**: 高效处理不同大小的实例数据结构
///
/// # 合并机制
///
/// ## 数据范围计算
/// 根据实例化信息的元数据计算精确的数据位置：
/// - 单个实例数据大小: 总数据长度 ÷ 实例总数
/// - 起始位置: 实例起始索引 × 单个实例数据大小
/// - 结束位置: 实例结束索引 × 单个实例数据大小
///
/// ## 内存复制策略
/// - 使用切片引用避免数据拷贝
/// - 直接复制到合并缓冲区的连续内存区域
/// - 保持数据在GPU上的对齐要求
///
/// # 状态同步
/// 合并完成后更新目标状态：
/// - **data.end**: 更新合并缓冲区的已使用大小
/// - **itemcount**: 累加实例数量
/// - **slot**: 记录实例化数据插槽索引
///
/// # 性能优化
/// - 范围检查避免处理无效实例数据
/// - 连续内存复制提高GPU访问效率
/// - 累计计数减少后续计算开销
fn _combine_instance(
    combinedata: & mut CombineBuffer,                    // 合并缓冲区（存储所有批处理的实例数据）
    lastinsdata: &mut EVerteicesInstance,                // 上一次合并的状态信息（更新目标）
    drawinfo: &DrawTmpRef                                 // 当前要合并的绘制对象信息
) {
    // 检查几何体是否包含实例化数据插槽
    if let Some(slot) = &drawinfo.rendergeo.instance_slot {
        // 验证实例化数据的有效性
        if drawinfo.instancessortinfo.count > 0 && drawinfo.inscombinerange.start < drawinfo.inscombinerange.end {
            // 计算单个实例的数据大小（字节）
            let size = drawinfo.instancessortinfo.data.len() / drawinfo.instancessortinfo.count as usize;

            // 计算当前实例范围在原始数据中的字节位置
            let start = drawinfo.inscombinerange.start as usize * size;  // 起始字节位置
            let end = drawinfo.inscombinerange.end as usize * size;      // 结束字节位置

            // 将实例数据复制到合并缓冲区中
            combinedata.record(&drawinfo.instancessortinfo.data.as_slice()[start..end]);

            // 更新合并状态信息
            lastinsdata.data.end = combinedata.usedsize();                                   // 更新已使用的缓冲区大小
            lastinsdata.itemcount += drawinfo.inscombinerange.end - drawinfo.inscombinerange.start; // 累加实例数量
            lastinsdata.slot = *slot as u8;                                                   // 设置实例化数据插槽
        }
    }
}

/// 批处理绘制对象生成函数
///
/// # 功能职责
/// 将临时绘制引用和合并的实例数据转换为最终的GPU绘制对象：
/// - **实例化对象处理**: 创建包含合并实例数据的GPU绘制对象
/// - **普通对象处理**: 创建单一几何体的GPU绘制对象
/// - **GPU缓冲区管理**: 分配和管理GPU顶点和实例缓冲区
/// - **内存对齐处理**: 确保数据符合GPU内存对齐要求
/// - **性能统计**: 更新顶点计数和绘制调用统计
///
/// # 处理流程
///
/// ## 实例化对象处理路径
/// 当对象包含实例数据时：
/// 1. **数据验证**: 检查实例数据的有效性
/// 2. **内存对齐**: 按照wgpu要求对齐数据大小
/// 3. **缓冲区填充**: 如需要，填充数据到对齐边界
/// 4. **GPU缓冲区创建**: 创建可更新的GPU缓冲区
/// 5. **绘制对象构建**: 包含所有渲染状态的DrawObj
/// 6. **统计更新**: 计算并累加顶点数量
///
/// ## 普通对象处理路径
/// 当对象不包含实例数据时：
/// 1. 直接使用几何体的原始实例信息
/// 2. 构建简化的DrawObj结构
/// 3. 进行顶点计数和统计更新
///
/// # GPU内存管理
///
/// ## 内存对齐策略
/// - 使用wgpu::COPY_BUFFER_ALIGNMENT确保GPU兼容性
/// - 必要时填充零数据到对齐边界
/// - 计算精确的数据范围用于GPU缓冲区创建
///
/// ## 缓冲区类型
/// - **实例化对象**: 使用EVBRange可更新缓冲区
/// - **普通对象**: 使用几何体原始缓冲区引用
///
/// # 性能优化
/// - 早期返回避免无效数据处理
/// - 批量顶点计数减少重复计算
/// - 使用Arc共享DrawObj减少内存占用
fn collect_draw_batch(
    combinebuffer: &mut CombineBuffer,            // 合并缓冲区（包含所有批处理的实例数据）
    tempdraw: DrawTmpRef,                         // 临时绘制引用（包含所有渲染信息）
    instancedata: &EVerteicesInstance,             // 合并后的实例化数据状态
    draws: &mut DrawList3D,                       // 绘制列表（输出，添加最终的DrawObj）
    allocator: &mut VertexBufferAllocator3D,       // 顶点缓冲区分配器
    device: &PiRenderDevice,                      // GPU设备引用
    queue: &PiRenderQueue,                        // GPU命令队列
    count_vertex: &mut usize,                     // 顶点计数器（输出统计）
) {
    let geo = tempdraw.rendergeo;

    // 判断是否为实例化对象
    if tempdraw.rendergeo.instance_slot.is_some() {
        // 实例化对象处理路径
        let mem: &EVerteicesInstance = instancedata;

        // 验证实例数据的有效性
        if mem.itemcount == 0 {
            // 实例数量为0，跳过处理
            return;
        } else if mem.data.end <= mem.data.start  {
            // 数据范围为空，跳过处理
            return;
        };

        // 计算每个实例的数据大小（字节）
        let size_per_value = (mem.data.end - mem.data.start) as u32 / mem.itemcount;
        let instances = Range { start: 0, end: mem.itemcount, };

        // 计算GPU内存对齐后的数据范围
        let range = {
            let size = wgpu::COPY_BUFFER_ALIGNMENT as usize;  // GPU内存对齐大小（通常是4字节）
            let temp = (mem.data.end / size) * size;          // 对齐到下一个边界
            let start = mem.data.start;
            let mut end = mem.data.end;

            // 如果需要对齐，填充数据到对齐边界
            if temp < mem.data.end {
                end = temp + size;  // 扩展到对齐边界
                let hascount = combinebuffer.data.len();

                // 如果缓冲区不够大，填充零数据
                if hascount < end {
                    let placehold: [u8;4] = [0, 0, 0, 0];  // 零填充数据
                    let count = end - hascount;
                    combinebuffer.record(&placehold[0..count]);
                }
            }
            Range { start, end }
        };

        // 从合并缓冲区创建GPU缓冲区
        let data = combinebuffer.data(&range, allocator, device, queue);

        if let Some(data) = data {
            // 创建实例化绘制对象
            let mut draw = DrawObj {
                pipeline: Some(tempdraw.pipeline.clone()),           // 渲染管线
                bindgroups: tempdraw.bindgroups.groups(),            // GPU绑定组
                vertices: tempdraw.rendergeo.vertices(),             // 几何体顶点缓冲区
                instances,                                           // 实例范围
                vertex: tempdraw.vertexrange.apply(geo),              // 顶点渲染范围
                indices: tempdraw.indicerange.apply(geo),            // 索引渲染范围
            };

            // 插入实例化顶点数据
            draw.insert_vertices(RenderVertices {
                slot: mem.slot as u32,                                             // 实例化数据插槽
                buffer: EVerticesBufferUsage::EVBRange(Share::new(data)),          // GPU缓冲区引用
                buffer_range: None,                                                // 缓冲区范围（由EVBRange管理）
                size_per_value: size_per_value as u64,                             // 每个值的大小
            });

            // 设置实例范围
            draw.instances = Range { start: 0, end: mem.itemcount };

            // 计算实际顶点数量（考虑索引）
            let vertex = if let Some(indices) = &draw.indices {
                indices.value_range().end - indices.value_range().start
            } else {
                draw.vertex.end - draw.vertex.start
            };

            // 顶点数量为0则跳过
            if vertex == 0 {
                return;
            }

            // 更新顶点统计（顶点数 × 实例数）
            *count_vertex += (vertex * (draw.instances.end - draw.instances.start)) as usize;

            // 添加到绘制列表
            arr_push(&mut draws.list, Arc::new(draw));
        } else {
            // GPU缓冲区创建失败的情况（注释保留用于调试）
        };
    } else {
        // 普通对象处理路径（非实例化）
        let instances = geo.instances();                                    // 获取几何体的原始实例信息
        let vertex = tempdraw.vertexrange.apply(geo);                       // 应用顶点范围
        let indices = tempdraw.indicerange.apply(geo);                      // 应用索引范围

        // 计算顶点数量
        let vertexcount = if let Some(indices) = &indices {
            indices.value_range().end - indices.value_range().start
        } else {
            vertex.end - vertex.start
        };

        // 验证数据有效性
        if vertexcount == 0 || instances.start >= instances.end {
            return;
        }

        // 创建普通绘制对象
        let draw = DrawObj {
            pipeline: Some(tempdraw.pipeline.clone()),
            bindgroups: tempdraw.bindgroups.groups(),
            vertices: tempdraw.rendergeo.vertices(),
            instances,
            vertex,
            indices,
        };

        // 更新顶点统计
        *count_vertex += (vertexcount * (draw.instances.end - draw.instances.start)) as usize;

        // 添加到绘制列表
        arr_push(&mut draws.list, Arc::new(draw));
    }
}