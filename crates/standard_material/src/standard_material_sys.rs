use std::time::Instant;

use pi_ecs::{prelude::{Query, ResMut, Res}, query::{With, Write, Changed, Or}};
use pi_ecs_macros::setup;
use pi_render::{rhi::{device::RenderDevice, RenderQueue}};
use pi_scene_math::{Vector3};
use render_geometry::geometry::VertexAttributeMeta;

use pi_scene_context::{
    object::{GameObject, ObjectID},
    transforms::{transform_node::{GlobalTransform}, dirty::DirtyGlobalTransform},
    renderers::{
        render_object::{RenderObjectID, RenderObjectMetaOpaque, RenderObjectVertice, RenderObjectIndices, RenderObjectBindGroup, RenderObjectMetaTransparent},
        pipeline::PipelineKey,
        render_blend::RenderBlend,
        render_depth_and_stencil::RenderDepthAndStencil,
        render_primitive::PrimitiveState,
        render_sort::RenderSortParam,
        render_target_state::RenderTargetState, render_mode::{RenderMode, ERenderMode},
    },
    flags::{SceneID},
    resources::{SingleRenderObjectPipelinePool, render_resource::uniform_buffer::RenderDynUniformBuffer},
    cameras::camera::{CameraRenderData, CameraGlobalPosition},
    materials::{material::MaterialID, bind_group::RenderBindGroup},
    meshes::model::BuildinModelBind,
    vertex_data::{indices::{IDAttributeIndices, AttributeIndices}, position::{IDAttributePosition, AttributePosition}, normal::{IDAttributeNormal, AttributeNormal}},
    main_camera_render::{MainCameraRenderer, bind_group::IDMainCameraRenderBindGroup},
    layer_mask::LayerMask, texture::{texture2d::{scale_offset::Texture2DScaleOffset, Texture2D}, texture_sampler::{TextureSamplerDesc, PluginTextureSampler}}
};
use render_resource::sampler::SamplerPool;

use crate::{standard_material::{StandardMaterialPropertype}, bind_group::{StandardMaterialTextureBindGroup, SingleStandardBindGroupList}, shader::StandardShaderPool, define::StandardMaterialDefines};

use super::{pipeline::StandardMaterialPipeline};

pub struct SysStandardShaderEffectUpdate;
impl SysStandardShaderEffectUpdate {
    pub fn sys(
        shaders: Res<_>,
    ) {

    }
}

pub struct DefaultModelUniformUpdate;
#[setup]
impl DefaultModelUniformUpdate {
    #[system]
    pub fn tick(
        meshes: Query<GameObject, (&GlobalTransform, &MaterialID, &BuildinModelBind), With<DirtyGlobalTransform>>,
        materials: Query<GameObject, &StandardMaterialPropertype>,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
    ) {
        //  log::debug!("DefaultMaterial Uniform TickUpdate");
        meshes.iter().for_each(|(transform, mat_id, model)| {
            match materials.get(mat_id.0) {
                Some(_) => {
                    //  log::debug!("DefaultMaterial >>>>>>>>>>>> ");
                    dynbuffer.as_mut().set_uniform::<GlobalTransform>(&model.bind_offset, transform);
                },
                None => {
                    
                }
            }
        });
    }
}

pub struct StandardMaterialUniformUpdate;
#[setup]
impl StandardMaterialUniformUpdate {
    #[system]
    pub fn tick(
        materials: Query<GameObject, &StandardMaterialPropertype, Changed<StandardMaterialPropertype>>,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
    ) {
        //  log::debug!("DefaultMaterial Uniform TickUpdate");
        materials.iter().for_each(|(material)| {
            dynbuffer.as_mut().set_uniform::<StandardMaterialPropertype>(&material.bind_offset, &material);
        });
    }
}

pub struct SysStandardMaterialPipelineKey;
#[setup]
impl SysStandardMaterialPipelineKey {
    #[system]
    pub fn tick(
        mut items: Query<GameObject, (&MaterialID, &RenderBlend, &RenderDepthAndStencil, &PrimitiveState, Write<PipelineKey>), Or<(Changed<MaterialID>, Changed<RenderBlend>, Changed<RenderDepthAndStencil>, Changed<PrimitiveState>)>>,
        materials: Query<GameObject, (&StandardMaterialDefines), With<StandardMaterialPropertype>>,
        device: Res<RenderDevice>,
        shaderpool: ResMut<StandardShaderPool>,
        mut pipelines: ResMut<StandardMaterialPipeline>,
        mut pipeline_pool: ResMut<SingleRenderObjectPipelinePool>,
    ) {
        items.iter_mut().for_each(|(matid, blend, depth_stencil, primitive, mut pipeline )| {
            if let Some((defines)) = materials.get(matid.0) {
                let shader = shaderpool.get(defines, &device);
                let definemode = defines.mode();

                let key = pipelines.build(
                    definemode,
                    &device,
                    &shader,
                    RenderTargetState::color_target(blend).as_slice(),
                    depth_stencil.state(),
                    primitive.state,
                    &mut pipeline_pool,
                );
                
                let key = PipelineKey { id: key };
    
                pipeline.write(key);
            }
        });
    }
}

