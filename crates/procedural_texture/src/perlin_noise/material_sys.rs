use pi_ecs::prelude::{Query, Res, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_scene_math::Vector3;
use render_geometry::geometry::VertexBufferMeta;

use pi_scene_context::{
    cameras::camera::{CameraRenderData, CameraGlobalPosition},
    flags::{SceneID},
    layer_mask::LayerMask,
    main_camera_render::{bind_group::IDMainCameraRenderBindGroup, MainCameraRenderer},
    materials::{bind_group::{RenderBindGroup, RenderBindGroupPool, RenderBindGroupKey}, material::MaterialID},
    meshes::model::BuildinModelBind,
    object::{GameObject, ObjectID},
    renderers::{render_object::{RenderObjectID, RenderObjectBindGroup, RenderObjectMetaOpaque, RenderObjectVertice, RenderObjectIndices}, pipeline::PipelineKey, render_blend::RenderBlend, render_depth_and_stencil::RenderDepthAndStencil, render_primitive::PrimitiveState, render_sort::RenderSortParam, render_target_state::RenderTargetState},
    resources::{RenderDynUniformBuffer, SingleRenderObjectPipelinePool},
    transforms::transform_node::GlobalTransform,
    vertex_data::{
        indices::{Indices, IDAttributeIndices},
        normal::{IDBufferNormal, BufferNormal},
        position::{BufferPosition, IDBufferPosition},
    },
};

use super::{material::PerlinNoiseMaterialPropertype, shader::PerlinNoiseShader, bind_group::IDPerlinNoiseMaterialBindGroup, pipeline::PerlinNoiseMaterialPipeline};

pub struct PerlinNoiseMaterialUniformUpdate;
#[setup]
impl PerlinNoiseMaterialUniformUpdate {
    #[system]
    pub fn tick(
        neshes: Query<GameObject, (&GlobalTransform, &MaterialID, &BuildinModelBind)>,
        materials: Query<GameObject, &PerlinNoiseMaterialPropertype>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        println!("PerlinNoiseMaterial Uniform TickUpdate");
        neshes
            .iter()
            .for_each(|(transform, mat_id, model)| match materials.get(mat_id.0) {
                Some(material) => {
                    println!("PerlinNoiseMaterial >>>>>>>>>>>> ");
                    dynbuffer
                        .as_mut()
                        .set_uniform::<GlobalTransform>(&model.bind_offset, transform);
                    dynbuffer
                        .as_mut()
                        .set_uniform::<PerlinNoiseMaterialPropertype>(&material.bind_offset, &material);
                }
                None => {

                },
            });
    }
}

pub struct PerlinNoiseMaterialFilter;
#[setup]
impl PerlinNoiseMaterialFilter {
    #[system]
    pub fn tick(
        query_camera: Query<GameObject, (&RenderObjectID, &SceneID, &LayerMask, &CameraGlobalPosition)>,
        mut query_renderers: Query<GameObject, &mut MainCameraRenderer>,
        meshes: Query<
            GameObject,
            (
                &MaterialID,
                &SceneID,
                &LayerMask,
                &RenderSortParam,
                &RenderBlend,
                &PrimitiveState,
                &RenderDepthAndStencil,
                &GlobalTransform,
                &IDBufferPosition,
                &IDBufferNormal,
                &IDAttributeIndices,
                &BuildinModelBind,
            ),
        >,
        materials: Query<GameObject, &PerlinNoiseMaterialPropertype>,
        bind_groups: Res<RenderBindGroupPool>,
        positions: Query<GameObject, &BufferPosition>,
        normals: Query<GameObject, &BufferNormal>,
        indices: Query<GameObject, &Indices>,
        device: Res<RenderDevice>,
        shader: Res<PerlinNoiseShader>,
        id_bind_group_default: Res<IDPerlinNoiseMaterialBindGroup>,
        mut pipelines: ResMut<PerlinNoiseMaterialPipeline>,
        mut pipeline_pool: ResMut<SingleRenderObjectPipelinePool>,
    ) {
        let id_bind_group_default = &id_bind_group_default.0;
        println!("PerlinNoiseMaterial Filter");
        query_camera
            .iter()
            .for_each(|(renderid, sceneid, layermask, global_position)| {
                println!("Camera >>>>>>>>>>>>>>>");
                if bind_groups
                        .get(id_bind_group_default)
                        .unwrap()
                        .bind_group
                        .is_some()
                {
                    match query_renderers.get_mut(renderid.0) {
                        Some(mut renderer) => {
                            // println!("opaque List >>>>>>>>>>>>>>> {:?}", renderid.0);

                            if renderer.ready {
                                let renderlist = &mut renderer.opaque_draws;
                                collect_opaque_normal_depth(
                                    sceneid.0,
                                    layermask,
                                    &materials,
                                    &meshes,
                                    &global_position.0,
                                    &positions,
                                    &normals,
                                    &indices,
                                    &mut pipelines,
                                    &device,
                                    &shader,
                                    &mut pipeline_pool,
                                    &mut renderlist.draws,
                                    id_bind_group_default,
                                );
                            }
                        }
                        None => todo!(),
                    }
                }
            });
    }
}

fn collect_opaque_normal_depth(
    sceneid: ObjectID,
    layermask: &LayerMask,
    materials: &Query<GameObject, &PerlinNoiseMaterialPropertype>,
    query: &Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &RenderBlend, &PrimitiveState, &RenderDepthAndStencil, &GlobalTransform, &IDBufferPosition, &IDBufferNormal, &IDAttributeIndices, &BuildinModelBind)>,
    camerapos: &Vector3,
    positions: &Query<GameObject, &BufferPosition>,
    normals: &Query<GameObject, &BufferNormal>,
    indices: &Query<GameObject, &Indices>,
    pipelines: &mut PerlinNoiseMaterialPipeline,
    device: & RenderDevice,
    shader: & PerlinNoiseShader,
    pipeline_pool: &mut SingleRenderObjectPipelinePool,
    list: &mut Vec<RenderObjectMetaOpaque>,
    id_bind_group_default: &RenderBindGroupKey,
) {
    query.iter().for_each(|item| {
        let rendersort = item.3;
        let blend = item.4;
        let primit = item.5;
        let depth_stencil = item.6;
        let globaltransform = item.7;
        let model = item.11;
        if sceneid == item.1.0 && layermask.include(&item.2) {
            match materials.get(item.0.0) {
                Some(mat) => {
                    println!("opaque draw obj >>>>>>>>>>>>>>> {:?}, {:?}, {:?}, {:?}", sceneid, item.1.0, layermask, item.5);
                    match (positions.get(item.8.0), normals.get(item.9.0), indices.get(item.10.0)) {
                        (Some(position), Some(normal), Some(indices)) => {
                            let view_distance = camerapos.metric_distance(&globaltransform.position);
                            let pipeline = pipelines.build(
                                device,
                                shader,
                                RenderTargetState::color_target(blend).as_slice(),
                                depth_stencil.state(),
                                primit.state,
                                pipeline_pool,
                            );
                            
                            let pipeline = PipelineKey { id: pipeline };
                            let mut bind_groups = vec![];
                            bind_groups.push(RenderObjectBindGroup {
                                bind_group: id_bind_group_default.clone(),
                                offsets: vec![
                                    *model.bind_offset,
                                    *mat.bind_offset
                                ],
                            });

                            let positions = RenderObjectVertice {
                                slot: BufferPosition::SLOT,
                                gbid: position.meta.buffer_id,
                                start: position.meta.start,
                                end: position.meta.end,
                                count: (position.meta.end - position.meta.start) / position.meta.data_bytes_size
                            };
                            let indices = Some(
                                RenderObjectIndices {
                                    slot: BufferPosition::SLOT,
                                    gbid: indices.meta.buffer_id,
                                    start: indices.meta.start,
                                    end: indices.meta.end,
                                    count: (indices.meta.end - indices.meta.start) / indices.meta.data_bytes_size,
                                    format: indices.format,
                                }
                            );
                            let mut vertices = vec![];
                            let normal = RenderObjectVertice {
                                slot: BufferNormal::SLOT,
                                gbid: normal.meta.buffer_id,
                                start: normal.meta.start,
                                end: normal.meta.end,
                                count: (normal.meta.end - normal.meta.start) / normal.meta.data_bytes_size
                            };
                            vertices.push(normal);
                            let mut instances = vec![];
                            let mut meta = RenderObjectMetaOpaque {
                                bind_groups,
                                pipeline,
                                positions,
                                indices,
                                vertices,
                                instances,
                                render_sort: *rendersort,
                                view_distance,
                            };
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