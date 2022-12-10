use std::{time::Instant, ops::Range};

use crate::{materials::bind_group::RenderBindGroupPool, renderers::render_object::TempDrawInfoRecord};

use super::render_object::{RenderObjectBindGroup, RenderObjectMetaOpaque, RenderObjectMetaTransparent, DrawObject};


pub trait DrawList<T: DrawObject> {
    fn bindgroups(&self) -> &Vec<RenderObjectBindGroup>;
    fn drawlist(&self) -> &Vec<T>;
    fn render<'a>(
        &self,
        commands: &'a mut wgpu::CommandEncoder,
        target_view: &wgpu::TextureView,
        depth_stencil: Option<wgpu::RenderPassDepthStencilAttachment>,
        bindgrouppool: &RenderBindGroupPool,
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
        color_attachments.push(
            Some(
                wgpu::RenderPassColorAttachment {
                    resolve_target: None,
                    ops,
                    view: target_view,
                }
            )
        );

        let mut renderpass = commands.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderNode"),
                color_attachments: color_attachments.as_slice(),
                depth_stencil_attachment: depth_stencil,
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
            renderpass.set_pipeline(draw.pipeline());
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

            let mut vertex_range = 0..0;
            let mut instance_range = 0..1;
            draw.vertices().iter().for_each(|item| {
                if temp_vertex_record.record_vertex_and_check_diff_with_last(item) {
                    renderpass.set_vertex_buffer(item.slot, item.slice());
                    vertex_range = item.value_range();
                }
            });

            draw.instances().iter().for_each(|item| {
                if temp_vertex_record.record_vertex_and_check_diff_with_last(item) {
                    renderpass.set_vertex_buffer(item.slot, item.slice());
                    instance_range = item.value_range();
                }
            });

            match &draw.indices() {
                Some(indices) => {
                    if temp_vertex_record.record_indices_and_check_diff_with_last(indices) {
                        renderpass.set_index_buffer(indices.slice(), indices.format);
                    }

                    renderpass.draw_indexed(indices.value_range(), 0 as i32, instance_range);
                },
                None => {
                    renderpass.draw(vertex_range, instance_range);
                },
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
