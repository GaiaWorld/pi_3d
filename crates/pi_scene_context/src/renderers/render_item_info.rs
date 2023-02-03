use std::time::Instant;

use pi_assets::mgr::AssetMgr;
use pi_ecs::{sys::system, prelude::{Query, Commands, Res}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, assets::sync_load::{InterfaceAssetSyncCreate}, run_stage::TSystemStageInfo};
use pi_hash::XHashMap;
use pi_render::rhi::{device::RenderDevice};
use pi_scene_math::Vector3;
use pi_share::Share;
use render_data_container::{RenderIndices, TRenderGeometry};
use render_geometry::indices::{IndicesBufferDesc, AssetResBufferIndices};
use render_shader::{
    shader::{ResShader, KeyShader},
    shader_set::{ShaderSetSceneAbout, ShaderSetModelAbout, ShaderSetEffectAbout, ShaderSetModelAboutBindOffset, ShaderSetSceneAboutBindOffset},
    instance_code::EInstanceCode, 
    shader_bind::ShaderBindEffectValue
};

use crate::{
    materials::{
        material::{MaterialID},
        shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta},
    },
    geometry::{AssetKeyVBLayouts, AssetResVBLayouts, geometry::{RenderGeometry, RenderIndicesFrom}}, 
    transforms::transform_node::GlobalTransform,
    viewer::ViewerGlobalPosition,
    bindgroup::{
        RenderBindGroupKey, RenderBindGroupPool,
        uniform_buffer::{SysDynUnifromBufferUpdate, DynUnifromBufferReBindFlag},
    }, skeleton::skeleton::Skeleton
};

