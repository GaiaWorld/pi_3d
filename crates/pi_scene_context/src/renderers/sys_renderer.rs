use std::{marker::PhantomData, time::Instant, sync::Arc};

use pi_ecs::{prelude::{Query, Commands, Res, Component, ResMut}, query::{Or, Changed, With}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::{
    rhi::{device::RenderDevice},
    render_3d::{
        shader::{shader::{KeyShader3D}, instance_code::EInstanceCode}
    },
    renderer::{
        vertex_buffer::{VertexBufferLayouts, KeyPipelineFromAttributes},
        pipeline::KeyRenderPipelineState
    }
};
use pi_share::Share;
use crate::{
    viewer::{ViewerID, ModelListAfterCulling},
    pass::*,
    geometry::{geometry::{RenderGeometry}, sys_vertex_buffer_use::SysRenderGeometryInit},
    cameras::camera::CameraViewport,
};

use super::{
    render_primitive::PrimitiveState,
    base::*,
    pass::*,
    render_depth_and_stencil::RenderDepthAndStencil,
    render_blend::RenderBlend,
    render_target_state::RenderTargetState,
    renderer::Renderer,
    sys_renderer_pre::{SysSet0ModifyByRendererID, SysSet0ModifyFromScene, SysSet1ModifyByModel, SysSet2ModifyByRendererID, SysSet2ModifyByModel, SysSet1ModifyByRendererID, SysBufferAllocatorUpdate}
};


/// 渲染器搜集渲染
#[derive(Debug, Default)]
pub struct SysPassBindGroups<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysPassBindGroups<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSet0ModifyByRendererID::<T, I>::key(), SysSet0ModifyFromScene::<T, I>::key(),
            SysSet1ModifyByRendererID::<T, I>::key(), SysSet1ModifyByModel::<T, I>::key(),
            SysSet2ModifyByRendererID::<T, I>::key(), SysSet2ModifyByModel::<T, I>::key(),
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysPassBindGroups<T, I> {
    #[system]
    fn sys(
        passes: Query<
            GameObject,
            (ObjectID, &PassSource, &PassReady, &PassBindGroupScene, &PassBindGroupModel, &PassBindGroupTextureSamplers, &PassBindGroups, &T),
            Or<(Changed<PassReady>, Changed<PassBindGroupScene>, Changed<PassBindGroupModel>, Changed<PassBindGroupTextureSamplers>)>
        >,
        mut cmd: Commands<GameObject, PassBindGroups>,
    ) {
        passes.iter().for_each(|(id_pass, id_model, ready, set0, set1, set2, old, _)| {
            if let Some((key_meta, meta)) = ready.val() {
                if let (Some(set0), Some(set1)) = (set0.val(), set1.val()) {
                    if meta.textures.len() > 0 && set2.val().is_none() {
                        if old.val().is_some() {
                            cmd.insert(id_pass, PassBindGroups::new(None));
                        }
                    } else {
                        cmd.insert(id_pass, PassBindGroups::new(Some(
                            BindGroups3D::create(set0.clone(), set1.clone(), set2.val().clone())
                        )));
                    }
                } else {
                    if old.val().is_some() {
                        cmd.insert(id_pass, PassBindGroups::new(None));
                    }
                }
            } else {
                if old.val().is_some() {
                    cmd.insert(id_pass, PassBindGroups::new(None));
                }
            }
        });
    }
}


/// 渲染器搜集渲染
#[derive(Debug, Default)]
pub struct SysPassShaderRequestByModel<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysPassShaderRequestByModel<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysPassBindGroups::<T, I>::key(),
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysPassShaderRequestByModel<T, I> {
    #[system]
    fn sys(
        models: Query<
            GameObject,
            (
                ObjectID,
                &VertexBufferLayouts, 
                &EInstanceCode,
                &I,
            ),
            Or<
                (
                    Changed<VertexBufferLayouts>,
                    Changed<EInstanceCode>,
                )
            >
        >,
        passes: Query<GameObject, (&PassReady, &PassBindGroups, &PassShader), With<T>>,
        mut shader_cmd: Commands<GameObject, PassShader>,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<RenderDevice>,
    ) {
        let time1 = Instant::now();

        models.iter().for_each(
            |(id_model, vb, instance, passid)| {
                let id_pass = passid.id();
                if let Some((ready, bindgroups, old_shader)) = passes.get(id_pass.clone()) {
                    if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                        
                        let key_attributes = vb.as_key_shader_from_attributes();
                        let key_shader_defines = 0;

                        let mut key_set_blocks = bindgroups.key_set_blocks();

                        let key_shader = KeyShader3D {
                            key_meta: key_meta.clone(),
                            key_attributes,
                            key_set_blocks,
                            defines: 0,
                        };

                        let (set0, set1, set2) = (&bindgroups.scene, &bindgroups.model, bindgroups.textures.as_ref());
                        let set2 = if let Some(set2) = set2 {
                            Some(set2.as_ref())
                        } else { None };
                
                        if let Some(shader) = shader_center.get(&key_shader) {
                            shader_cmd.insert(id_pass, PassShader::from((shader, None)));
                        } else {
                            if !shader_center.check(&key_shader) {
                                let shader = meta.build(&device, &key_shader.key_meta, &key_shader.key_attributes, &instance, set0.as_ref(), set1.as_ref(), set2, None);
                                shader_center.add(&key_shader, shader, None);
                            }
                            shader_loader.request(id_pass, &key_shader);
                        }
                    } else {
                        if old_shader.val().is_some() {
                            log::info!("SysPassShaderRequestByModel: No Ready");
                            shader_cmd.insert(id_pass, PassShader(None));
                        }
                    }
                }
            }
        );

        log::info!("SysPassShaderRequestByModel: {:?}", Instant::now() - time1);
    }
}

