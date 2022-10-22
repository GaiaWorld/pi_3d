use pi_ecs::{prelude::{Query, ResMut, Res}, query::With};
use pi_ecs_macros::setup;
use pi_render::{rhi::{bind_group::BindGroup, dyn_uniform_buffer::DynUniformBuffer, device::RenderDevice, RenderQueue}, graph::graph::RenderGraph};
use render_data_container::GeometryBufferPool;
use render_geometry::geometry::VertexAttributeMeta;

use crate::{object::{GameObject, ObjectID}, transforms::transform_node::GlobalTransform, renderers::{main_camera::{MainCameraBindGroup, MainCameraOpaqueRenderer}, render_object::{RenderObjectID, RenderObjectMeta, RenderObjectOpaqueList, RenderObjectPipeline, RenderObjectVertice, RenderObjectIndices, RenderObjectBindGroup}}, flags::{SceneID01, SceneCameraID01, SceneID, RenderSortParam, RenderLayerMask, RenderBlend, PrimitiveState, RenderDepthAndStencil, RenderTargetState}, scene::SceneTime, environment::fog::SceneFog, default_render::default_material::{DefaultMaterialMeta, DefaultMaterialPropertype, DefaultMaterialPipeline}, shaders::{buildin_attributes::{BuildinAttributePosition, BuildinAttributeNormal, BuildinAttributeIndices}}, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}};

use super::default::DefaultShader;

pub struct DefaultMaterialUniformTickUpdate;
#[setup]
impl DefaultMaterialUniformTickUpdate {
    #[system]
    pub fn tick(
        query_materials: Query<GameObject, (&GlobalTransform, & DefaultMaterialMeta)>,
        mut dynbuffer: ResMut<DynUniformBuffer>,
    ) {
        println!("DefaultMaterial Uniform TickUpdate");
        query_materials.iter().for_each(|(transform, material)| {
            println!("DefaultMaterial >>>>>>>>>>>> ");
            dynbuffer.set_uniform::<GlobalTransform>(&material.model_bind_offset, transform);
            dynbuffer.set_uniform::<DefaultMaterialPropertype>(&material.value.bind_offset, &material.value);
        });
    }
}

pub struct DefaultMaterialTickBeforeRender;
#[setup]
impl DefaultMaterialTickBeforeRender {
    #[system]
    pub fn tick(
        query_camera: Query<GameObject, (&RenderObjectID, &MainCameraBindGroup, &SceneID, &RenderLayerMask)>,
        mut query_render_opaque: Query<GameObject, (&mut RenderObjectOpaqueList)>,
        query_drawobj: Query<GameObject, (&DefaultMaterialMeta, &SceneID, &BuildinAttributePosition, &BuildinAttributeNormal, &BuildinAttributeIndices, &RenderLayerMask, &RenderSortParam, &RenderBlend, &PrimitiveState, &RenderDepthAndStencil)>,
        mut pipelines: ResMut<DefaultMaterialPipeline>,
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
        shader: Res<DefaultShader>,
        gbp: Res<SingleGeometryBufferPool>,
        mut dynbuffer: ResMut<DynUniformBuffer>,
        mut pipeline_pool: ResMut<SingleRenderObjectPipelinePool>,
    ) {
        println!("DefaultMaterial Tick BeforeRender");
        dynbuffer.write_buffer(&device, &queue);
        query_camera.iter().for_each(|(renderid, camera_bind_group, sceneid, layermask)| {
            println!("Camera >>>>>>>>>>>>>>>");
            match &camera_bind_group.bind_group {
                Some(bind_group) => {
                    match query_render_opaque.get_mut(renderid.0) {
                        Some(mut renderlist) => {
                            println!("opaque List >>>>>>>>>>>>>>>");
                            renderlist.draws.clear();
                            renderlist.bind_groups.clear();

                            renderlist.bind_groups.push(RenderObjectBindGroup { value: bind_group.clone(), set: camera_bind_group.set });
        
                            let mut opaque_list = vec![];
        
                            query_drawobj.iter().for_each(|item| {
                                println!("opaque draw obj >>>>>>>>>>>>>>> {:?}, {:?}, {:?}, {:?}", sceneid.0, item.1.0, layermask, item.5);
                                if sceneid.0 == item.1.0 && layermask.include(&item.5) {
                                    collect_opaque_normal_depth(item, &mut pipelines, &device, &queue, &shader, &gbp, &mut dynbuffer, &mut pipeline_pool, &mut opaque_list);
                                }
                            });
        
                            opaque_list.into_iter().for_each(|v| {
                                renderlist.draws.push(v);
                            });
                        },
                        None => {},
                    }
                },
                None => {
                    
                },
            }
        });
    }
}