use super::{
    ModelListAfterCulling,
    render_object::{RendererID, RenderObjectMetaOpaque, RenderObjectBindGroup, RenderObjectMetaTransparent},
    pipeline::{KeyRenderPipeline, ResRenderPipeline, pipeline_state_key, render_pipeline},
    render_primitive::{PrimitiveState},
    render_depth_and_stencil::{RenderDepthAndStencil},
    render_blend::{RenderBlend},
    render_target_state::RenderTargetState,
    renderer::Renderer,
    render_sort::RenderSortParam,
    render_mode::{RenderMode, ERenderMode}
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyRenderItem {
    pub id_effect_about: ObjectID,
}

#[derive(Debug)]
pub struct RenderItemPipelineInfo {
    pub shader_key: KeyShader,
    pub shader: pi_assets::asset::Handle<ResShader>,
    pub pipeline_key: KeyRenderPipeline,
    pub pipeline: pi_assets::asset::Handle<ResRenderPipeline>,
    pub id_scene_about: RenderBindGroupKey,
    pub id_model_about: RenderBindGroupKey,
    pub id_effect_about: RenderBindGroupKey,
}

#[derive(Debug, Default)]
pub struct RenderItemInfo {
    pub map: XHashMap<KeyRenderItem, RenderItemPipelineInfo>,
}


#[derive(Debug, Default)]
pub struct RenderItemInfoList(pub XHashMap<ObjectID, RenderItemInfo>, pub Vector3);

pub struct RendererItemsReset;
impl TSystemStageInfo for RendererItemsReset {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysDynUnifromBufferUpdate::key()
        ]
    }
}
#[setup]
impl RendererItemsReset {
    #[system]
    pub fn sys(
        viewers: Query<
            GameObject,
            (ObjectID, &ModelListAfterCulling, &RendererID, &ViewerGlobalPosition),
            // Changed<ModelListAfterCulling>
        >,
        mut renderers: Query<
            GameObject,
            (ObjectID, &ShaderSetSceneAbout, &ShaderSetSceneAboutBindOffset, &mut Renderer)
        >,
        mut meshes: Query<
            GameObject,
            (
                ObjectID,
                &MaterialID, &AssetKeyVBLayouts, &AssetResVBLayouts, &EInstanceCode,
                &RenderBlend, &RenderDepthAndStencil, &PrimitiveState, 
                &RenderSortParam, &mut GlobalTransform, &RenderMode, &RenderGeometry, Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>, &ShaderSetModelAbout, &ShaderSetModelAboutBindOffset
            ),
        >,
        materials: Query<
            GameObject,
            (
                &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &ShaderSetEffectAbout, &ShaderBindEffectValue
            ),
        >,
        asset_mgr_shaders: Res<Share<AssetMgr<ResShader>>>,
        asset_mgr_pipelines: Res<Share<AssetMgr<ResRenderPipeline>>>,
        device: Res<RenderDevice>,
        layoutpool: Res<RenderBindGroupPool>,
    ) {
        let time = Instant::now();
        log::trace!("RendererItemsReset: ");
        viewers.iter().for_each(|(view_id, cullinglist, rendererid, camerapos)| {
            log::debug!("RendererItemsReset: 0");
            if let Some((scene_id, scene_set, scene_bindoff, mut renderer)) = renderers.get_mut(rendererid.0.clone()) {

                renderer.clear();
                
                let key_scene = RenderBindGroupKey::SceneAbout(rendererid.0.clone());
                if let Some(bindgroup) = layoutpool.get(&key_scene) {
                    if bindgroup.bind_group.is_some() {
                        renderer.reset(RenderObjectBindGroup {
                            bind_group: key_scene,
                            offsets: scene_bindoff.get(),
                        });
                    } else {
                        log::warn!(">>>>>>>>>>>>>>>>> bind_group SceneAbout No");
                        return;
                    }
                } else {
                    log::warn!(">>>>>>>>>>>>>>>>> bindgroup SceneAbout No");
                    return;
                }

                // log::debug!("RendererItemsReset: 1, {}", cullinglist.0.len());
                cullinglist.0.iter().for_each(|entity| {
                    // log::debug!("RendererItemsReset: 2");

                    if let Some((
                        entity,
                        matid, keyvb, vb, instance, 
                        blend, depth_stencil, primitive,
                        rendersort, mut gtransform, rendermode, rendergeo, indice_desc, indices_res, model_set, model_bindoff
                    )) = meshes.get_mut(entity.clone()) {
                        // log::debug!("RendererItemsReset: 3");

                        if let Some((matkey, matmeta, effect_set, effect_bindoff)) = materials.get(matid.0) {
                            // log::debug!("RendererItemsReset: 4");
                            
                            collect_render_item(
                                &rendererid.0,
                                &entity,
                                &matid.0,
                                (scene_set, &camerapos.0.clone(), &mut renderer),
                                (keyvb, vb, instance, blend, depth_stencil, primitive, rendersort, &mut gtransform, rendermode, rendergeo, indice_desc, indices_res, model_set, model_bindoff),
                                (matkey, matmeta, effect_set, effect_bindoff),
                                &asset_mgr_shaders,
                                &asset_mgr_pipelines,
                                &device,
                                &layoutpool,
                                &layoutpool,
                            );
                        }
                    }
                });
            }
        });
        let time1 = Instant::now();
        log::trace!("RendererItemsReset: {:?}", time1 - time);
    }
}


