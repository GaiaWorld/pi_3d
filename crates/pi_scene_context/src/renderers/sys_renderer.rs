use std::{hash::Hasher, ops::Range, sync::Arc};

use pi_scene_shell::{prelude::*, run_stage::EngineCustomPlugins};
use crate::{
    bindgroup::*, flags::*, geometry::{instance::instanced_buffer::*, prelude::*}, materials::prelude::*, meshes::prelude::*, object::{TmpSortDrawOpaqueVec, TmpSortDrawTransparentVec}, pass::*, scene::prelude::*, skeleton::prelude::*, transforms::prelude::*, viewer::prelude::*
};

use super::{
    _set0_modify, _set1_modify, base::*, render_depth_and_stencil::*, render_object::RenderState, render_primitive::*, render_sort::*, render_target_state::*, renderer::*
};
    pub fn sys_pass_bind_groups(
        addeds: ComponentAdded<PassBindGroupsDirty>,
        changes: ComponentChanged<PassBindGroupsDirty>,
        mut passes: Query<
            (ObjectID, &PassModelID, &PassMaterialID, &PassRendererID, &mut PassBindGroups, &mut PassFlagShader, &PassTag)
        >,
        renderers: Query<(&SceneID, &ViewerID)>,
        materials: Query<( &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &MaterialRefs, &EffectTextureSamplersComp, &TextureKeyList )>,
        models: Query<( Option<&BindModel>, &BindModelMatIdx, &BindSkinValue, &SkeletonID, &ModelLightingIndexs )>,
        targets: Res<CustomRenderTargets>,
        viewers: Query<&BindViewer>,
        scenes: Query<(&BindSceneEffect, &SceneLightingInfos, &BRDFTexture, &BRDFSampler, &MainCameraOpaqueTarget, &MainCameraDepthTarget, &SceneShadowRenderTarget, Option<&SceneShadowInfos>, &EnvTexture, &EnvIrradiance, &EnvSampler)>,
        device: Res<PiRenderDevice>,
        asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
        asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
        bindpassindexs: Res<BindPassIndexPool>,
        mut errors: ResMut<ErrorRecord>,
        entitysets: Res<EntityFilterForComponentChanged>,
        // mut performance: ResMut<Performance>,
    ) {
        // performance.systems.push(String::from("sys_pass_bind_groups"));
        let mut entities = entitysets.pop();
        changes.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        addeds.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        entities.iter().for_each(|entity| {
            if let Ok((_id_pass, idmodel, idmat, idrenderer, mut bindgroups, mut flag, passidx)) = passes.get_mut(*entity) {
                let (idscene, idviewer) = if let Ok((idscene, idviewer)) = renderers.get(idrenderer.0) {
                    (idscene.0, idviewer.0)
                } else {
                    // log::error!("Bindgroups viewer Fail");
                    return;
                };
                let bind_passindex = if let Some(bindpassindex) = bindpassindexs.get(passidx.index()) {
                    bindpassindex
                } else { 
                    // log::error!("Bindgroups bind_passindex Fail");
                    return;
                };

                // log::error!("Bindgroups {:?}", (idrenderer.0));
                let idmodel = idmodel.0;
                let scenes = &scenes;
                let device = &device;
                let asset_mgr_bindgroup_layout = &asset_mgr_bindgroup_layout;
                let asset_mgr_bindgroup = &asset_mgr_bindgroup;
                let targets = &targets;
                let errors = &mut errors;
                let viewers = &viewers;
                let models = &models;
    
                if let Ok((effect_key, meta, bind, _list, textures, texkeys)) = materials.get(idmat.0) {
                    let (bindvalue, bindtextures, effect) = _pass_effect_ready(
                        effect_key, textures, texkeys, meta, bind
                    );
    
                    if let Some((key_meta, meta)) = &effect {
                        let need_set0 = true;
                        let need_set1 = BindDefines::need_bind_group_set1(meta.binddefines);
                        let need_set2 = bind.0.is_some();
                        let need_set3 = meta.textures.len() > 0;
                        let matidx = if let Some(temp) = &bind.0 {
                            temp.bind.matidx()
                        } else { 0 };

                        let set0 = { 
                            let temp = _set0_modify(
                                idmodel, idscene, idviewer, meta,
                                viewers, scenes, device,
                                asset_mgr_bindgroup_layout, asset_mgr_bindgroup, targets, 
                                bind_passindex,
                                errors
                            );
                            if temp.is_none() {
                                if bindgroups.val().is_some() {
                                    *bindgroups = PassBindGroups::new(None);
                                    *flag = PassFlagShader;
                                }
                                // log::error!("Bindgroups Fail set0");
                                return;
                            }
                            temp
                        };

                        let set1 = if need_set1 {
                            let temp = _set1_modify(
                                idmodel, key_meta, meta,
                                models, device, asset_mgr_bindgroup_layout, asset_mgr_bindgroup,
                                passidx.index(), matidx
                            );
                            if temp.is_none() {
                                if bindgroups.val().is_some() {
                                    *bindgroups = PassBindGroups::new(None);
                                    *flag = PassFlagShader;
                                }
                                // log::error!("Bindgroups Fail set1");
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

                        let data = BindGroups3D::create(set0, set1, set2, set3);
                        *bindgroups = PassBindGroups::new(Some(data));
                        *flag = PassFlagShader;
                    } else {
                        // log::error!("Bindgroups Fail effect");
                    }
                } else {
                    // log::error!("Bindgroups Fail materials");
                    if bindgroups.val().is_some() {
                        *bindgroups = PassBindGroups::new(None);
                        *flag = PassFlagShader;
                    }
                }
            } else {
                // log::error!("Bindgroups Fail Pass");
            }
        });
        entitysets.push(entities);
    }

/// 渲染器搜集渲染
    pub fn sys_pass_shader_request_by_model(
        addeds0: ComponentAdded<GeometryID>,
        changes0: ComponentChanged<GeometryID>,
        addeds1: ComponentAdded<RenderAlignment>,
        changes1: ComponentChanged<RenderAlignment>,
        models: Query<
            (
                &GeometryID, &PassIDs
            )
        >,
        geoaddeds: ComponentAdded<VertexBufferLayoutsComp>,
        geochanges: ComponentChanged<VertexBufferLayoutsComp>,
        geometrys: Query<(Entity, &MeshID)>,
        mut passes: Query<(&mut PassGeometryID, &mut PassPipelineStateDirty, &mut PassFlagShader)>,
        entitysets: Res<EntityFilterForComponentChanged>,
    ) {
        // let time1 = pi_time::Instant::now();
        let mut entities = entitysets.pop();
        addeds0.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        changes0.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        addeds1.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        changes1.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        entities.iter().for_each(|entity| {
            if let Ok((id_geo, passids)) = models.get(*entity) {
                // log::error!("sys_pass_shader_request_by_model");
                passids.0.iter().for_each(|id| {
                    if let Ok((mut idgeometry, mut flagpipeline, mut flagshader)) = passes.get_mut(*id) {
                        *idgeometry = PassGeometryID(id_geo.0);
                        *flagpipeline = PassPipelineStateDirty;
                        *flagshader = PassFlagShader;
                    }
                });
            }
        });

        entities.clear();
        geoaddeds.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        geochanges.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        entities.iter().for_each(|entity| {
            // log::error!("sys_pass_shader_request_by_geometry");
            if let Ok((entity, idmesh)) = geometrys.get(*entity) {
                if let Ok((id_geo, passids)) = models.get(idmesh.0) {
                    if entity == id_geo.0 {
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
        // log::debug!("SysPassShaderRequestByModel: {:?}", pi_time::Instant::now() - time1);
    }

/// 渲染器搜集渲染
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
        changes.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        addeds.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        entities.iter().for_each(|entity| {
            if let Ok((id_pass, disposeready, id_model, id_geo, idmat, bindgroups, mut old_shader, mut flagpipeline)) = passes.get_mut(*entity) {

                if disposeready.0 == true { return; }
            
                if let Ok((effect_key, meta)) = materials.get(idmat.0) {
                    let key_meta = &effect_key.0;
                    let meta = &meta.0;
                    // log::debug!("SysPassShaderRequestByPass: 0");
                    if let (Some(meta), Some(bindgroups)) = (meta, bindgroups.val()) {
                        // log::debug!("SysPassShaderRequestByPass: 1");
                        match (models.get(id_model.0), geometrys.get(id_geo.0)) {
                            (Ok(renderalignment), Ok(vb)) => {
        
                                let limit = device.0.limits();
                                if vb.0.attrcount as u32 <= limit.max_vertex_attributes && vb.0.desccount as u32 <= limit.max_vertex_buffers {
                                    let renderalignment = renderalignment.shader_tag(false);
                                    if let Ok(shader) = shader(
                                        id_pass, meta, key_meta, vb, bindgroups, renderalignment, &assets, &device, &engineopt
                                    ) {
                                        // log::error!("Shader Success");
                            
                                        if let Some(old) = &old_shader.0 {
                                            if old.key() != shader.key() {
                                                *old_shader = PassShader(Some(shader));
                                                *flagpipeline = PassPipelineStateDirty;
                                            }
                                        } else {
                                            *old_shader = PassShader(Some(shader));
                                            *flagpipeline = PassPipelineStateDirty;
                                        }
                                    } else {
                                        // log::error!("Shader Fail");
                                        if old_shader.0.is_some() {
                                            *old_shader = PassShader(None);
                                            *flagpipeline = PassPipelineStateDirty;
                                        }
                                    }
                                } else {
                                    // log::error!("MAX_ATTRIBUTES: {}, Using Attributes: {}, MAX_BUFFER: {}, Using Buffers: {}", limit.max_vertex_attributes, vb.0.attrcount, limit.max_vertex_buffers, vb.0.desccount);
                                    *old_shader = PassShader(None);
                                    *flagpipeline = PassPipelineStateDirty;
                                }
                            },
                            _ => {
                                // log::error!("Shader Fail Geometry");
                                if old_shader.0.is_some() {
                                    *old_shader = PassShader(None);
                                    *flagpipeline = PassPipelineStateDirty;
                                }
                            }
                        };
                    }
                }
            }
        });
        entitysets.push(entities);

        // log::debug!("SysPassShaderRequestByPass: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_request_by_renderer(
        passaddeds: ComponentAdded<PassRendererID>,
        passaddeds2: ComponentAdded<RenderState>,
        passchanges: ComponentChanged<PassRendererID>,
        passchanges2: ComponentChanged<RenderState>,
        changes0: ComponentAdded<FlagRendererParamForPipeline>,
        changes: ComponentChanged<FlagRendererParamForPipeline>,
        renderers: Query<(&RendererParam, &ViewerID, &PassTag)>,
        viewers: Query<(&ModelList, &ForceIncludeModelList)>,
        modelspass: Query<&PassIDs>,
        mut passes: Query<&mut PassPipelineStateDirty>,
        entitysets: Res<EntityFilterForComponentChanged>,
    ) {
        let mut entities = entitysets.pop();
        passaddeds.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        passaddeds2.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        passchanges.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        passchanges2.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        entities.iter().for_each(|entity| {
            // log::error!("sys_pass_pipeline_request_by_model");
            if let Ok(mut flag) = passes.get_mut(*entity) {
                *flag = PassPipelineStateDirty;
            }
        });

        entities.clear();
        changes.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        changes0.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        // let changes = changes0.iter().chain(changes.iter());
        // let time1 = pi_time::Instant::now();
        entities.iter().for_each(|entity| {
            // log::error!("sys_pass_pipeline_request_by_renderer");
            if let Ok((param, idviewer, passtag)) = renderers.get(*entity) {
                if param.enable.0 {
                    if let Ok((modellist, forcemodels)) = viewers.get(idviewer.0) {
                        modellist.0.iter().for_each(|idmodel| {
                            if let Ok(passid) = modelspass.get(*idmodel) { _pass_pipeline_request_by_renderer(passid.0[passtag.index()], &mut passes); }
                        });
                        forcemodels.0.iter().for_each(|idmodel| {
                            if let Ok(passid) = modelspass.get(*idmodel) { _pass_pipeline_request_by_renderer(passid.0[passtag.index()], &mut passes); }
                        });
                    }
                }
            }
        });
        entitysets.push(entities);
    }

    fn _pass_pipeline_request_by_renderer(
        passid: Entity,
        passes: &mut Query<&mut PassPipelineStateDirty>,
    ) {
        if let Ok( mut flag ) = passes.get_mut(passid) { *flag = PassPipelineStateDirty; }
    }

    pub fn sys_pass_pipeline(
        addeds: ComponentAdded<PassPipelineStateDirty>,
        changes: ComponentChanged<PassPipelineStateDirty>,
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
        mut errors: ResMut<ErrorRecord>,
        entitysets: Res<EntityFilterForComponentChanged>,
    ) {
        // let time1 = pi_time::Instant::now();
        let mut entities = entitysets.pop();
        changes.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        addeds.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        entities.iter().for_each(|entity| {
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
                                } else {
                                    errors.record(id_model.0, ErrorRecord::ERROR_PASS_PIPELINE_FAIL);
                                    if oldpipeline.0.is_some() {
                                        *oldpipeline = PassPipeline(None);
                                        *flag = PassDrawDirty;
                                    }
                                }
                            },
                            _ => { 
                                if oldpipeline.0.is_some() {
                                    *oldpipeline = PassPipeline(None);
                                    *flag = PassDrawDirty;
                                }
                            }
                        }
                    } else {
                        if oldpipeline.0.is_some() {
                            *oldpipeline = PassPipeline(None);
                            *flag = PassDrawDirty;
                        }
                    }
                } else {
                    if oldpipeline.0.is_some() {
                        *oldpipeline = PassPipeline(None);
                        *flag = PassDrawDirty;
                    }
                }
            }
        });
        entitysets.push(entities);
        // // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }


    pub fn sys_pass_draw_modify_by_model(
        models: Query<&PassIDs>,
        changes0: ComponentChanged<RenderGeometryEable>,
        changes1: ComponentChanged<IndiceRenderRange>,
        changes2: ComponentChanged<VertexRenderRange>,
        mut passes: Query<&mut PassDrawDirty>,
        entitysets: Res<EntityFilterForComponentChanged>,
    ) {
        let mut entities = entitysets.pop();
        changes0.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        changes1.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        changes2.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        entities.iter().for_each(|entity| {
            if let Ok(passids) = models.get(*entity) {
                passids.0.iter().for_each(|id| {
                    if let Ok(mut drawdirty) = passes.get_mut(*id) { *drawdirty = PassDrawDirty; }
                });
            }
        });
        entitysets.push(entities);

        // // log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }
    
    pub fn sys_pass_draw_modify_by_pass(
        models: Query<(&GeometryID, &RenderGeometryEable, &DisposeReady)>,
        geometrys: Query<&RenderGeometryComp>,
        changes: ComponentChanged<PassDrawDirty>,
        mut passes: Query<(&PassModelID, &PassBindGroups, &PassPipeline, &mut PassDraw)>,
        // mut commands: Commands,
        entitysets: Res<EntityFilterForComponentChanged>,
    ) {
        let mut entities = entitysets.pop();
        changes.iter().for_each(|entity| {
            entities.insert(*entity);
        });
        entities.iter().for_each(|entity| {
            if let Ok((id_model, bindgroups, pipeline, mut old_draw)) = passes.get_mut(*entity) {
                if let (Some(_), Some(_)) = (bindgroups.val(), pipeline.val()) {
                    if let Ok((id_geo, geoenable, disposed)) = models.get(id_model.0) {
                        if geoenable.0 == false || disposed.0 == true {
                            if old_draw.val() {
                                *old_draw = PassDraw(false);
                                // log::error!("PassDraw Disabled {:?}", (geoenable.0, disposed.0));
                            };
                            return;
                        }
    
                        if let Ok(RenderGeometryComp(Some(rendergeo))) = geometrys.get(id_geo.0.clone()) {
                            if rendergeo.isok() {
                                // log::error!("PassDraw Ok");
                                *old_draw = PassDraw(true);
                            } else {
                                // log::error!("PassDraw Geo Not Ok");
                                *old_draw = PassDraw(false);
                            }
                        } else {
                            *old_draw = PassDraw(false);
                        }
                    }
                } else {
                    *old_draw = PassDraw(false);
                }
            }
        });
        entitysets.push(entities);

        // // log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }

    /// 遍历 Renderer , 对所属 Viewer 的 ModelListAfterCulling 进行遍历, 获取 与 Renderer 相关 Pass 关联的 物体
    /// 对收集到的物体进行排序、渲染合并
    pub fn sys_renderer_draws_modify(
        mut renderers: Query< ( ObjectID, &SceneID, &ViewerID, &mut Renderer, &PassTag, &RendererParam ) >,
        viewers: Query< (&ModelListAfterCulling, &ViewerGlobalPosition, &ViewerDirection, &DisposeReady, &ViewerDistanceCompute), >,
        scenes: Query< (&BatchParamOpaque, &BatchParamTransparent) >,
        models: Query<
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
    ) {
        // performance.systems.push(String::from("sys_renderer_draws_modify"));
        if performance.debug { performance.t_drawobjs = pi_time::Instant::now(); }

        let mut opaque_list: Vec<DrawTmpRef> = Vec::with_capacity(4096);
        let mut transparent_list: Vec<DrawTmpRef> = Vec::with_capacity(4096);

        let mut lastinsdata: EVerteicesInstance = EVerteicesInstance::default();
        let mut lastdraw: Option<DrawTmpRef> = None;

        performance.drawcalls = 0;
        renderers.iter_mut().for_each(|(_id_renderer, idscene, id_viewer, mut renderer, passtag, param)| {
            renderer.clear();
            // log::warn!("Renderer: {:?}, Camera {:?}, {:?}", _id_renderer, id_viewer.0, (param.enable.0, passtag));
            if param.enable.0 == false {
                // log::warn!("Renderer Disable: {:?}, Camera {:?}, {:?}", _id_renderer, id_viewer.0, (param.enable.0, passtag));
                return;
            }

            let mut count_vertex = 0;
            let mut countmesh = 0;
            opaque_list.clear();
            transparent_list.clear();
            if let (Ok((list_model, viewposition, viewdirection, disposed, distancecomp)), Ok((batchopaque, batchtransparent))) = (viewers.get(id_viewer.0), scenes.get(idscene.0)) {
                if disposed.0 {
                    // log::warn!("Renderer Viewer disposed: {:?}, Camera {:?}, {:?}", _id_renderer, id_viewer.0, (param.enable.0, passtag));
                    return;
                }
                countmesh = 0;
                renderer.draws.viewport = param.viewport.val();
                let viewposition = (viewposition.0.x, viewposition.0.y, viewposition.0.z );
                let viewdirection = (viewdirection.0.x, viewdirection.0.y, viewdirection.0.z );
                for id_obj in list_model.0.iter() {
                    if let Ok(
                        (
                            globalenable, nodeposition, rendersort, instancessortinfo,
                            passids, idgeometry, indicerange, vertexrenage, geoenable
                        )
                    ) = models.get(id_obj.clone()) {
                        let passids = &passids.0;
                        // log::warn!("Renderer: A {:?}", (disposed.0, globalenable.0));
                        if geoenable.0 && globalenable.0 == true && passtag.index() < passids.len() {
                            let idpass = passids[passtag.index()];
                            if let Ok(RenderGeometryComp(Some(rendergeo))) = geometrys.get(idgeometry.0) {
                                let index = 0;
                                let is_transparent = param.blend.0;
                                // log::error!("is_transparent {:?}", (is_transparent));
                                if let Ok((passrendererid, bindgroups, pipeline)) = passes.get(idpass) {
                                    if let (true, Some(bindgroups), Some(pipeline)) = (passrendererid.0 == _id_renderer, bindgroups.val(), pipeline.val()) {
                                        let mut distance = 0.;
                                        if is_transparent {
                                            if batchtransparent.0.distance {
                                                distance = distancecomp.distance(&viewposition, &viewdirection, &nodeposition.xyz());
                                            }
                                        } else {
                                            if batchopaque.0.distance {
                                                distance = distancecomp.distance(&viewposition, &viewdirection, &nodeposition.xyz());
                                            }
                                        }

                                        collect_draw(
                                            is_transparent, index, rendergeo, bindgroups, pipeline, indicerange, vertexrenage, distance,
                                            rendersort, &mut opaque_list, &mut transparent_list, &instancessortinfo, distancecomp, &viewposition, &viewdirection
                                        );
                                    } else {
                                        // log::error!("PassDraw Renderer Error {:?}", (passtag));
                                    }
                                } else {
                                //     log::error!("PassDraw Error {:?}", (passtag));
                                }
                                countmesh += 1;
                                // log::warn!("OK {:?}", id_obj);
                            } else {
                                // log::error!("Fail rendergeo {:?}", (idgeometry.0, id_obj));
                            }
                        } else {
                            // log::error!("Fail rendergeo {:?}", (geoenable.0, globalenable.0));
                            
                        }
                    } else {
                        // log::warn!("models.get Fail {:?}", id_obj);
                    }
                }

                opaque_list.sort_by(|a, b| DrawTmpRef::cmp_opaque(a, b));
                transparent_list.sort_by(|a, b| DrawTmpRef::cmp_transparent(a, b));

                // return;
                // log::error!("Mesh: {:?}", countmesh);
                // log::warn!("Opaque: {:?}", opaque_list.len());
                // log::error!("Transparent: {:?}", transparent_list.len());
                lastinsdata.reset();
                lastdraw = None;
                opaque_list.drain(..).for_each(|drawinfo| {
                    // log::warn!("{:?}", tmp);
                    if let Some(tempdraw) = lastdraw.take() {
                        let batchcount_ok = tempdraw.instancecount() + drawinfo.instancecount() < engineopt.max_instance_batch_count;
                        let batchmem_ok = tempdraw.can_batch_instance_memory(&drawinfo, true);
                        let batchmaxsize_ok = combinebuffer.usedsize() + drawinfo.instancedatasize() < combinebuffer.maxcombinesize;
                        let batchcomb_ok = combinebuffer.combinecommon(drawinfo.instancedatasize());
                        if batchcount_ok && batchmem_ok && batchmaxsize_ok && batchcomb_ok {
                            _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                            lastdraw = Some(tempdraw);
                        } else {
                            // lastdraw 转 DrawObj
                            collect_draw_batch(&mut combinebuffer, tempdraw, &lastinsdata, &mut renderer, &mut allocator, &device, &queue, &mut count_vertex);
                            lastinsdata.reset();
                            lastinsdata.data.start = combinebuffer.usedsize();
                            _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                            lastdraw = Some(drawinfo);
                        }
                    } else {
                        lastinsdata.reset();
                        lastinsdata.data.start = combinebuffer.usedsize();
                        _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                        lastdraw = Some(drawinfo);
                    }
                });

                // lastdraw 转 DrawObj
                if let Some(tempdraw) = lastdraw.take() {
                    collect_draw_batch(&mut combinebuffer, tempdraw, &lastinsdata, &mut renderer, &mut allocator, &device, &queue, &mut count_vertex);
                    lastinsdata.reset();
                    lastinsdata.data.start = combinebuffer.usedsize();
                    lastdraw = None;
                }
                transparent_list.drain(..).for_each(|drawinfo| {
                    if let Some(tempdraw) = lastdraw.take() {
                        let batchcount_ok = tempdraw.instancecount() + drawinfo.instancecount() < engineopt.max_instance_batch_count;
                        let batchmem_ok = tempdraw.can_batch_instance_memory(&drawinfo, true);
                        let batchmaxsize_ok = combinebuffer.usedsize() + drawinfo.instancedatasize() < combinebuffer.maxcombinesize;
                        let batchcomb_ok = combinebuffer.combinecommon(drawinfo.instancedatasize());
                        if batchcount_ok && batchmem_ok && batchmaxsize_ok && batchcomb_ok {
                            _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                            lastdraw = Some(tempdraw);
                        } else {
                            // lastdraw 转 DrawObj
                            collect_draw_batch(&mut combinebuffer, tempdraw, &lastinsdata, &mut renderer, &mut allocator, &device, &queue, &mut count_vertex);
                            lastinsdata.reset();
                            lastinsdata.data.start = combinebuffer.usedsize();
                            _combine_instance(&mut combinebuffer, &mut lastinsdata, &drawinfo);
                            lastdraw = Some(drawinfo);
                        }
                    } else {
                        lastinsdata.reset();
                        lastinsdata.data.start = combinebuffer.usedsize();
                        _combine_instance( &mut combinebuffer, &mut lastinsdata, &drawinfo);
                        lastdraw = Some(drawinfo);
                    }
                });
                // lastdraw 转 DrawObj
                if let Some(tempdraw) = lastdraw.take() {
                    collect_draw_batch(&mut combinebuffer, tempdraw, &lastinsdata, &mut renderer, &mut allocator, &device, &queue, &mut count_vertex);
                    lastinsdata.reset();
                    lastinsdata.data.start = combinebuffer.usedsize();
                    // lastdraw = None;
                }
            } else {
                // log::warn!("Renderer Viewer Not Found: {:?}, Camera {:?}, {:?}", _id_renderer, id_viewer.0, (param.enable.0, passtag));
            }

            performance.drawcalls += renderer.draws.list.len() as u32;
            // log::warn!("Renderer {:?},", (renderer.draws.list.len(), count_vertex, passtag));
            renderer.vertexs = count_vertex;
        });

        combinebuffer.apply(&queue);

        if performance.debug { performance.drawobjs = (pi_time::Instant::now() - performance.t_drawobjs).as_micros() as u32; }
    }


fn shader(
    _id_pass: Entity,
    meta: &Handle<ShaderEffectMeta>,
    key_meta: &Atom,
    vb: &VertexBufferLayoutsComp,
    bindgroups: &BindGroups3D,
    renderalignment: ERenderAlignmentForShader,
    assets: & ShareAssetMgr<Shader3D>,
    device: &RenderDevice,
    engineopt: &EngineCustomPlugins,
) -> Result<Handle<Shader3D>, Shader3D> {
    let key_attributes = &vb.1;

    let (set0, set1, set2, set3) = (&bindgroups.scene, &bindgroups.model, bindgroups.matvalues.as_ref(), bindgroups.textures.as_ref());

    let mut hash = DefaultHasher::default();
    if let Some(set) = set0 { set.hash_for_shader(&mut hash); }
    if let Some(set) = set1 { set.hash_for_shader(&mut hash); }
    if let Some(set) = set2 { set.hash_for_shader(&mut hash); }
    if let Some(set) = set3 { set.hash_for_shader(&mut hash); }
    let key_shader = KeyShader3D {
        key_meta: key_meta.clone(),
        bind_defines: meta.binddefines,
        key_attributes: key_attributes.clone(),
        renderalignment: renderalignment,
        bindgroups_for_shader: hash.finish(),
    };

    if let Some(shader) = assets.get(&key_shader) {
        // log::debug!("SysPassShaderRequestByModel: 4");
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
        let mut vs_running_before_effect_snippets = vec![];
        let mut fs_running_before_effect_snippets = vec![];
        let fs_running_after_effect_snippets = vec![];
    
        // log::error!("Shader: {:?}", key_meta);
        // log::error!("{:?}", key_attributes);
        // log::error!("{:?}", key_attributes.vs_define_code());
    
        vs_defined_snippets.push(key_attributes.vs_define_code());
        vs_extend_varying += &key_attributes.vs_varying_code(meta.varyings.0.len() as u32, meta);
        fs_extend_varying += &key_attributes.fs_varying_code(meta.varyings.0.len() as u32, meta);
    
        if let Some(set) = set0 {
            vs_defined_snippets.push(set.vs_define_code(setidx));
            fs_defined_snippets.push(set.fs_define_code(setidx));
            setidx += 1;
        }
    
        if let Some(set) = set1 {
            let skin = set.key().key.skin;
            vs_defined_snippets.push(set.vs_define_code(setidx));
            fs_defined_snippets.push(set.fs_define_code(setidx));
    
            vs_running_attribute_snippets.push(set.vs_running_model_snippet(meta));
            vs_running_model_snippets.push(skin.running_code());
            vs_running_model_snippets.push(renderalignment.running_code());
    
            vs_defined_snippets.push(renderalignment.define_code());
    
            setidx += 1;
        }
        vs_running_attribute_snippets.push(key_attributes.vs_running_code());
        fs_running_before_effect_snippets.push(key_attributes.fs_running_code(meta));
    
        if let Some(set) = set2 {
            vs_defined_snippets.push(set.vs_define_code(setidx, meta, engineopt));
            fs_defined_snippets.push(set.fs_define_code(setidx, meta, engineopt));
            setidx += 1;
        }
        
        if let Some(set) = set3 {
            vs_defined_snippets.push(set.vs_define_code(setidx, meta, engineopt));
            fs_defined_snippets.push(set.fs_define_code(setidx, meta, engineopt));
            setidx += 1;
        }

        let shader = meta.build_2(
            &device,
            &key_meta,
            &vs_defined_snippets,
            &vs_extend_varying,
            &fs_extend_varying,
            &vs_running_attribute_snippets,
            &vs_running_model_snippets,
            &vs_running_before_effect_snippets, &vs_running_after_effect_snippets,
            &fs_defined_snippets,
            &fs_running_before_effect_snippets, &fs_running_after_effect_snippets,
            engineopt
        );

        assets.insert(key_shader, shader)
    }
}

fn pipeline(
    shader: &Handle<Shader3D>,
    bindgroups: &BindGroups3D,
    vb: &VertexBufferLayoutsComp,
    colorformat: &ColorFormat,
    depthstencilformat: &DepthStencilFormat,
    blend: ModelBlend,
    depth_state: &DepthState, stencil_state: &StencilState,
    cull: &PrimitiveState,
    _id_pass: Entity,
    assets: &ShareAssetMgr<Pipeline3D>,
    device: &RenderDevice,
) -> Result<Handle<Pipeline3D>, Pipeline3D> {
    // log::error!("Create pipeline");
    
    let key_shader = shader.key().clone();
    log::error!("Shader: {:?}", &key_shader);
    let bind_group_layouts = bindgroups.bind_group_layouts();
    let key_bindgroup_layouts = KeyPipelineFromBindGroup(bindgroups.key_bindgroup_layouts());

    let key_vertex_layouts = vb.0.as_key_pipeline_from_vertex_layout();

    let pass_color_format = colorformat.val();
    let pass_depth_format = depthstencilformat.val();
    let depth_stencil = if let Some(pass_depth_format) = pass_depth_format {
        Some(
            depth_stencil_state(
                pass_depth_format,
                depth_state, stencil_state
            )
        )
    } else { None };

    let targets = RenderTargetState::color_target(pass_color_format, &blend);
    let key_state = KeyRenderPipelineState {
        primitive: cull.state(),
        target_state: targets[0].clone(),
        depth_stencil: depth_stencil,
        multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }
    };

    // log::warn!("{:?}", key_state);

    let key_pipeline = KeyPipeline3D {
        key_state,
        key_shader,
        key_bindgroup_layouts,
        key_vertex_layouts,
    };

    let key_u64 = key_pipeline.to_u64();

    if let Some(pipeline) = assets.get(&key_u64) {
        // log::debug!("SysPipeline: 3 Pass");
        // *oldpipeline = PassPipeline::new(Some(pipeline));
        Ok(pipeline)
    } else {
        let pipeline = KeyPipeline3D::create(key_pipeline, shader.clone(), bind_group_layouts, &device);
        assets.insert(key_u64, pipeline)
    }
}

fn collect_draw<'w>(
    is_transparent: bool,
    pass: u8,
    rendergeo: &'w RenderGeometry,
    bindgroups: &'w BindGroups3D,
    pipeline: &'w Pipeline3DUsage,
    indicerange: &'w IndiceRenderRange,
    vertexrange: &'w VertexRenderRange,
    distance: f32,
    sort_param: &'w RenderQueueSortParam,
    opaque_list: & mut Vec<DrawTmpRef<'w>>,
    transparent_list: & mut Vec<DrawTmpRef<'w>>,
    instancessortinfo: &'w InstancedMeshTransparentSortCollection,
    distancecomp: &ViewerDistanceCompute,
    viewposition: &(Number, Number, Number),
    viewdirection: &(Number, Number, Number),
) {
    if rendergeo.instance_slot.is_some() {
        if instancessortinfo.ranges.len() > 0 {
            instancessortinfo.ranges.iter().for_each(|(alphaindex, range, center)| {
                let distance = distancecomp.distance(&viewposition, &viewdirection, center);
                if range.start < range.end && range.end <= instancessortinfo.count as u32 {
                    let mut draw = DrawTmpRef {
                        rendergeo,
                        pipeline,
                        bindgroups,
                        indicerange,
                        vertexrange,
                        inscombinerange: range.clone(),
                        instancessortinfo,
                        pass,
                        distance,
                        queue: sort_param.clone(),
                    };
                    
                    if is_transparent == false {
                        opaque_list.push(draw);
                    } else {
                        // let mut queue = sort_param.clone();
                        draw.queue.index = *alphaindex;
                        transparent_list.push(draw);

                    }
                } else {
                    // log::error!("instancessortinfo Error {:?}", (range, instance_memory.itemcount));
                }
            });
        } else {
            // let range = Range { start: 0, end: instancessortinfo.count as u32 };
            // let draw = DrawTmpRef {
            //     rendergeo,
            //     pipeline,
            //     bindgroups,
            //     indicerange,
            //     vertexrange,
            //     inscombinerange: range.clone(),
            //     instancessortinfo,
            //     pass,
            //     distance,
            //     queue: sort_param.clone(),
            // };
            // log::warn!("Range {:?}", range);
            
            // if is_transparent == false {
            //     opaque_list.push(draw);
            // } else {
            //     transparent_list.push(draw);
            // }
        }
    } else {
        let draw = DrawTmpRef {
            rendergeo,
            pipeline,
            bindgroups,
            indicerange,
            vertexrange,
            inscombinerange: Range { start: 0, end: 0 },
            instancessortinfo,
            pass,
            distance,
            queue: sort_param.clone(),
        };
        if is_transparent == false {
            opaque_list.push(draw);
        } else {
            transparent_list.push(draw);
        }
    }
}

fn _combine_instance(
    combinedata: & mut CombineBuffer,
    lastinsdata: &mut EVerteicesInstance,
    drawinfo: &DrawTmpRef
) {
    if let Some(slot) = &drawinfo.rendergeo.instance_slot {
        // log::warn!("_combine_instance {:?}", &drawinfo.inscombinerange);
        if drawinfo.instancessortinfo.count > 0 && drawinfo.inscombinerange.start < drawinfo.inscombinerange.end {
            let size = drawinfo.instancessortinfo.data.len() / drawinfo.instancessortinfo.count as usize;
            let start = drawinfo.inscombinerange.start as usize * size;
            let end = drawinfo.inscombinerange.end as usize * size;

            combinedata.record(&drawinfo.instancessortinfo.data.as_slice()[start..end]);
            lastinsdata.data.end = combinedata.usedsize();
            lastinsdata.itemcount += drawinfo.inscombinerange.end - drawinfo.inscombinerange.start;
            lastinsdata.slot = *slot as u8;
        }
    }
}

fn collect_draw_batch(
    combinebuffer: &mut CombineBuffer,
    tempdraw: DrawTmpRef,
    instancedata: &EVerteicesInstance,
    renderer: &mut Renderer,
    allocator: &mut VertexBufferAllocator3D,
    device: &PiRenderDevice,
    queue: &PiRenderQueue,
    count_vertex: &mut usize,
) {
    let geo = tempdraw.rendergeo;
    if tempdraw.rendergeo.instance_slot.is_some() {
        let mem: &EVerteicesInstance = instancedata;

        if mem.itemcount == 0 {
            // log::warn!("mem.itemcount 0 {:?}", (mem.data.len(), &tempdraw.inscombinerange));
            return;
        } else if mem.data.end <= mem.data.start  {
            // log::warn!("mem.data 0 {:?}", (mem.data.len(), &tempdraw.inscombinerange));
            return;
        };

        let size_per_value = (mem.data.end - mem.data.start) as u32 / mem.itemcount;
        let instances = Range { start: 0, end: mem.itemcount, };
        // log::warn!("Buffer {:?}", (&mem.data, size_per_value, &instances));
        let range = {
            let size = wgpu::COPY_BUFFER_ALIGNMENT as usize;
            let temp = (mem.data.end / size) * size;
            let start = mem.data.start;
            let mut end = mem.data.end;
            if temp < mem.data.end {
                end = temp + size;
                let hascount = combinebuffer.data.len();
                if hascount < end {
                    let placehold: [u8;4] = [0, 0, 0, 0];
                    let count = end - hascount;
                    combinebuffer.record(&placehold[0..count]);
                }
            }
            Range { start, end }
        };
        let data = combinebuffer.data(&range, allocator, device, queue);

        if let Some(data) = data {
            // log::warn!("Draw Instance {:?}", (&instances, &range));
            let mut draw = DrawObj {
                pipeline: Some(tempdraw.pipeline.clone()),
                bindgroups: tempdraw.bindgroups.groups(),
                vertices: tempdraw.rendergeo.vertices(),
                instances,
                vertex: tempdraw.vertexrange.apply(geo),
                indices: tempdraw.indicerange.apply(geo),
            };
            draw.insert_vertices(RenderVertices { slot: mem.slot as u32, buffer: EVerticesBufferUsage::EVBRange(Arc::new(data)), buffer_range: None, size_per_value: size_per_value as u64 });
            draw.instances = Range { start: 0, end: mem.itemcount };
            let vertex = if let Some(indices) = &draw.indices {
                indices.value_range().end - indices.value_range().start
            } else { draw.vertex.end - draw.vertex.start };
            if vertex == 0 {
                return;
            }
            *count_vertex += (vertex * (draw.instances.end - draw.instances.start)) as usize;
            renderer.draws.list.push(Arc::new(draw));
        } else {
            // log::error!("create_not_updatable_buffer fail {:?}", bytelen);
            // let data = instancedcache.instance_initial_buffer();
            // EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))
        };
    } else {
        let instances = geo.instances();
        let vertex = tempdraw.vertexrange.apply(geo);
        let indices = tempdraw.indicerange.apply(geo);
        let vertexcount = if let Some(indices) = &indices {
            indices.value_range().end - indices.value_range().start
        } else { vertex.end - vertex.start };
        if vertexcount == 0 || instances.start >= instances.end { return; }

        let draw = DrawObj {
            pipeline: Some(tempdraw.pipeline.clone()),
            bindgroups: tempdraw.bindgroups.groups(),
            vertices: tempdraw.rendergeo.vertices(),
            instances,
            vertex,
            indices,
        };

        *count_vertex += (vertexcount * (draw.instances.end - draw.instances.start)) as usize;
        renderer.draws.list.push(Arc::new(draw));
    }
}