fn collect_opaque_normal_depth(
    query: (&DefaultMaterialMeta, &SceneID, &BuildinAttributePosition, &BuildinAttributeNormal, &BuildinAttributeIndices, &RenderLayerMask, &RenderSortParam, &RenderBlend, &PrimitiveState, &RenderDepthAndStencil),
    pipelines: &mut DefaultMaterialPipeline,
    device: & RenderDevice,
    queue: & RenderQueue,
    shader: & DefaultShader,
    gbp: & SingleGeometryBufferPool,
    dynbuffer: &mut DynUniformBuffer,
    pipeline_pool: &mut SingleRenderObjectPipelinePool,
    list: &mut Vec<RenderObjectMeta>,
) {
    println!("collect_opaque_normal_depth ");
    let (mat, sceneid, position, normal, indices, layermask, rendersort, blend, primit, depth_stencil) = query;
    if depth_stencil.depth {
        match &mat.bind_group {
            Some(bind_group) => {
                let pipeline = pipelines.build(
                    device,
                    shader,
                    RenderTargetState::color_target(blend).as_slice(),
                    depth_stencil.state(),
                    primit.state,
                    pipeline_pool,
                );
                let pipeline = RenderObjectPipeline { id: pipeline };
                let mut bind_groups = vec![];
                bind_groups.push(RenderObjectBindGroup { value: bind_group.clone(), set: mat.set });

                let positions = RenderObjectVertice {
                    slot: BuildinAttributePosition::SLOT,
                    gbid: position.meta.buffer_id,
                    start: position.meta.start,
                    end: position.meta.end,
                    count: (position.meta.end - position.meta.start) / position.meta.data_bytes_size
                };
                let indices = Some(
                    RenderObjectIndices {
                        slot: BuildinAttributePosition::SLOT,
                        gbid: indices.meta.buffer_id,
                        start: indices.meta.start,
                        end: indices.meta.end,
                        count: (indices.meta.end - indices.meta.start) / indices.meta.data_bytes_size,
                        format: indices.format,
                    }
                );
                let mut vertices = vec![];
                let normal = RenderObjectVertice {
                    slot: BuildinAttributePosition::SLOT,
                    gbid: normal.meta.buffer_id,
                    start: normal.meta.start,
                    end: normal.meta.end,
                    count: (normal.meta.end - normal.meta.start) / normal.meta.data_bytes_size
                };
                vertices.push(normal);
                let mut instances = vec![];
                let mut meta = RenderObjectMeta {
                    bind_groups,
                    pipeline,
                    positions,
                    indices,
                    vertices,
                    instances,
                };
                println!("{:?}", meta);
                list.push(meta);
            },
            None => {
                println!("DefaultMaterialMeta Not Get Bindgroup!");
            }
        }
    }
}

fn render_scene_camera<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    camera: &'a MainCameraBindGroup,
) {
    match camera.bind_group.as_ref() {
        Some(bind_group) => {
            renderpass.set_bind_group(camera.set, &bind_group, &[]);
        },
        None => {},
    }
}

// fn render_scene_camera_model<'a>(
//     renderpass: &mut wgpu::RenderPass<'a>,
//     material: &'a DefaultMaterialMeta,
// ) {
//     match material.bind_group.as_ref() {
//         Some(bind_group) => {
//             renderpass.set_bind_group(material.set, &bind_group, &[]);
//         },
//         None => {},
//     }
// }

// fn render_vertex_position<'a>(
//     renderpass: &mut wgpu::RenderPass<'a>,
//     position: & BuildinAttributePosition,
//     gbp: &'a SingleGeometryBufferPool,
// ) {
//     match gbp.get_buffer(&position.buffer_id) {
//         Some(buffer) => {
//             renderpass.set_vertex_buffer(BuildinAttributePosition::SLOT, buffer.slice(..));
//         },
//         None => {},
//     }
// }

// fn render_vertex_color<'a>(
//     renderpass: &mut wgpu::RenderPass<'a>,
//     color: & BuildinAttributeColor4,
//     gbp: &'a SingleGeometryBufferPool,
// ) {
//     match gbp.get_buffer(&color.buffer_id) {
//         Some(buffer) => {
//             renderpass.set_vertex_buffer(BuildinAttributeColor4::SLOT, buffer.slice(..));
//         },
//         None => {},
//     }
// }

// fn render_vertex_indices<'a>(
//     renderpass: &mut wgpu::RenderPass<'a>,
//     indices: & BuildinAttributeIndices,
//     gbp: &'a SingleGeometryBufferPool,
// ) {
//     match gbp.get_buffer(&indices.buffer_id) {
//         Some(buffer) => {
//             renderpass.set_index_buffer(buffer.slice(..), indices.format);
//         },
//         None => {},
//     }
// }

// fn render_draw<'a>(
//     renderpass: &mut wgpu::RenderPass<'a>,
//     position: & BuildinAttributePosition,
//     gbp: &'a SingleGeometryBufferPool,
// ) {
//     let vertices = gbp.get_size(&position.buffer_id) as u32;
//     renderpass.draw(0..vertices, 0..1);
// }

// fn render_draw_indexed<'a>(
//     renderpass: &mut wgpu::RenderPass<'a>,
//     indices: & BuildinAttributeIndices,
//     position: & BuildinAttributePosition,
//     gbp: &'a SingleGeometryBufferPool,
// ) {
//     let indices = gbp.get_size(&indices.buffer_id) as u32;
//     let base_vertex = gbp.get_size(&position.buffer_id) as i32;
//     renderpass.draw_indexed(0..indices, base_vertex, 0..1);
// }