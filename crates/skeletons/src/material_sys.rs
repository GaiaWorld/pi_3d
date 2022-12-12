use pi_ecs::prelude::{Query, Res, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_scene_math::Vector3;
use render_geometry::geometry::VertexAttributeMeta;

use pi_scene_context::{
    cameras::camera::{CameraGlobalPosition, CameraRenderData},
    flags::SceneID,
    layer_mask::LayerMask,
    main_camera_render::{bind_group::IDMainCameraRenderBindGroup, MainCameraRenderer},
    materials::{
        bind_group::{RenderBindGroup, RenderBindGroupKey, RenderBindGroupPool},
        material::MaterialID,
    },
    meshes::model::BuildinModelBind,
    object::{GameObject, ObjectID},
    renderers::{
        pipeline::PipelineKey,
        render_blend::RenderBlend,
        render_depth_and_stencil::RenderDepthAndStencil,
        render_object::{
            RenderObjectBindGroup, RenderObjectID, RenderObjectIndices, RenderObjectMetaOpaque,
            RenderObjectVertice,
        },
        render_primitive::PrimitiveState,
        render_sort::RenderSortParam,
        render_target_state::RenderTargetState,
    },
    resources::{RenderDynUniformBuffer, SingleRenderObjectPipelinePool},
    transforms::transform_node::GlobalTransform,
    vertex_data::{
        indices::{AttributeIndices, IDAttributeIndices},
        normal::{AttributeNormal, IDAttributeNormal},
        position::{AttributePosition, IDAttributePosition},
    },
};

use crate::{
    material::SkeletonsPropertype,
    matrices_indices::{AttributeMatricesIndices, IDAttributeMatricesIndices},
    matrices_weights::{AttributeMatricesWeights, IDAttributeMatricesWeights}, pipeline::SkeletonsPipeline, shader::SkeletonsShader, bind_group::IDSkeletonsBindGroup,
};

pub struct SkeletonsUniformUpdate;
#[setup]
impl SkeletonsUniformUpdate {
    #[system]
    pub fn tick(
        neshes: Query<GameObject, (&GlobalTransform, &MaterialID, &BuildinModelBind)>,
        skeletons: Query<GameObject, &SkeletonsPropertype>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        println!("SkyboxMaterial Uniform TickUpdate");
        neshes
            .iter()
            .for_each(|(transform, mat_id, model)| match skeletons.get(mat_id.0) {
                Some(skeletons) => {
                    println!("SkyboxMaterial >>>>>>>>>>>> ");
                    dynbuffer
                        .as_mut()
                        .set_uniform::<GlobalTransform>(&model.bind_offset, transform);
                    dynbuffer
                        .as_mut()
                        .set_uniform::<SkeletonsPropertype>(&skeletons.bind_offset, &skeletons);
                }
                None => {}
            });
    }
}

pub struct SkeletonsFilter;
#[setup]
impl SkeletonsFilter {
    #[system]
    pub fn tick(
        query_camera: Query<
            GameObject,
            (&RenderObjectID, &SceneID, &LayerMask, &CameraGlobalPosition),
        >,
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
                &IDAttributePosition,
                &IDAttributeNormal,
                &IDAttributeIndices,
                &IDAttributeMatricesIndices,
                &IDAttributeMatricesWeights,
                &BuildinModelBind,
            ),
        >,
        skeletons: Query<GameObject, &SkeletonsPropertype>,
        bind_groups: Res<RenderBindGroupPool>,
        positions: Query<GameObject, &AttributePosition>,
        normals: Query<GameObject, &AttributeNormal>,
        indices: Query<GameObject, &AttributeIndices>,
        matrices_indices: Query<GameObject, &AttributeMatricesIndices>,
        matrices_weights: Query<GameObject, &AttributeMatricesWeights>,
        device: Res<RenderDevice>,
        shader: Res<SkeletonsShader>,
        id_bind_group_default: Res<IDSkeletonsBindGroup>,
        mut pipelines: ResMut<SkeletonsPipeline>,
        mut pipeline_pool: ResMut<SingleRenderObjectPipelinePool>,
    ) {
        let id_bind_group_default = id_bind_group_default.0;
        println!("SkyboxMaterial Filter");
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
                                    &skeletons,
                                    &meshes,
                                    &global_position.0,
                                    &positions,
                                    &normals,
                                    &indices,
                                    &matrices_indices,
                                    &matrices_weights,
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
    skeletons: &Query<GameObject, &SkeletonsPropertype>,
    query: &Query<
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
            &IDAttributePosition,
            &IDAttributeNormal,
            &IDAttributeIndices,
            &IDAttributeMatricesIndices,
            &IDAttributeMatricesWeights,
            &BuildinModelBind,
        ),
    >,
    camerapos: &Vector3,
    positions: &Query<GameObject, &AttributePosition>,
    normals: &Query<GameObject, &AttributeNormal>,
    indices: &Query<GameObject, &AttributeIndices>,
    matrices_indices: &Query<GameObject, &AttributeMatricesIndices>,
    matrices_weights: &Query<GameObject, &AttributeMatricesWeights>,
    pipelines: &mut SkeletonsPipeline,
    device: &RenderDevice,
    shader: &SkeletonsShader,
    pipeline_pool: &mut SingleRenderObjectPipelinePool,
    list: &mut Vec<RenderObjectMetaOpaque>,
    id_bind_group_default: RenderBindGroupKey,
) {
    query.iter().for_each(|item| {
        let rendersort = item.3;
        let blend = item.4;
        let primit = item.5;
        let depth_stencil = item.6;
        let globaltransform = item.7;
        let model = item.13;
        if sceneid == item.1 .0 && layermask.include(&item.2) {
            match skeletons.get(item.0 .0) {
                Some(mat) => {
                    println!(
                        "opaque draw obj >>>>>>>>>>>>>>> {:?}, {:?}, {:?}, {:?}",
                        sceneid, item.1 .0, layermask, item.5
                    );
                    match (
                        positions.get(item.8 .0),
                        normals.get(item.9 .0),
                        indices.get(item.10 .0),
                        matrices_indices.get(item.11 .0),
                        matrices_weights.get(item.12 .0),
                    ) {
                        (
                            Some(position),
                            Some(normal),
                            Some(indices),
                            Some(matrices_indices),
                            Some(matrices_weights),
                        ) => {
                            let view_distance =
                                camerapos.metric_distance(&globaltransform.position);
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
                                bind_group: id_bind_group_default,
                                offsets: vec![*model.bind_offset, *mat.bind_offset],
                            });

                            let positions = RenderObjectVertice {
                                slot: AttributePosition::SLOT,
                                gbid: position.meta.buffer_id,
                                start: position.meta.start,
                                end: position.meta.end,
                                count: (position.meta.end - position.meta.start)
                                    / position.meta.data_bytes_size,
                            };

                            let indices = Some(RenderObjectIndices {
                                slot: AttributePosition::SLOT,
                                gbid: indices.meta.buffer_id,
                                start: indices.meta.start,
                                end: indices.meta.end,
                                count: (indices.meta.end - indices.meta.start)
                                    / indices.meta.data_bytes_size,
                                format: indices.format,
                            });

                            let mut vertices = vec![];
                            let normal = RenderObjectVertice {
                                slot: AttributeNormal::SLOT,
                                gbid: normal.meta.buffer_id,
                                start: normal.meta.start,
                                end: normal.meta.end,
                                count: (normal.meta.end - normal.meta.start)
                                    / normal.meta.data_bytes_size,
                            };
                            vertices.push(normal);

                            let matrices_indices = RenderObjectVertice {
                                slot: AttributeMatricesIndices::SLOT,
                                gbid: matrices_indices.meta.buffer_id,
                                start: matrices_indices.meta.start,
                                end: matrices_indices.meta.end,
                                count: (matrices_indices.meta.end - matrices_indices.meta.start)
                                    / matrices_indices.meta.data_bytes_size,
                            };
                            vertices.push(matrices_indices);

                            let matrices_weights = RenderObjectVertice {
                                slot: AttributeMatricesWeights::SLOT,
                                gbid: matrices_weights.meta.buffer_id,
                                start: matrices_weights.meta.start,
                                end: matrices_weights.meta.end,
                                count: (matrices_weights.meta.end - matrices_weights.meta.start)
                                    / matrices_weights.meta.data_bytes_size,
                            };
                            vertices.push(matrices_weights);

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
                            println!("{:?}", meta);
                            list.push(meta);
                        }
                        (_, _, _, _, _) => {}
                    }
                }
                None => {}
            }
        }
    });
}
