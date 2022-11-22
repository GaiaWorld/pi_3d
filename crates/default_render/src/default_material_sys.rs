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
    resources::{SingleRenderObjectPipelinePool, RenderDynUniformBuffer},
    cameras::camera::{CameraRenderData, CameraGlobalPosition},
    materials::{material::MaterialID, bind_group::{RenderBindGroup, RenderBindGroupPool, RenderBindGroupKey}},
    meshes::model::BuildinModelBind,
    vertex_data::{indices::{IDAttributeIndices, AttributeIndices}, position::{IDAttributePosition, AttributePosition}, normal::{IDAttributeNormal, AttributeNormal}},
    main_camera_render::{MainCameraRenderer, bind_group::IDMainCameraRenderBindGroup},
    layer_mask::LayerMask
};

use crate::default_material::DefaultMaterialPropertype;

use super::{shader::DefaultShader, bind_group::{IDDefaultMaterialBindGroup}, pipeline::DefaultMaterialPipeline};


pub struct DefaultModelUniformUpdate;
#[setup]
impl DefaultModelUniformUpdate {
    #[system]
    pub fn tick(
        meshes: Query<GameObject, (&GlobalTransform, &MaterialID, &BuildinModelBind), With<DirtyGlobalTransform>>,
        materials: Query<GameObject, &DefaultMaterialPropertype>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        //  println!("DefaultMaterial Uniform TickUpdate");
        meshes.iter().for_each(|(transform, mat_id, model)| {
            match materials.get(mat_id.0) {
                Some(_) => {
                    //  println!("DefaultMaterial >>>>>>>>>>>> ");
                    dynbuffer.as_mut().set_uniform::<GlobalTransform>(&model.bind_offset, transform);
                },
                None => {
                    
                }
            }
        });
    }
}

pub struct DefaultMaterialUniformUpdate;
#[setup]
impl DefaultMaterialUniformUpdate {
    #[system]
    pub fn tick(
        materials: Query<GameObject, &DefaultMaterialPropertype, Changed<DefaultMaterialPropertype>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        //  println!("DefaultMaterial Uniform TickUpdate");
        materials.iter().for_each(|(material)| {
            dynbuffer.as_mut().set_uniform::<DefaultMaterialPropertype>(&material.bind_offset, &material);
        });
    }
}

pub struct SysDefaultMaterialPipelineKey;
#[setup]
impl SysDefaultMaterialPipelineKey {
    #[system]
    pub fn tick(
        mut items: Query<GameObject, (&MaterialID, &RenderBlend, &RenderDepthAndStencil, &PrimitiveState, Write<PipelineKey>), Or<(Changed<MaterialID>, Changed<RenderBlend>, Changed<RenderDepthAndStencil>, Changed<PrimitiveState>)>>,
        materials: Query<GameObject, &DefaultMaterialPropertype>,
        device: Res<RenderDevice>,
        shader: Res<DefaultShader>,
        mut pipelines: ResMut<DefaultMaterialPipeline>,
        mut pipeline_pool: ResMut<SingleRenderObjectPipelinePool>,
    ) {
        items.iter_mut().for_each(|(matid, blend, depth_stencil, primitive, mut pipeline )| {
            if let Some(_) = materials.get(matid.0) {
                let key = pipelines.build(
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

pub struct DefaultMaterialFilter;
#[setup]
impl DefaultMaterialFilter {
    #[system]
    pub fn tick(
        query_camera: Query<GameObject, (&RenderObjectID, &SceneID, &LayerMask, &CameraGlobalPosition)>,
        mut query_renderers: Query<GameObject, &mut MainCameraRenderer>,
        meshes: Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &PipelineKey, &GlobalTransform, &IDAttributePosition, &IDAttributeNormal, &IDAttributeIndices, &BuildinModelBind, &RenderMode)>,
        materials: Query<GameObject, &DefaultMaterialPropertype>,
        bind_groups: Res<RenderBindGroupPool>,
        positions: Query<GameObject, &AttributePosition>,
        normals: Query<GameObject, &AttributeNormal>,
        indices: Query<GameObject, &AttributeIndices>,
        id_bind_group_default: Res<IDDefaultMaterialBindGroup>,
    ) {
        
        let time = Instant::now();

        let id_bind_group_default = id_bind_group_default.0;
        //  println!("DefaultMaterial Filter");
        query_camera.iter().for_each(|(renderid, sceneid, layermask, camerapos)| {
            //  println!("Camera >>>>>>>>>>>>>>>");
    
            if bind_groups.get(id_bind_group_default).unwrap().bind_group.is_some() {
                // println!("Main Camera >>>>>>>>>>>>>>>");

                match query_renderers.get_mut(renderid.0) {
                    Some(mut renderer) => {
                        // println!("opaque List >>>>>>>>>>>>>>> {:?}", renderid.0);
                        if renderer.ready {
                            collect_opaque_normal_depth(
                                sceneid.0,
                                layermask,
                                &materials,
                                &meshes,
                                &camerapos.0,
                                &positions, &normals, &indices,
                                &mut renderer,
                                id_bind_group_default
                            );
                        }
                    },
                    None => todo!(),
                }
            }
        });
        // let _use_time = Instant::now() - pre_frame_time;
        let time1 = Instant::now();
        println!("DefaultMaterialFilter: {:?}", time1 - time);
    }
}

fn collect_opaque_normal_depth(
    camera_sceneid: ObjectID,
    layermask: &LayerMask,
    materials: &Query<GameObject, &DefaultMaterialPropertype>,
    query: &Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &PipelineKey, &GlobalTransform, &IDAttributePosition, &IDAttributeNormal, &IDAttributeIndices, &BuildinModelBind, &RenderMode)>,
    camerapos: &Vector3,
    positions: &Query<GameObject, &AttributePosition>,
    normals: &Query<GameObject, &AttributeNormal>,
    indices: &Query<GameObject, &AttributeIndices>,
    renderer: &mut MainCameraRenderer,
    id_bind_group_default: RenderBindGroupKey,
) {
    query.iter().for_each(|(matid, sceneid, layer, rendersort, pipeline, globaltransform, position, normal, indice, model, rendermode)| {

        // println!("opaque draw obj >>>>>>>>>>>>>>> {:?}, {:?}, {:?}, {:?}", sceneid, item.1.0, layermask, item.5);
        if camera_sceneid == sceneid.0 && layermask.include(&layer) {
            match materials.get(matid.0) {
                Some(mat) => {
                    match (positions.get(position.0), normals.get(normal.0), indices.get(indice.0)) {
                        (Some(position), Some(normal), Some(indices)) => {
                            // if list.len() >= 1 { return; }
                            let view_distance = camerapos.metric_distance(&globaltransform.position);

                            let mut bind_groups = vec![];
                            bind_groups.push(RenderObjectBindGroup {
                                bind_group: id_bind_group_default,
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