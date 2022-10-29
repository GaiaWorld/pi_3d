use futures::FutureExt;
use pi_ecs::query::QueryState;
use pi_futures::BoxFuture;
use pi_render::{graph::{NodeId, graph::RenderGraph, node::Node, RenderContext}, rhi::texture::ScreenTexture};
use render_data_container::GeometryBufferPool;

use crate::{cameras::camera::CameraViewport, renderers::render_object::{RenderObjectOpaqueList, RenderObjectTransparentList}, object::{ObjectID, GameObject}, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}, main_camera_render::MainCameraRenderer, materials::bind_group::RenderBindGroup};

pub struct SingleMainCameraOpaqueRenderNode {
    pub renderer_id: ObjectID,
}
impl SingleMainCameraOpaqueRenderNode {
    pub fn new(renderer_id: ObjectID) -> Self {
        Self {
            renderer_id,
        }
    }
}
impl Node for SingleMainCameraOpaqueRenderNode {
    type Input = ();

    type Output = ();

    fn run<'a>(
        &'a mut self,
        context: pi_render::graph::RenderContext,
        mut commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        usage: &'a pi_render::graph::node::ParamUsage,
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        let RenderContext {
            mut world, device, queue, ..
        } = context;

        // let window = world.get_resource::<RenderWindow>().unwrap();

        let query = QueryState::<GameObject, &MainCameraRenderer>::new(&mut world);
        let bind_groups = QueryState::<GameObject, &RenderBindGroup>::new(&mut world);

        println!("SingleMainCameraOpaqueRenderNode ............. {:?}", self.renderer_id);
        match query.get(&world, self.renderer_id) {
            Some(renderer) => {
                let opaque_list = &renderer.opaque_draws;
                let surface = world.get_resource::<ScreenTexture>().unwrap();
                let ops = wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                };
                let mut color_attachments = vec![];
                let mut depth_stencil_attachment = None;
                color_attachments.push(
                    Some(
                        wgpu::RenderPassColorAttachment {
                            resolve_target: None,
                            ops,
                            view: surface.view.as_ref().unwrap(),
                        }
                    )
                );
                depth_stencil_attachment = None;
        
                let mut renderpass = commands.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("RenderNode"),
                        color_attachments: color_attachments.as_slice(),
                        depth_stencil_attachment: depth_stencil_attachment,
                    }
                );

                // let x = renderer.viewport.x;
                // let y = renderer.viewport.y;
                // let w = renderer.viewport.w * window.;
                // let h = renderer.viewport.h;

        
                // renderpass.set_viewport(0., 0., 600., 600., 0., 1.);

                opaque_list.bind_groups.iter().for_each(|bindinfo| {
                    match bind_groups.get(&world, bindinfo.bind_group) {
                        Some(render_bind_group) => {
                            match &render_bind_group.bind_group {
                                Some(group) => {
                                    println!("MainCameraOpaque set_bind_group ............. {:?}", bindinfo.offsets);
                                    renderpass.set_bind_group(render_bind_group.set, &group, &bindinfo.offsets);
                                },
                                None => todo!(),
                            }
                        },
                        None => todo!(),
                    }
                });
        
                opaque_list.draws.iter().for_each(|draw| {
                    println!("SingleMainCameraOpaqueRenderNode draws .............");
                    let pipelines = world.get_resource::<SingleRenderObjectPipelinePool>().unwrap();
                    let gbp = world.get_resource::<SingleGeometryBufferPool>().unwrap();
                    match pipelines.map.get(draw.pipeline.id) {
                        Some(pipeline) => {
                            println!("SingleMainCameraOpaqueRenderNode pipeline .............");
                            let positions = &draw.positions;
                            match gbp.get_buffer(&positions.gbid) {
                                Some(buffer) => {
                                    println!("SingleMainCameraOpaqueRenderNode draw .............");
                                    let start = positions.start as wgpu::BufferAddress;
                                    let end = positions.end as wgpu::BufferAddress;
                                    renderpass.set_vertex_buffer(positions.slot, buffer.slice(start..end));
                                    let vertex_count = positions.count as u32;
                                    
                                    renderpass.set_pipeline(pipeline);
                                    draw.bind_groups.iter().for_each(|bindinfo| {
                                        match bind_groups.get(&world, bindinfo.bind_group) {
                                            Some(render_bind_group) => {
                                                match &render_bind_group.bind_group {
                                                    Some(group) => {
                                                        renderpass.set_bind_group(render_bind_group.set, &group, &bindinfo.offsets);
                                                    },
                                                    None => todo!(),
                                                }
                                            },
                                            None => todo!(),
                                        }
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
                                                    renderpass.draw_indexed(0..indices_count, 0 as i32, 0..1);
                                                    println!("SingleMainCameraOpaqueRenderNode draw_indexed .............");
                                                },
                                                None => {
                                                    renderpass.draw(0..vertex_count, 0..1);
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
            },
            None => {
                
            },
        }

        async move {
            Ok(())
        }.boxed()
    }
}

