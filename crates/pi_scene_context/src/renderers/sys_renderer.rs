use std::{marker::PhantomData, time::Instant, sync::Arc};

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::{prelude::{Query, Commands, Res, Component}, query::{Or, Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::{
    rhi::{device::RenderDevice, asset::RenderRes},
    render_3d::{
        bind_groups::{scene::BindGroupScene, model::BindGroupModel, texture_sampler::BindGroupTextureSamplers},
        shader::{shader::{KeyShader3D, Shader3D, EKeyShader3DSetBlock}, instance_code::EInstanceCode, shader_effect_meta::ShaderEffectMeta}
    },
    renderer::{
        vertex_buffer::{VertexBufferLayouts, KeyPipelineFromAttributes},
        indices::{IndicesBufferDesc, AssetResBufferIndices},
        shader::KeyShaderMeta,
        pipeline::KeyRenderPipelineState
    }
};
use pi_share::Share;
use crate::{viewer::{ViewerID, ModelListAfterCulling}, pass::*, geometry::{geometry::{RenderGeometry}}, cameras::camera::CameraViewport};

use super::{
    render_primitive::PrimitiveState,
    pass::*,
    render_depth_and_stencil::RenderDepthAndStencil,
    render_blend::RenderBlend,
    render_target_state::RenderTargetState,
    renderer::Renderer,
    sys_renderer_pre::{SysSet0ModifyByRendererID, SysSet0ModifyFromScene, SysSet1ModifyByModel, SysSet2ModifyByRendererID, SysSet2ModifyByModel, SysSet1ModifyByRendererID, SysBufferAllocatorUpdate}
};

/// 渲染器搜集渲染
#[derive(Debug, Default)]
pub struct SysPassShaderUpdate<
    R: TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> + Component,
    S0: TPassData<Option<BindGroupScene>> + Component,
    S1: TPassData<Option<BindGroupModel>> + Component,
    S2: TPassData<Option<BindGroupTextureSamplers>> + Component,
    SS: TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> + Component,
>(pub(crate) PhantomData<(R, S0, S1, S2, SS)>);
impl<
    R: TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> + Component,
    S0: TPassData<Option<BindGroupScene>> + Component,
    S1: TPassData<Option<BindGroupModel>> + Component,
    S2: TPassData<Option<BindGroupTextureSamplers>> + Component,
    SS: TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> + Component,
> TSystemStageInfo for SysPassShaderUpdate<R, S0, S1, S2, SS> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSet0ModifyByRendererID::key(), SysSet0ModifyFromScene::key(),
            SysSet1ModifyByRendererID::key(), SysSet1ModifyByModel::key(),
            SysSet2ModifyByRendererID::key(), SysSet2ModifyByModel::key(),
        ]
    }
}
#[setup]
impl<
    R: TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> + Component,
    S0: TPassData<Option<BindGroupScene>> + Component,
    S1: TPassData<Option<BindGroupModel>> + Component,
    S2: TPassData<Option<BindGroupTextureSamplers>> + Component,
    SS: TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> + Component,