pub struct StandardMaterialFilter;
#[setup]
impl StandardMaterialFilter {
    #[system]
    pub fn tick(
        query_camera: Query<GameObject, (&RenderObjectID, &SceneID, &LayerMask, &CameraGlobalPosition)>,
        mut query_renderers: Query<GameObject, &mut MainCameraRenderer>,
        meshes: Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &PipelineKey, &GlobalTransform, &IDAttributePosition, &IDAttributeNormal, &IDAttributeIndices, &BuildinModelBind, &RenderMode)>,
        materials: Query<GameObject, (&StandardMaterialDefines, &StandardMaterialPropertype)>,
        bind_groups: Res<SingleStandardBindGroupList>,
        positions: Query<GameObject, &AttributePosition>,
        normals: Query<GameObject, &AttributeNormal>,
        indices: Query<GameObject, &AttributeIndices>,
    ) {
        
        let time = Instant::now();

        query_camera.iter().for_each(|(renderid, sceneid, layermask, camerapos)| {
            //  log::debug!("Camera >>>>>>>>>>>>>>>");
    
            if bind_groups(id_bind_group_standard).unwrap().bind_group.is_some() {
                // log::debug!("Main Camera >>>>>>>>>>>>>>>");

                match query_renderers.get_mut(renderid.0) {
                    Some(mut renderer) => {
                        // log::debug!("opaque List >>>>>>>>>>>>>>> {:?}", renderid.0);
                        if renderer.ready {
                            collect_opaque_normal_depth(
                                sceneid.0,
                                layermask,
                                &materials,
                                &meshes,
                                &camerapos.0,
                                &positions, &normals, &indices,
                                &mut renderer,
                                id_bind_group_standard
                            );
                        }
                    },
                    None => todo!(),
                }
            }
        });
        // let _use_time = Instant::now() - pre_frame_time;
        let time1 = Instant::now();
        log::debug!("DefaultMaterialFilter: {:?}", time1 - time);
    }
}

fn collect_opaque_normal_depth(
    camera_sceneid: ObjectID,
    layermask: &LayerMask,
    materials: &Query<GameObject, &StandardMaterialPropertype>,
    query: &Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &PipelineKey, &GlobalTransform, &IDAttributePosition, &IDAttributeNormal, &IDAttributeIndices, &BuildinModelBind, &RenderMode)>,
    camerapos: &Vector3,
    positions: &Query<GameObject, &AttributePosition>,
    normals: &Query<GameObject, &AttributeNormal>,
    indices: &Query<GameObject, &AttributeIndices>,
    renderer: &mut MainCameraRenderer,
    id_bind_group_standard: ObjectID,
) {
    query.iter().for_each(|(matid, sceneid, layer, rendersort, pipeline, globaltransform, position, normal, indice, model, rendermode)| {

        // log::debug!("opaque draw obj >>>>>>>>>>>>>>> {:?}, {:?}, {:?}, {:?}", sceneid, item.1.0, layermask, item.5);
        if camera_sceneid == sceneid.0 && layermask.include(&layer) {
            match materials.get(matid.0) {
                Some(mat) => {
                    match (positions.get(position.0), normals.get(normal.0), indices.get(indice.0)) {
                        (Some(position), Some(normal), Some(indices)) => {
                            // if list.len() >= 1 { return; }
                            let view_distance = camerapos.metric_distance(&globaltransform.position);

                            let mut bind_groups = vec![];
                            bind_groups.push(RenderObjectBindGroup {
                                bind_group: id_bind_group_standard,
                                offsets: vec![
                                    *model.bind_offset,
                                    *mat.bind_offset
                                ],
                            });

                            let positions = RenderObjectVertice {
                                slot: AttributePosition::SLOT,
                                gbid: position.meta.buffer_id,
                                start: position.meta.start,
                                end: position.meta.end,
                                count: (position.meta.end - position.meta.start) / position.meta.data_bytes_size
                            };
                            let indices = Some(
                                RenderObjectIndices {
                                    slot: AttributePosition::SLOT,
                                    gbid: indices.meta.buffer_id,
                                    start: indices.meta.start,
                                    end: indices.meta.end,
                                    count: (indices.meta.end - indices.meta.start) / indices.meta.data_bytes_size,
                                    format: indices.format,
                                }
                            );
                            let mut vertices = vec![];
                            let normal = RenderObjectVertice {
                                slot: AttributeNormal::SLOT,
                                gbid: normal.meta.buffer_id,
                                start: normal.meta.start,
                                end: normal.meta.end,
                                count: (normal.meta.end - normal.meta.start) / normal.meta.data_bytes_size
                            };

                            vertices.push(normal);
                            let mut instances = vec![];
                            match rendermode.0 {
                                ERenderMode::AlphaTest => {

                                },
                                ERenderMode::Opaque => {
                                    let mut meta = RenderObjectMetaOpaque {
                                        bind_groups,
                                        pipeline: *pipeline,
                                        positions,
                                        indices,
                                        vertices,
                                        instances,
                                        render_sort: *rendersort,
                                        view_distance,
                                    };
                                    renderer.opaque_draws.draws.push(meta);
                                },
                                ERenderMode::Skybox => todo!(),
                                ERenderMode::Transparent => {
                                    let mut meta = RenderObjectMetaTransparent {
                                        bind_groups,
                                        pipeline: *pipeline,
                                        positions,
                                        indices,
                                        vertices,
                                        instances,
                                        render_sort: *rendersort,
                                    };
                                    renderer.transparent_draws.draws.push(meta);
                                },
                            };
                        },
                        (_, _, _) => {

                        }
                    }
                },
                None => {

                }
            }
        }
    });
}