pub struct RendererItemsModifyByModelChange;
impl TSystemStageInfo for RendererItemsModifyByModelChange {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            RendererItemsReset::key()
        ]
    }
}
#[setup]
impl RendererItemsModifyByModelChange {
    #[system]
    pub fn sys(
        mut viewers: Query<
            GameObject,
            (ObjectID, &ModelListAfterCulling, &RendererID, &ViewerGlobalPosition),
        >,
        mut renderers: Query<
            GameObject,
            (ObjectID, &ShaderSetSceneAbout, &mut Renderer)
        >,
        mut meshes: Query<
            GameObject,
            (
                ObjectID,
                &MaterialID, &AssetKeyVBLayouts, &AssetResVBLayouts, &EInstanceCode,
                &RenderBlend, &RenderDepthAndStencil, &PrimitiveState, 
                &RenderSortParam, &mut GlobalTransform, &RenderMode, &RenderGeometry, Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>, &ShaderSetModelAbout, &ShaderSetModelAboutBindOffset
            ),
            Or<(Changed<MaterialID>, Changed<AssetResVBLayouts>, Changed<RenderBlend>, Changed<RenderDepthAndStencil>, Changed<PrimitiveState>, Changed<ShaderSetModelAbout>, Changed<ShaderSetModelAboutBindOffset>, Changed<RenderGeometry>)>
        >,
        materials: Query<
            GameObject,
            (
                &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &ShaderSetEffectAbout, &ShaderBindEffectValue
            ),
        >,
        asset_mgr_shaders: Res<Share<AssetMgr<ResShader>>>,
        asset_mgr_pipelines: Res<Share<AssetMgr<ResRenderPipeline>>>,
        device: Res<RenderDevice>,
        layoutpool: Res<RenderBindGroupPool>,
    ) {
        log::debug!("RendererItemsModifyByModelChange: ");
        meshes.iter_mut().for_each(|(
            entity,
            matid, keyvb, vb, instance, 
            blend, depth_stencil, primitive,
            rendersort, mut gtransform, rendermode, rendergeo, indice_desc, indices_res, model_set, model_bindoff
        )| {
            viewers.iter().for_each(|(view_id, cullinglist, rendererid, camerapos)| {
                log::debug!("RendererItemsModifyByModelChange: 0");
                if cullinglist.0.contains(&entity) {
                    if let Some((scene_id, scene_set, mut renderer)) = renderers.get_mut(rendererid.0.clone()) {

                        log::debug!("RendererItemsModifyByModelChange: 1");
                        renderer.remove(&entity);
    
                        if let Some((matkey, matmeta, effect_set, effect_bindoff)) = materials.get(matid.0) {
                            
                            log::debug!("RendererItemsModifyByModelChange: 2");
                            collect_render_item(
                                &rendererid.0,
                                &entity,
                                &matid.0,
                                (scene_set, &camerapos.0.clone(), &mut renderer),
                                (keyvb, vb, instance, blend, depth_stencil, primitive, rendersort, &mut gtransform, rendermode, rendergeo, indice_desc, indices_res, model_set, model_bindoff),
                                (matkey, matmeta, effect_set, effect_bindoff),
                                &asset_mgr_shaders,
                                &asset_mgr_pipelines,
                                &device,
                                &layoutpool,
                                &layoutpool,
                            );
                        }
                    }
                }
            });
        });
    }
}

pub struct RendererItemsModifyByMaterialChange;
impl TSystemStageInfo for RendererItemsModifyByMaterialChange {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            RendererItemsReset::key(),
        ]
    }
}
#[setup]
impl RendererItemsModifyByMaterialChange {
    #[system]
    pub fn sys(
        mut viewers: Query<
            GameObject,
            (ObjectID, &ModelListAfterCulling, &RendererID, &ViewerGlobalPosition),
        >,
        mut renderers: Query<
            GameObject,
            (ObjectID, &ShaderSetSceneAbout, &mut Renderer)
        >,
        mut meshes: Query<
            GameObject,
            (
                ObjectID,
                &MaterialID, &AssetKeyVBLayouts, &AssetResVBLayouts, &EInstanceCode,
                &RenderBlend, &RenderDepthAndStencil, &PrimitiveState, 
                &RenderSortParam, &mut GlobalTransform, &RenderMode, &RenderGeometry, Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>, &ShaderSetModelAbout, &ShaderSetModelAboutBindOffset
            ),
        >,
        materials: Query<
            GameObject,
            (
                ObjectID,
                &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &ShaderSetEffectAbout, &ShaderBindEffectValue
            ),
            Changed<ShaderSetEffectAbout>
        >,
        asset_mgr_shaders: Res<Share<AssetMgr<ResShader>>>,
        asset_mgr_pipelines: Res<Share<AssetMgr<ResRenderPipeline>>>,
        device: Res<RenderDevice>,
        layoutpool: Res<RenderBindGroupPool>,
    ) {
        // log::debug!("RendererItemsModifyByMaterialChange: ");
        materials.iter().for_each(|(id_mat, matkey, matmeta, effect_set, effect_bindoff)| {
            viewers.iter().for_each(|(view_id, cullinglist, rendererid, camerapos)| {
                // log::debug!("RendererItemsModifyByMaterialChange: 0");
                meshes.iter_mut().for_each(|(
                    entity,
                    matid, keyvb, vb, instance, 
                    blend, depth_stencil, primitive,
                    rendersort, mut gtransform, rendermode, rendergeo, indice_desc, indices_res, model_set, model_bindoff
                )| {
                    // log::debug!("RendererItemsModifyByMaterialChange: 1");
                    if matid.0 == id_mat {
                        // log::debug!("RendererItemsModifyByMaterialChange: 2");
                        if cullinglist.0.contains(&entity) {
                            if let Some((scene_id, scene_set, mut renderer)) = renderers.get_mut(rendererid.0.clone()) {

                                // log::debug!("RendererItemsModifyByMaterialChange: 3");
                                renderer.remove(&entity);
    
                                collect_render_item(
                                    &rendererid.0,
                                    &entity,
                                    &matid.0,
                                    (scene_set, &camerapos.0.clone(), &mut renderer),
                                    (keyvb, vb, instance, blend, depth_stencil, primitive, rendersort, &mut gtransform, rendermode, rendergeo, indice_desc, indices_res, model_set, model_bindoff),
                                    (matkey, matmeta, effect_set, effect_bindoff),
                                    &asset_mgr_shaders,
                                    &asset_mgr_pipelines,
                                    &device,
                                    &layoutpool,
                                    &layoutpool,
                                );
                            }
                        }
                    }
                });
            });
        });
    }
}

