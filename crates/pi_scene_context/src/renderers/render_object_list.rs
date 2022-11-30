use std::time::Instant;

use render_data_container::GeometryBufferPool;

use crate::{materials::bind_group::RenderBindGroupPool, renderers::render_object::TempDrawInfoRecord, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}};

use super::render_object::{RenderObjectBindGroup, RenderObjectMetaOpaque, RenderObjectMetaTransparent, DrawObject};


pub trait DrawList<T: DrawObject> {
    fn bindgroups(&self) -> &Vec<RenderObjectBindGroup>;
    fn drawlist(&self) -> &Vec<T>;
    fn render(
        &self,
        commands: &mut wgpu::CommandEncoder,
        target_view: &wgpu::TextureView,
        bindgrouppool: &RenderBindGroupPool,
        pipelines: &SingleRenderObjectPipelinePool,
        gbp: &SingleGeometryBufferPool,
    ) {
        let mut time = Instant::now();

        let bind_groups = self.bindgroups();
        let draws = self.drawlist();

        let mut temp_vertex_record: TempDrawInfoRecord = TempDrawInfoRecord::default();

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
                    view: target_view,
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

        bind_groups.iter().for_each(|bindinfo| {
            match bindgrouppool.get(&bindinfo.bind_group) {
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

        time = Instant::now();

        draws.iter().for_each(|draw| {
            match pipelines.map.get(draw.pipeline().id) {
                Some(pipeline) => {
                    let positions = &draw.positions();
                    
                    if temp_vertex_record.record_vertex_and_check_diff_with_last(positions) {
                        match gbp.get_buffer(&positions.gbid) {
                            Some(buffer) => {
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
                    draw.bind_groups().iter().for_each(|bindinfo| {
                        match bindgrouppool.get(&bindinfo.bind_group) {
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

                    draw.vertices().iter().for_each(|item| {
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
                            
                    match &draw.indices() {
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
                        },
                        None => {
                            renderpass.draw(0..vertex_count, 0..1);
                        },
                    }
                },
                None => {},
            }
        });
        
        let time1 = Instant::now();
        println!("DrawList: {:?}", time1 - time);
    }
}

#[derive(Default)]
pub struct RenderObjectOpaqueList {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub draws: Vec<RenderObjectMetaOpaque>,
}
impl DrawList<RenderObjectMetaOpaque> for RenderObjectOpaqueList {
    fn bindgroups(&self) -> &Vec<RenderObjectBindGroup> {
        &self.bind_groups
    }

    fn drawlist(&self) -> &Vec<RenderObjectMetaOpaque> {
        &self.draws
    }
}

#[derive(Default)]
pub struct RenderObjectTransparentList {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub draws: Vec<RenderObjectMetaTransparent>,
}
impl DrawList<RenderObjectMetaTransparent> for RenderObjectTransparentList {
    fn bindgroups(&self) -> &Vec<RenderObjectBindGroup> {
        &self.bind_groups
    }

    fn drawlist(&self) -> &Vec<RenderObjectMetaTransparent> {
        &self.draws
    }
}