/// 渲染器搜集渲染
#[derive(Debug, Default)]
pub struct SysPassShaderRequestByPass<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysPassShaderRequestByPass<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysPassShaderRequestByModel::<T, I>::key(),
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysPassShaderRequestByPass<T, I> {
    #[system]
    fn sys(
        models: Query<
            GameObject,
            (
                &VertexBufferLayouts, &EInstanceCode, &I,
            ),
        >,
        passes: Query<
            GameObject,
            (ObjectID, &PassSource, &PassReady, &PassBindGroups, &PassShader, &T),
            Or<(Changed<PassReady>, Changed<PassBindGroups>)>
        >,
        mut shader_cmd: Commands<GameObject, PassShader>,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<RenderDevice>,
    ) {
        let time1 = Instant::now();

        passes.iter().for_each(|(id_pass, id_model, ready, bindgroups, old_shader, _)| {
            if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                if let Some((vb, instance, passid)) = models.get(id_model.0) {
                    let key_attributes = vb.as_key_shader_from_attributes();
                    let key_shader_defines = 0;
    
                    let mut key_set_blocks = bindgroups.key_set_blocks();
    
                    let key_shader = KeyShader3D {
                        key_meta: key_meta.clone(),
                        key_attributes,
                        key_set_blocks,
                        defines: 0,
                    };
    
                    let (set0, set1, set2) = (&bindgroups.scene, &bindgroups.model, bindgroups.textures.as_ref());
                    let set2 = if let Some(set2) = set2 {
                        Some(set2.as_ref())
                    } else { None };
            
                    if let Some(shader) = shader_center.get(&key_shader) {
                        shader_cmd.insert(id_pass, PassShader::from((shader, None)));
                    } else {
                        if !shader_center.check(&key_shader) {
                            let shader = meta.build(&device, &key_shader.key_meta, &key_shader.key_attributes, &instance, set0.as_ref(), set1.as_ref(), set2, None);
                            shader_center.add(&key_shader, shader, None);
                        }
                        shader_loader.request(id_pass, &key_shader);
                    }
                } else {
                    if old_shader.val().is_some() {
                        log::info!("SysPassShaderRequestByModel: No Geo");
                        shader_cmd.insert(id_pass, PassShader(None));
                    }
                }
            } else {
                if old_shader.val().is_some() {
                    log::info!("SysPassShaderRequestByModel: No Ready");
                    shader_cmd.insert(id_pass, PassShader(None));
                }
            }
        });

        log::info!("SysPassShaderRequestByPass: {:?}", Instant::now() - time1);
    }
}