> SysPassShaderUpdate<R, S0, S1, S2, SS> {
    #[system]
    fn sys(
        models: Query<
            GameObject,
            (
                ObjectID,
                &R,
                &S0, &S1, &S2,
                &VertexBufferLayouts, 
                &EInstanceCode,
            ),
            Or<
                (
                    Changed<R>,
                    Changed<S0>, Changed<S1>, Changed<S2>,
                    Changed<VertexBufferLayouts>,
                    Changed<EInstanceCode>
                )
            >
        >,
        mut shader_cmd: Commands<GameObject, SS>,
        asset_shader: Res<Share<AssetMgr<Shader3D>>>,
        device: Res<RenderDevice>,
    ) {
        let time1 = Instant::now();

        models.iter().for_each(
            |(
                id_obj,
                ready,
                set_0, set_1, set_2,
                vb,
                instance
            )| {
                // log::info!("SysPassShaderUpdate: >>>>>>>>>> {:?}", ready.val());
                if let Some((key_meta, meta, )) = ready.val() {
                    match (set_0.val(), set_1.val()) {
                        (Some(set0), Some(set1)) => {
                            let key_attributes = vb.as_key_shader_from_attributes();
                            let key_shader_defines = 0;

                            let mut bindgroups = BindGroups3D([None, None, None, None]);
                            let mut key_set_blocks = [None, None, None, None];
                            key_set_blocks[0] = Some(EKeyShader3DSetBlock::Scene(set0.key.clone()));
                            key_set_blocks[1] = Some(EKeyShader3DSetBlock::Model(set1.key.clone()));
                            bindgroups.0[0] = Some(set0.bind_group.clone());
                            bindgroups.0[1] = Some(set1.bind_group.clone());
                            if let Some(set2) = set_2.val() {
                                key_set_blocks[2] = Some(EKeyShader3DSetBlock::TextureSampler(set2.key.clone()));
                                bindgroups.0[2] = Some(set2.bind_group.clone());
                            };

                            let key_shader = KeyShader3D {
                                key_meta: key_meta.clone(),
                                key_attributes,
                                key_set_blocks: pi_render::renderer::shader::KeyShaderSetBlocks(key_set_blocks),
                                defines: 0,
                            };

                            if let Some(shader) = asset_shader.get(&key_shader) {
                                shader_cmd.insert(id_obj.clone(), SS::new(Some((key_shader.clone(), shader, bindgroups))));
                            } else {
                                let shader = meta.build(
                                    &device,
                                    key_meta,
                                    &key_shader.key_attributes,
                                    instance,
                                    set0,
                                    set1,
                                    set_2.val().as_ref(),
                                    None,
                                );
                                asset_shader.insert(key_shader.clone(), shader);
                                if let Some(shader) = asset_shader.get(&key_shader) {
                                    shader_cmd.insert(id_obj.clone(), SS::new(Some((key_shader.clone(), shader, bindgroups))));
                                } else {
                                    shader_cmd.insert(id_obj.clone(), SS::new(None))
                                }
                            }
                        },
                        _ => {
                            shader_cmd.insert(id_obj.clone(), SS::new(None))
                        }
                    }
                } else {
                    shader_cmd.insert(id_obj.clone(), SS::new(None))
                }
            }
        );

        log::info!("SysPassShaderUpdate: {:?}", Instant::now() - time1);
    }
}

pub type SysPass01ShaderUpdate = SysPassShaderUpdate<Pass01Ready, Pass01BindGroupScene, Pass01BindGroupModel, Pass01BindGroupTextureSamplers, Pass01Shader>;
pub type SysPass02ShaderUpdate = SysPassShaderUpdate<Pass02Ready, Pass02BindGroupScene, Pass02BindGroupModel, Pass02BindGroupTextureSamplers, Pass02Shader>;
pub type SysPass03ShaderUpdate = SysPassShaderUpdate<Pass03Ready, Pass03BindGroupScene, Pass03BindGroupModel, Pass03BindGroupTextureSamplers, Pass03Shader>;
pub type SysPass04ShaderUpdate = SysPassShaderUpdate<Pass04Ready, Pass04BindGroupScene, Pass04BindGroupModel, Pass04BindGroupTextureSamplers, Pass04Shader>;
pub type SysPass05ShaderUpdate = SysPassShaderUpdate<Pass05Ready, Pass05BindGroupScene, Pass05BindGroupModel, Pass05BindGroupTextureSamplers, Pass05Shader>;
pub type SysPass06ShaderUpdate = SysPassShaderUpdate<Pass06Ready, Pass06BindGroupScene, Pass06BindGroupModel, Pass06BindGroupTextureSamplers, Pass06Shader>;
pub type SysPass07ShaderUpdate = SysPassShaderUpdate<Pass07Ready, Pass07BindGroupScene, Pass07BindGroupModel, Pass07BindGroupTextureSamplers, Pass07Shader>;
pub type SysPass08ShaderUpdate = SysPassShaderUpdate<Pass08Ready, Pass08BindGroupScene, Pass08BindGroupModel, Pass08BindGroupTextureSamplers, Pass08Shader>;

