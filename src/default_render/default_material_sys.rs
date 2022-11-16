use std::time::Instant;

use pi_ecs::{prelude::{Query, ResMut, Res}, query::{With, Write, WithOut, Changed, Or}};
use pi_ecs_macros::setup;
use pi_render::{rhi::{device::RenderDevice, RenderQueue}};
use pi_scene_math::{Vector3};
use render_geometry::geometry::VertexAttributeMeta;

use crate::{
    object::{GameObject, ObjectID},
    transforms::{transform_node::{GlobalTransform}, dirty::DirtyGlobalTransform},
    renderers::{
        render_object::{RenderObjectID, RenderObjectMetaOpaque, RenderObjectVertice, RenderObjectIndices, RenderObjectBindGroup},
        pipeline::PipelineKey,
        render_blend::RenderBlend,
        render_depth_and_stencil::RenderDepthAndStencil,
        render_primitive::PrimitiveState,
        render_sort::RenderSortParam,
        render_target_state::RenderTargetState,
    },
    flags::{SceneID01, SceneCameraID01, SceneID},
    environment::fog::SceneFog, default_render::default_material::{DefaultMaterialPropertype},
    resources::{SingleRenderObjectPipelinePool, RenderDynUniformBuffer},
    cameras::camera::{CameraRenderData, CameraGlobalPosition},
    materials::{material::MaterialID, bind_group::RenderBindGroup},
    meshes::model::BuildinModelBind,
    vertex_data::{indices::{IDAttributeIndices, AttributeIndices}, position::{IDAttributePosition, AttributePosition}, normal::{IDAttributeNormal, AttributeNormal}},
    main_camera_render::{MainCameraRenderer, bind_group::IDMainCameraRenderBindGroup},
    layer_mask::LayerMask
};

use super::{shader::DefaultShader, bind_group::{IDDefaultMaterialBindGroup}, pipeline::DefaultMaterialPipeline, dirty::DirtyDefaultMaterialPropertype};


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
                Some(material) => {
                    //  println!("DefaultMaterial >>>>>>>>>>>> ");
                    dynbuffer.as_mut().set_uniform::<GlobalTransform>(&model.bind_offset, transform);
                },
                None => todo!(),
            }
        });
    }
}

pub struct DefaultMaterialUniformUpdate;
#[setup]
impl DefaultMaterialUniformUpdate {
    #[system]
    pub fn tick(
        materials: Query<GameObject, &DefaultMaterialPropertype, With<DirtyDefaultMaterialPropertype>>,
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
            let key = pipelines.build(
                &device,
                &shader,
                RenderTargetState::color_target(blend).as_slice(),
                depth_stencil.state(),
                primitive.state,
                &mut pipeline_pool,
            );
            
            let key = PipelineKey { id: key };

            pipeline.insert_no_notify(key);
        });
    }
}

pub struct DefaultMaterialFilter;
#[setup]
impl DefaultMaterialFilter {
    #[system]
    pub fn tick(
        query_camera: Query<GameObject, (&RenderObjectID, &SceneID, &LayerMask, &CameraRenderData, &CameraGlobalPosition)>,
        mut query_renderers: Query<GameObject, &mut MainCameraRenderer>,
        meshes: Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &PipelineKey, &GlobalTransform, &IDAttributePosition, &IDAttributeNormal, &IDAttributeIndices, &BuildinModelBind)>,
        materials: Query<GameObject, &DefaultMaterialPropertype>,
        bind_groups: Query<GameObject, &RenderBindGroup>,
        positions: Query<GameObject, &AttributePosition>,
        normals: Query<GameObject, &AttributeNormal>,
        indices: Query<GameObject, &AttributeIndices>,
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
        id_bind_group_main_camera: Res<IDMainCameraRenderBindGroup>,
        id_bind_group_default: Res<IDDefaultMaterialBindGroup>,
    ) {
        
        let time = Instant::now();

        let id_bind_group_main_camera = id_bind_group_main_camera.0;
        let id_bind_group_default = id_bind_group_default.0;
        //  println!("DefaultMaterial Filter");
        query_camera.iter().for_each(|(renderid, sceneid, layermask, cameradata, camerapos)| {
            //  println!("Camera >>>>>>>>>>>>>>>");
            if bind_groups.get(id_bind_group_main_camera).unwrap().bind_group.is_some()
            && bind_groups.get(id_bind_group_default).unwrap().bind_group.is_some() {
                //  println!("Main Camera >>>>>>>>>>>>>>>");
                let camera_bind_group = RenderObjectBindGroup {
                    bind_group: id_bind_group_main_camera,
                    offsets: vec![
                        *cameradata.bind_offset,
                        0, 0, 0,
                    ],
                };

                match query_renderers.get_mut(renderid.0) {
                    Some(mut renderer) => {
                        // println!("opaque List >>>>>>>>>>>>>>> {:?}", renderid.0);

                        let renderlist = &mut renderer.opaque_draws;
                        renderlist.draws.clear();
                        renderlist.bind_groups.clear();
                        renderlist.bind_groups.push(camera_bind_group);
                        collect_opaque_normal_depth(
                            sceneid.0,
                            layermask,
                            &materials,
                            &meshes,
                            &camerapos.0,
                            &positions, &normals, &indices,
                            &mut renderlist.draws,
                            id_bind_group_default
                        );
                    },
                    None => todo!(),
                }
            }
        });
        // let _use_time = Instant::now() - pre_frame_time;
        let time1 = Instant::now();
        // println!("DefaultMaterialFilter: {:?}", time1 - time);
    }
}

fn collect_opaque_normal_depth(
    camera_sceneid: ObjectID,
    layermask: &LayerMask,
    materials: &Query<GameObject, &DefaultMaterialPropertype>,
    query: &Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &PipelineKey, &GlobalTransform, &IDAttributePosition, &IDAttributeNormal, &IDAttributeIndices, &BuildinModelBind)>,
    camerapos: &Vector3,
    positions: &Query<GameObject, &AttributePosition>,
    normals: &Query<GameObject, &AttributeNormal>,
    indices: &Query<GameObject, &AttributeIndices>,
    list: &mut Vec<RenderObjectMetaOpaque>,
    id_bind_group_default: ObjectID,
) {
    query.iter().for_each(|(matid, sceneid, layer, rendersort, pipeline, globaltransform, position, normal, indice, model)| {

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

                            // draw_write.insert_no_notify(meta);

                            // println!("{:?}", meta);
                            list.push(meta);
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