pub struct SysPassShaderLoad;
impl TSystemStageInfo for SysPassShaderLoad {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysPassShaderRequestByModel::<Pass01, PassID01>::key(), SysPassShaderRequestByPass::<Pass01, PassID01>::key(),
        ]
    }
}
#[setup]
impl SysPassShaderLoad {
    #[system]
    fn sys(
        passes: Query<GameObject, &PassSource>,
        mut shader_cmd: Commands<GameObject, PassShader>,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<RenderDevice>,
    ) {
        let time1 = Instant::now();
        shader_center.single_create().iter().for_each(|(key, value)| {
            shader_loader.loaded(key, value).drain(..).for_each(|(entity, component)| {
                shader_cmd.insert(entity, component);
            })
        });

        log::info!("SysPassShaderLoad: {:?}", Instant::now() - time1);
    }
}


#[derive(Debug, Default)]
pub struct SysPassPipelineRequestByModel<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysPassPipelineRequestByModel<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysPassShaderLoad::key(), SysRenderGeometryInit::key()
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysPassPipelineRequestByModel<T, I> {
    #[system]
    fn sys(
        models: Query<
            GameObject,
            (
                &VertexBufferLayouts, &PrimitiveState, &RenderDepthAndStencil, &RenderBlend,
                &RenderGeometry, &I
            ),
            Or<(Changed<PrimitiveState>, Changed<RenderDepthAndStencil>, Changed<RenderBlend>, Changed<RenderGeometry>)>
        >,
        passes: Query<
            GameObject,
            (&PassShader, &PassBindGroups, &PassPipeline),
            With<T>
        >,
        mut pipeline_cmd: Commands<GameObject, PassPipeline>,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<RenderDevice>,
    ) {
        let time1 = Instant::now();

        models.iter().for_each(| (vb, primitive, depth_stencil, blend, rendergeo, passid) |{
            let id_pass = passid.id();
            if let Some((shader, bindgroups, old_draw)) = passes.get(id_pass) {
                if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                    let key_shader = shader.key().clone();
                    let mut bind_group_layouts = bindgroups.bind_group_layouts();
                    let mut key_bindgroup_layouts = bindgroups.key_bindgroup_layouts();

                    let key_vertex_layouts = KeyPipelineFromAttributes::new(vb.clone());
    
                    let targets = RenderTargetState::color_target(blend);
                    let key_state = KeyRenderPipelineState {
                        primitive: primitive.state,
                        target_state: vec![targets[0].clone()],
                        depth_stencil: depth_stencil.0.clone(),
                        multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }
                    };

                    let key_pipeline = KeyPipeline3D {
                        key_state,
                        key_shader,
                        key_bindgroup_layouts,
                        key_vertex_layouts,
                    };

                    let key_u64 = key_pipeline.to_u64();

                    if let Some(pipeline) = pipeline_center.get(&key_u64) {
                        pipeline_cmd.insert(id_pass, PassPipeline::new(Some(pipeline)));
                    } else {
                        if !pipeline_center.check(&key_u64) {
                            let pipeline = KeyPipeline3D::create(key_pipeline, shader.clone(), bind_group_layouts, &device);
                            pipeline_center.add(&key_u64, pipeline, None);
                        }
                        pipeline_loader.request(id_pass, &key_u64);
                    }
                } else {
                    if old_draw.val().is_some() {
                        log::info!("SysPassPipelineRequest: No Shader");
                        pipeline_cmd.insert(id_pass, PassPipeline::new(None));
                    }
                }
            }
        });

        log::info!("SysPassPipelineRequest: {:?}", Instant::now() - time1);
    }
}