fn collect_render_item(
    id_scene: &ObjectID,
    id_model: &ObjectID,
    id_mat: &ObjectID,
    scene_state: (&ShaderSetSceneAbout, &Vector3, &mut Renderer),
    mut model_state: (&AssetKeyVBLayouts, &AssetResVBLayouts, &EInstanceCode, &RenderBlend, &RenderDepthAndStencil, &PrimitiveState,
        &RenderSortParam, &mut GlobalTransform, &RenderMode, &RenderGeometry, Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>, &ShaderSetModelAbout, &ShaderSetModelAboutBindOffset
    ),
    effect_state: (&AssetKeyShaderEffect, &AssetResShaderEffectMeta, &ShaderSetEffectAbout, &ShaderBindEffectValue),
    // skeleton_state: Option<(&Skeleton)>,
    asset_mgr_shaders: &Share<AssetMgr<ResShader>>,
    asset_mgr_pipelines: &Share<AssetMgr<ResRenderPipeline>>,
    device: &RenderDevice,
    layoutpool: &RenderBindGroupPool,
    bindgrouppool: &RenderBindGroupPool,
) {
    let (
        keyvb, vb, instance, blend, depth_stencil, primitive,
        rendersort, gtransform, rendermode, rendergeo, indesc, inres,
        model_set, model_bindoff) = model_state;
    let (matkey, matmeta, effect_set, effect_bindoff) = effect_state;
    let (scene_set, camerapos, mut renderer) = scene_state;

    let vertex_layouts_key = keyvb.0.clone();

    let shader_key = KeyShader { shader: matkey.0.clone(), defines_key: 0, vs_layouts: vertex_layouts_key, scene_about: scene_set.clone(), model_about: model_set.clone() };
    
    let shader = if let Some(shader) = asset_mgr_shaders.get(&shader_key) {
        shader
    } else {
        asset_mgr_shaders.create_asset(shader_key.clone(), ResShader::build(&device, &matkey.0, &matmeta.0, &vb.0, instance, scene_set, model_set, effect_set))
    };

    let targets = RenderTargetState::color_target(blend);
    let primitive = primitive.state;
    let depth_stencil = depth_stencil.state();

    let pipeline_key = KeyRenderPipeline { 
        shader_key: shader_key.clone(),
        state_key: pipeline_state_key(
            targets.as_slice(),
            &primitive,
            &depth_stencil,
            0, 8
        )
    };

    let id_scene_about = RenderBindGroupKey::SceneAbout(id_scene.clone());
    let id_model_about = RenderBindGroupKey::ModelAbout(id_model.clone());
    let id_effect_about = RenderBindGroupKey::EffectAbout(id_mat.clone());

    let pipeline = if let Some(pipeline) = asset_mgr_pipelines.get(&pipeline_key) {
        pipeline
    } else {
        // log::debug!("Pipeline: {:?}, Shader: {:?}", pipeline_key, shader_key);

        let mut bind_group_layouts = vec![];

        let layout = layoutpool.get_layout(&id_scene_about);
        bind_group_layouts.push(layout.unwrap().value());

        let layout = layoutpool.get_layout(&id_model_about);
        bind_group_layouts.push(layout.unwrap().value());

        if let Some(layout) = layoutpool.get_layout(&id_effect_about) {
            bind_group_layouts.push(layout.value());
        }

        let pipeline = render_pipeline::<ResShader>(&shader, &device, targets.as_slice(), depth_stencil, primitive, &vb.0.layouts(), &bind_group_layouts);
        asset_mgr_pipelines.create_asset(pipeline_key.clone(), pipeline)
    };

    let mut indices = None;
    if let Some(indesc) = indesc {
        if let Some(inres) = inres {
            indices = Some(RenderIndices::create((indesc, inres)));
        } else {
            log::warn!(">>>>>>>>>>>>>>>>> RenderIndices No");
            return;
        }
    }

    // log::debug!("opaque draw obj >>>>>>>>>>>>>>> ");
    let mut bind_groups = vec![];

    let key_model = RenderBindGroupKey::ModelAbout(id_model.clone());
    if let Some(bindgroup) = bindgrouppool.get(&key_model) {
        if bindgroup.bind_group.is_some() {
            bind_groups.push(
                RenderObjectBindGroup {
                    bind_group: key_model,
                    offsets: model_bindoff.get(),
                }
            );
        } else {
            log::warn!(">>>>>>>>>>>>>>>>> bind_group ModelAbout No");
            return;
        }
    } else {
        log::warn!(">>>>>>>>>>>>>>>>> bindgroup ModelAbout No");
        return;
    }

    let key_effect = RenderBindGroupKey::EffectAbout(id_mat.clone());
    if let Some(bindgroup) = bindgrouppool.get(&key_effect) {
        if bindgroup.bind_group.is_some() {
            bind_groups.push(
                RenderObjectBindGroup {
                    bind_group: key_effect,
                    offsets: effect_bindoff.bind_offset_info(),
                }
            );
        } else {
            log::warn!(">>>>>>>>>>>>>>>>> bind_group EffectAbout No");
            return;
        }
    } else if effect_bindoff.bind_offset_info().len() > 0 {
        log::warn!(">>>>>>>>>>>>>>>>> effect_bindoff No");
        return;
    }

    let view_distance = camerapos.metric_distance(gtransform.position());

    // log::debug!(">>>>>>>>>>>>>>>>> rendermode {:?}", rendermode.0);
    match rendermode.0 {
        ERenderMode::AlphaTest => {

        },
        ERenderMode::Opaque => {
            // log::debug!(">>>>>>>>>>>>>>>>> Opaque");
            let meta = RenderObjectMetaOpaque {
                bind_groups,
                pipeline: pipeline,
                indices,
                vertices: rendergeo.vertices(),
                instances: rendergeo.instances(),
                render_sort: *rendersort,
                view_distance,
            };
            renderer.opaque_draws.push(id_model, meta);
        },
        ERenderMode::Skybox => todo!(),
        ERenderMode::Transparent => {
            // log::debug!(">>>>>>>>>>>>>>>>> Transparent");
            let meta = RenderObjectMetaTransparent {
                bind_groups,
                pipeline: pipeline,
                indices,
                vertices: rendergeo.vertices(),
                instances: rendergeo.instances(),
                render_sort: *rendersort,
            };
            renderer.transparent_draws.push(id_model, meta);
        },
    };
}