pub struct SysPassDrawUpdate<
    SS: TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> + Component,
    D: TPassData<Option<Arc<DrawObj3D>>> + Component,
    T: TSystemStageInfo + Component,
>(pub(crate) PhantomData<(SS, D, T)>);
impl<
    SS: TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> + Component,
    D: TPassData<Option<Arc<DrawObj3D>>> + Component,
    T: TSystemStageInfo + Component,
> TSystemStageInfo for SysPassDrawUpdate<SS, D, T> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            T::key()
        ]
    }
}
#[setup]
impl<
    SS: TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> + Component,
    D: TPassData<Option<Arc<DrawObj3D>>> + Component,
    T: TSystemStageInfo + Component,
> SysPassDrawUpdate<SS, D, T> {
    #[system]
    fn sys(
        models: Query<
            GameObject,
            (
                ObjectID,
                &VertexBufferLayouts, &PrimitiveState, &RenderDepthAndStencil, &RenderBlend,
                &SS, &RenderGeometry,
            ),
            Or<
                (
                    Changed<SS>, Changed<VertexBufferLayouts>, Changed<PrimitiveState>, Changed<RenderDepthAndStencil>, Changed<RenderBlend>, 
                    Changed<RenderGeometry>, Changed<IndicesBufferDesc>, Changed<AssetResBufferIndices>
                )
            >
        >,
        mut draw_cmd: Commands<GameObject, D>,
        asset_mgr_pipeline: Res<Share<AssetMgr<RenderRes<Pipeline3D>>>>,
        device: Res<RenderDevice>,
    ) {
        let time1 = Instant::now();

        models.iter().for_each(
            |(
                id_obj,
                vb, primitive, depth_stencil, blend,
                shader, rendergeo,
            )| {
                if let Some((key_shader, shader, bindgroups)) = shader.val() {
                    let mut bind_group_layouts = [None, None, None, None];
                    let mut key_bindgroup_layouts = [None, None, None, None];
                    for i in 0..4 {
                        if let Some(val) = &bindgroups.0[i] {
                            bind_group_layouts[i] = Some(val.layout());
                            key_bindgroup_layouts[i] = Some(val.key_layout());
                        }
                    }

                    let key_vertex_layouts = KeyPipelineFromAttributes::new(vb.clone());
                     

                    let targets = RenderTargetState::color_target(blend);
                    let key_state = KeyRenderPipelineState {
                        primitive: primitive.state,
                        target_state: vec![targets[0].clone()],
                        depth_stencil: depth_stencil.0.clone(),
                        multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }
                    };

                    if let Some(pipeline) = KeyPipeline3D::create(key_state, key_shader.clone(), shader.clone(), key_bindgroup_layouts, bind_group_layouts, key_vertex_layouts, &asset_mgr_pipeline, &device) {

                        let draw = DrawObj3D {
                            pipeline: Some(pipeline),
                            bindgroups: bindgroups.groups(),
                            vertices: rendergeo.vertices(),
                            instances: rendergeo.instances(),
                            indices: rendergeo.indices.clone(),
                        };
                        draw_cmd.insert(id_obj.clone(), D::new(Some(Arc::new(draw))));
                    } else {
                        draw_cmd.insert(id_obj.clone(), D::new(None));
                    }
                } else {
                    draw_cmd.insert(id_obj.clone(), D::new(None));
                }
            }
        );

        log::info!("SysPassDrawUpdate: {:?}", Instant::now() - time1);
    }
}