#[derive(Debug, Default)]
pub struct SysPassPipelineRequestByPass<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysPassPipelineRequestByPass<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysPassPipelineRequestByModel::<T, I>::key(),
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysPassPipelineRequestByPass<T, I> {
    #[system]
    fn sys(
        models: Query<
            GameObject,
            (
                &VertexBufferLayouts, &PrimitiveState, &RenderDepthAndStencil, &RenderBlend,
                &RenderGeometry,
            ),
        >,
        passes: Query<
            GameObject,
            (ObjectID, &PassSource, &PassBindGroups, &PassShader),
            Changed<PassShader>
        >,
        mut pipeline_cmd: Commands<GameObject, PassPipeline>,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<RenderDevice>,
    ) {
        let time1 = Instant::now();

        passes.iter().for_each(|(id_pass, id_model, bindgroups, shader)| {
            if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                if let Some((vb, primitive, depth_stencil, blend, rendergeo)) = models.get(id_model.0) {
                    let key_shader = shader.key().clone();
                    let mut bind_group_layouts = bindgroups.bind_group_layouts();
                    let mut key_bindgroup_layouts = bindgroups.key_bindgroup_layouts();

                    let key_vertex_layouts = KeyPipelineFromAttributes::new(vb.clone());
    
                    let targets = RenderTargetState::color_target(blend);
                    let key_state = KeyRenderPipelineState {
                        primitive: primitive.state,
                        target_state: vec![targets[0].clone()],
                        depth_stencil: depth_stencil.0.clone(),
                        multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }
                    };

                    let key_pipeline = KeyPipeline3D {
                        key_state,
                        key_shader,
                        key_bindgroup_layouts,
                        key_vertex_layouts,
                    };

                    let key_u64 = key_pipeline.to_u64();

                    if let Some(pipeline) = pipeline_center.get(&key_u64) {
                        pipeline_cmd.insert(id_pass, PassPipeline::new(Some(pipeline)));
                    } else {
                        if !pipeline_center.check(&key_u64) {
                            let pipeline = KeyPipeline3D::create(key_pipeline, shader.clone(), bind_group_layouts, &device);
                            pipeline_center.add(&key_u64, pipeline, None);
                        }
                        pipeline_loader.request(id_pass, &key_u64);
                    }
                } else {
                    log::info!("SysPassPipelineRequest: No Geo");
                    pipeline_cmd.insert(id_pass, PassPipeline::new(None));
                }
            } else {
                log::info!("SysPassPipelineRequest: No Shader");
                pipeline_cmd.insert(id_pass, PassPipeline::new(None));
            }
        });

        log::info!("SysPassPipelineRequest: {:?}", Instant::now() - time1);
    }
}


pub struct SysPassPipeline3DLoad;
impl TSystemStageInfo for SysPassPipeline3DLoad {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysPassPipelineRequestByModel::<Pass01, PassID01>::key(), SysPassPipelineRequestByPass::<Pass01, PassID01>::key()
        ]
    }
}
#[setup]
impl SysPassPipeline3DLoad {
    #[system]
    fn sys(
        mut pipeline_cmd: Commands<GameObject, PassPipeline>,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<RenderDevice>,
    ) {
        let time1 = Instant::now();

        pipeline_center.single_create().iter().for_each(|(key, value)| {
            pipeline_loader.loaded(key, value).drain(..).for_each(|(entity, component)| { pipeline_cmd.insert(entity, component) })
        });

        log::info!("SysPassDrawLoad: {:?}", Instant::now() - time1);
    }
}

