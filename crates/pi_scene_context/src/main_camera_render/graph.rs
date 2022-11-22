use std::time::Instant;

use futures::FutureExt;
use pi_ecs::query::QueryState;
use pi_futures::BoxFuture;
use pi_render::{graph::{node::Node, RenderContext}, rhi::{texture::ScreenTexture}};
use render_data_container::GeometryBufferPool;

use crate::{renderers::render_object::{TempDrawInfoRecord}, object::{ObjectID, GameObject}, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}, main_camera_render::MainCameraRenderer, materials::bind_group::{RenderBindGroup, RenderBindGroupPool}};

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
        let mut time = Instant::now();

        let RenderContext {
            mut world, device, queue, ..
        } = context;

        // let window = world.get_resource::<RenderWindow>().unwrap();

        let query = QueryState::<GameObject, & MainCameraRenderer>::new(&mut world);
        let bind_groups = world.get_resource::<RenderBindGroupPool>().unwrap();

        let mut temp_vertex_record: TempDrawInfoRecord = TempDrawInfoRecord::default();

        //  println!("SingleMainCameraOpaqueRenderNode ............. {:?}", self.renderer_id);
        match query.get(&world, self.renderer_id) {
            Some(mut renderer) => {
                let opaque_list = &renderer.opaque_draws;

                if opaque_list.draws.len() > 0 {
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
                        match bind_groups.get(bindinfo.bind_group) {
                            Some(render_bind_group) => {
                                match &render_bind_group.bind_group {
                                    Some(group) => {
                                        //  println!("MainCameraOpaque set_bind_group ............. {:?}", bindinfo.offsets);
                                        renderpass.set_bind_group(render_bind_group.set, &group, &bindinfo.offsets);
                                    },
                                    None => todo!(),
                                }
                            },
                            None => todo!(),
                        }
                    });
                    let pipelines = world.get_resource::<SingleRenderObjectPipelinePool>().unwrap();
                    let gbp = world.get_resource::<SingleGeometryBufferPool>().unwrap();
    
                    time = Instant::now();
                    // let mut pipeline: Option<&RenderPipeline> = None;
                    let item = opaque_list.draws.get(0).unwrap();
                    // let buffer = gbp.get_buffer(&item.positions.gbid).clone();
                    // let buffer2 = gbp.get_buffer(&item.indices.as_ref().unwrap().gbid).clone();
                    opaque_list.draws.iter().for_each(|draw| {
                        //  println!("SingleMainCameraOpaqueRenderNode draws .............");
                        // if pipeline.is_none() {
                        //     pipeline = pipelines.map.get(draw.pipeline.id);
                        // }
                        match pipelines.map.get(draw.pipeline.id) {
                            Some(pipeline) => {
                                //  println!("SingleMainCameraOpaqueRenderNode pipeline .............");
                                let positions = &draw.positions;
                                
                                if temp_vertex_record.record_vertex_and_check_diff_with_last(positions) {
                                    match gbp.get_buffer(&positions.gbid) {
                                        Some(buffer) => {
                                            //  println!("SingleMainCameraOpaqueRenderNode draw .............");
                                            let start = positions.start as wgpu::BufferAddress;
                                            let end = positions.end as wgpu::BufferAddress;
                                            renderpass.set_vertex_buffer(positions.slot, buffer.slice(start..end));
                                        },
                                        None => {
                                        },
                                    }
                                }
                                let vertex_count = positions.count as u32;
                                        
                                renderpass.set_pipeline(pipeline);
                                draw.bind_groups.iter().for_each(|bindinfo| {
                                    match bind_groups.get(bindinfo.bind_group) {
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
                                    if temp_vertex_record.record_vertex_and_check_diff_with_last(item) {
                                        match gbp.get_buffer(&item.gbid) {
                                            Some(buffer) => {
                                                let start = item.start as wgpu::BufferAddress;
                                                let end = item.end as wgpu::BufferAddress;
                                                renderpass.set_vertex_buffer(item.slot, buffer.slice(start..end));
                                            },
                                            None => {
        
                                            },
                                        }
                                    }
                                });
                                        
                                match &draw.indices {
                                    Some(indices) => {
                                        if temp_vertex_record.record_indices_and_check_diff_with_last(indices) {
                                            match gbp.get_buffer(&indices.gbid) {
                                                Some(buffer) => {
                                                    let start = indices.start as wgpu::BufferAddress;
                                                    let end = indices.end as wgpu::BufferAddress;
                                                    renderpass.set_index_buffer(buffer.slice(start..end), indices.format);
                                                },
                                                None => {
                                                },
                                            }
                                        }
    
                                        let indices_count = indices.count as u32;
                                        renderpass.draw_indexed(0..indices_count, 0 as i32, 0..1);
                                        // println!("SingleMainCameraOpaqueRenderNode draw_indexed .............");
                                    },
                                    None => {
                                        renderpass.draw(0..vertex_count, 0..1);
                                    },
                                }
                            },
                            None => {},
                        }
                    });
                }
            },
            None => {
                
            },
        }
 
        let mut query = QueryState::<GameObject, &mut MainCameraRenderer>::new(&mut world);
        if let Some(mut renderer) = query.get_mut(&mut world, self.renderer_id) {
            renderer.clear();
        }

        let time1 = Instant::now();
        // println!("MainCameraOpaqueRenderNode: {:?}", time1 - time);

        async move {
            Ok(())
        }.boxed()
    }
}
