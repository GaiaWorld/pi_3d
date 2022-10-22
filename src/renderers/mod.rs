use pi_ecs::world::World;
use pi_futures::BoxFuture;
use pi_render::{components::view::target_alloc::ShareTargetView, graph::{node::Node, param::{InParam, OutParam}}};
use render_data_container::{TGeometryBufferID, GeometryBufferPool, TVertexBufferKindKey, EVertexDataFormat};
use render_geometry::geometry::{};
use render_material::error::EMaterialError;

use crate::{meshes::Mesh, geometry::{VDK, GBID}, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}, shaders::buildin_attributes::BuildinAttributeIndices};

use self::render_object::RenderObjectOpaqueList;

pub mod pipeline;
pub mod render_default;
pub mod main_camera;
pub mod render_object;

#[derive(Debug, Clone)]
pub struct RenderInput {

}
impl Default for RenderInput {
    fn default() -> Self {
        Self {

        }
    }
}
impl InParam for RenderInput {
    fn can_fill<O: OutParam + ?Sized>(
        &self,
        map: &mut pi_hash::XHashMap<std::any::TypeId, Vec<pi_render::graph::NodeId>>,
        pre_id: pi_render::graph::NodeId,
        out_param: &O,
    ) -> bool {
        todo!()
    }

    fn fill_from<O: OutParam + ?Sized>(&mut self, pre_id: pi_render::graph::NodeId, out_param: &O) -> bool {
        todo!()
    }
}

#[derive(Clone)]
pub struct RenderOutput {
    pub tex: Option<ShareTargetView>,
}
impl  Default for RenderOutput {
    fn default() -> Self {
        Self {
            tex: None,
        }
    }
}
impl OutParam for RenderOutput {
    fn can_fill(&self, set: &mut Option<&mut pi_hash::XHashSet<std::any::TypeId>>, ty: std::any::TypeId) -> bool {
        todo!()
    }

    fn fill_to(&self, this_id: pi_render::graph::NodeId, to: &mut dyn pi_render::graph::param::Assign, ty: std::any::TypeId) -> bool {
        todo!()
    }
}

pub struct RenderNode {
    pub world: World,
    pub opaque: RenderObjectOpaqueList,
}
impl RenderNode {
    pub fn new(world: &World) -> Self {
        Self {
            opaque: RenderObjectOpaqueList::default(),
            world: world.clone(),
        }
    }
}
impl Node for RenderNode {
    type Input = RenderInput;

    type Output = RenderOutput;

    fn run<'a>(
        &'a mut self,
        context: pi_render::graph::RenderContext,
        mut commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        usage: &'a pi_render::graph::node::ParamUsage,
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        let mut renderpass = commands.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderNode"),
                color_attachments: todo!(),
                depth_stencil_attachment: todo!(),
            }
        );

        self.opaque.bind_groups.iter().for_each(|bind| {
            renderpass.set_bind_group(bind.set, &bind.value, &[]);
        });

        self.opaque.draws.iter().for_each(|draw| {
            let pipelines = self.world.get_resource::<SingleRenderObjectPipelinePool>().unwrap();
            let gbp = self.world.get_resource::<SingleGeometryBufferPool>().unwrap();
            match pipelines.map.get(draw.pipeline.id) {
                Some(pipeline) => {
                    let positions = &draw.positions;
                    match gbp.get_buffer(&positions.gbid) {
                        Some(buffer) => {
                            let start = positions.start as wgpu::BufferAddress;
                            let end = positions.end as wgpu::BufferAddress;
                            renderpass.set_vertex_buffer(positions.slot, buffer.slice(start..end));
                            let vertex_count = positions.count as u32;
                            
                            renderpass.set_pipeline(pipeline);
                            draw.bind_groups.iter().for_each(|bind| {
                                renderpass.set_bind_group(bind.set, &bind.value, &[]);
                            });
                            draw.vertices.iter().for_each(|item| {
                                match gbp.get_buffer(&item.gbid) {
                                    Some(buffer) => {
                                        let start = item.start as wgpu::BufferAddress;
                                        let end = item.end as wgpu::BufferAddress;
                                        renderpass.set_vertex_buffer(item.slot, buffer.slice(start..end));
                                    },
                                    None => {

                                    },
                                }
                            });
                            
                            match &draw.indices {
                                Some(indices) => {
                                    match gbp.get_buffer(&indices.gbid) {
                                        Some(buffer) => {
                                            let start = indices.start as wgpu::BufferAddress;
                                            let end = indices.end as wgpu::BufferAddress;
                                            renderpass.set_index_buffer(buffer.slice(start..end), indices.format);
                                            let indices_count = indices.count as u32;
                                            renderpass.draw_indexed(0..indices_count, vertex_count as i32, 0..1)
                                        },
                                        None => {
                                            renderpass.draw(0..vertex_count, 0..1)
                                        },
                                    }
                                },
                                None => {
                                    
                                },
                            }
                        },
                        None => {
                        },
                    }
                },
                None => {},
            }
        });
        todo!()
    }

    fn build<'a>(
        &'a mut self,
        _context: pi_render::graph::RenderContext,
        _usage: &'a pi_render::graph::node::ParamUsage,
    ) -> Option<BoxFuture<'a, Result<(), String>>> {
        None
    }
}