pub struct SysPassDraw<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysPassDraw<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysPassPipeline3DLoad::key()
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysPassDraw<T, I> {
    #[system]
    fn sys(
        models: Query<GameObject, &RenderGeometry>,
        passes: Query<GameObject, (ObjectID, &PassSource, &PassBindGroups, &PassPipeline, &PassDraw, &T), Changed<PassPipeline>>,
        mut draw_cmd: Commands<GameObject, PassDraw>,
    ) {
        let time1 = Instant::now();

        passes.iter().for_each(|(id_pass, id_model, bindgroups, pipeline, old_draw, _)| {
            if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                if let Some(rendergeo) = models.get(id_model.0) {
                    let draw = DrawObj3D {
                        pipeline: Some(pipeline.clone()),
                        bindgroups: bindgroups.groups(),
                        vertices: rendergeo.vertices(),
                        instances: rendergeo.instances(),
                        indices: rendergeo.indices.clone(),
                    };

                    draw_cmd.insert(id_pass, PassDraw(Some(Arc::new(draw))));
                }
            } else {
                if old_draw.0.is_some() {
                    draw_cmd.insert(id_pass, PassDraw(None));
                }
            }
        });

        log::info!("SysPassDrawLoad: {:?}", Instant::now() - time1);
    }
}

pub struct SysRendererDraws;
impl TSystemStageInfo for SysRendererDraws {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key(),
            // SysModelListAfterCullingTick::key(), 在 Command 阶段, 因此屏蔽
            SysPassDraw::<Pass01, PassID01>::key(),
        ]
    }
}
#[setup]
impl SysRendererDraws {
    #[system]
    fn sys(
        mut renderers: Query<
            GameObject,
            (
                &ViewerID, &mut Renderer, &PassTagOrders, 
            )
        >,
        viewers: Query<
            GameObject,
            (&ModelListAfterCulling, Option<&CameraViewport>),
        >,
        models: Query<
            GameObject,
            (&PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08)
        >,
        passes: Query<
            GameObject,
            &PassDraw
        >
    ) {
        let time1 = Instant::now();

        let pass01_query = 

        renderers.iter_mut().for_each(|(id_viewer, mut renderer, passtag_orders)| {
            renderer.clear();
            if let Some((list_model, viewport)) = viewers.get(id_viewer.0) {
                if let Some(viewport) = viewport {
                    renderer.draws.viewport = (viewport.x, viewport.y, viewport.w, viewport.h, viewport.mindepth, viewport.maxdepth);
                } else {
                    renderer.draws.viewport = (0., 0., 1., 1., 0., 1.);
                }
                list_model.0.iter().for_each(|id_obj| {
                    if let Some(passrecord) = models.get(id_obj.clone()) {
                        passtag_orders.0.iter().for_each(|tag| {
                            let pass = tag.as_pass();
                            if pass == EPassTag::PASS_TAG_01 {
                                if let Some(PassDraw(Some(draw))) = passes.get(passrecord.0.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_02 {
                                if let Some(PassDraw(Some(draw))) = passes.get(passrecord.1.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_03 {
                                if let Some(PassDraw(Some(draw))) = passes.get(passrecord.2.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_04 {
                                if let Some(PassDraw(Some(draw))) = passes.get(passrecord.3.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_05 {
                                if let Some(PassDraw(Some(draw))) = passes.get(passrecord.4.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_06 {
                                if let Some(PassDraw(Some(draw))) = passes.get(passrecord.5.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_07 {
                                if let Some(PassDraw(Some(draw))) = passes.get(passrecord.6.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_08 {
                                if let Some(PassDraw(Some(draw))) = passes.get(passrecord.7.0) { renderer.draws.list.push(draw.clone()) }
                            }
                        });
                    }
                });
            }
        });

        log::info!("SysRendererDraws: {:?}", Instant::now() - time1);
    }
}