pub type SysPass01DrawUpdate = SysPassDrawUpdate<Pass01Shader, Pass01Draw, SysPass01ShaderUpdate>;
pub type SysPass02DrawUpdate = SysPassDrawUpdate<Pass02Shader, Pass02Draw, SysPass01ShaderUpdate>;
pub type SysPass03DrawUpdate = SysPassDrawUpdate<Pass03Shader, Pass03Draw, SysPass01ShaderUpdate>;
pub type SysPass04DrawUpdate = SysPassDrawUpdate<Pass04Shader, Pass04Draw, SysPass01ShaderUpdate>;
pub type SysPass05DrawUpdate = SysPassDrawUpdate<Pass05Shader, Pass05Draw, SysPass01ShaderUpdate>;
pub type SysPass06DrawUpdate = SysPassDrawUpdate<Pass06Shader, Pass06Draw, SysPass01ShaderUpdate>;
pub type SysPass07DrawUpdate = SysPassDrawUpdate<Pass07Shader, Pass07Draw, SysPass01ShaderUpdate>;
pub type SysPass08DrawUpdate = SysPassDrawUpdate<Pass08Shader, Pass08Draw, SysPass01ShaderUpdate>;

fn collect_draw(
    renderer: &mut Renderer,
    passtag_orders: &PassTagOrders,
    draws: (
        &Pass01Draw, &Pass02Draw, &Pass03Draw, &Pass04Draw,
        &Pass05Draw, &Pass06Draw, &Pass07Draw, &Pass08Draw,
    )
) {
    passtag_orders.0.iter().for_each(|tag| {
        match tag {
            EPassTag::ShadowCast => {
                if let Some(draw) = &draws.0.0 {
                    renderer.draws.list.push(draw.clone());
                }
            },
            EPassTag::Opaque =>  {
                if let Some(draw) = &draws.1.0 {
                    renderer.draws.list.push(draw.clone());
                }
            },
            EPassTag::Sky =>  {
                if let Some(draw) = &draws.2.0 {
                    renderer.draws.list.push(draw.clone());
                }
            },
            EPassTag::Water =>  {
                if let Some(draw) = &draws.3.0 {
                    renderer.draws.list.push(draw.clone());
                }
            },
            EPassTag::Transparent =>  {
                if let Some(draw) = &draws.4.0 {
                    renderer.draws.list.push(draw.clone());
                }
            },
            EPassTag::AlphaTest =>  {
                if let Some(draw) = &draws.5.0 {
                    renderer.draws.list.push(draw.clone());
                }
            },
            EPassTag::OpaqueExtend =>  {
                if let Some(draw) = &draws.6.0 {
                    renderer.draws.list.push(draw.clone());
                }
            },
            EPassTag::TransparentExtend =>  {
                if let Some(draw) = &draws.7.0 {
                    renderer.draws.list.push(draw.clone());
                }
            },
        }
    });
}

pub struct SysRendererDraws;
impl TSystemStageInfo for SysRendererDraws {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key(),
            // SysModelListAfterCullingTick::key(), 在 Command 阶段, 因此屏蔽
            SysPass01DrawUpdate::key(),
            SysPass02DrawUpdate::key(),
            SysPass03DrawUpdate::key(),
            SysPass04DrawUpdate::key(),
            SysPass05DrawUpdate::key(),
            SysPass06DrawUpdate::key(),
            SysPass07DrawUpdate::key(),
            SysPass08DrawUpdate::key(),
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
            (
                &Pass01Draw, &Pass02Draw, &Pass03Draw, &Pass04Draw,
                &Pass05Draw, &Pass06Draw, &Pass07Draw, &Pass08Draw,
            )
        >
    ) {
        let time1 = Instant::now();

        renderers.iter_mut().for_each(|(id_viewer, mut renderer, passtag_orders)| {
            renderer.clear();
            if let Some((list_model, viewport)) = viewers.get(id_viewer.0) {
                if let Some(viewport) = viewport {
                    renderer.draws.viewport = (viewport.x, viewport.y, viewport.w, viewport.h, viewport.mindepth, viewport.maxdepth);
                } else {
                    renderer.draws.viewport = (0., 0., 1., 1., 0., 1.);
                }
                list_model.0.iter().for_each(|id_obj| {
                    if let Some(draws) = models.get(id_obj.clone()) {
                        collect_draw(&mut renderer, passtag_orders, draws);
                    }
                });
            }
        });

        log::info!("SysRendererDraws: {:?}", Instant::now() - time1);
    }
}
