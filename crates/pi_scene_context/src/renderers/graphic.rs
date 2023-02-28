use std::{time::Instant, sync::Arc};

use futures::FutureExt;
use pi_atom::Atom;
use pi_ecs::query::QueryState;
use pi_engine_shell::object::{ObjectID, GameObject};
use pi_futures::BoxFuture;
use pi_render::{components::view::target_alloc::ShareTargetView, graph::{param::OutParam, node::Node, RenderContext}, rhi::{device::RenderDevice, texture::ScreenTexture}};

use crate::{renderers::{renderer::Renderer}, pass::PassTagOrders};


#[derive(Clone)]
pub struct RendererGraphicParam {
    pub srt: Option<ShareTargetView>,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub depth: bool,
}
impl Default for RendererGraphicParam {
    fn default() -> Self {
        Self {
            srt: None,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            depth: false,
        }
    }
}
impl OutParam for RendererGraphicParam {
    fn can_fill(&self, set: &mut Option<&mut pi_hash::XHashSet<std::any::TypeId>>, ty: std::any::TypeId) -> bool {
        if set.is_none() {
            true
        } else {
            std::any::TypeId::of::<Self>() == ty
        }
    }

    fn fill_to(&self, this_id: pi_render::graph::NodeId, to: &mut dyn pi_render::graph::param::Assign, ty: std::any::TypeId) -> bool {
        if std::any::TypeId::of::<Self>() == ty {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct RendererGraphicDesc {
    pub pre: Option<Atom>,
    pub curr: Atom,
    pub next: Option<Atom>,
    pub passorders: PassTagOrders,
}


pub struct RenderNode {
    pub renderer_id: ObjectID,
}
impl RenderNode {
    pub fn new(renderer_id: ObjectID) -> Self {
        Self {
            renderer_id,
        }
    }
}
impl Node for RenderNode {
    type Input = ();

    type Output = ();

    fn run<'a>(
        &'a mut self,
        context: pi_render::graph::RenderContext,
        mut commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        usage: &'a pi_render::graph::node::ParamUsage,
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        let time = Instant::now();

        let RenderContext {
            mut world, ..
        } = context;

        // let window = world.get_resource::<RenderWindow>().unwrap();

        let query = QueryState::<GameObject, &Renderer>::new(&mut world);

        let mut output = RendererGraphicParam::default();

        //  log::debug!("SingleMainCameraOpaqueRenderNode ............. {:?}", self.renderer_id);
        match query.get(&world, self.renderer_id) {
            Some(renderer) => {
                let window = world.get_resource::<Arc<winit::window::Window>>().unwrap();
                let device = world.get_resource::<RenderDevice>().unwrap();
                let width = window.inner_size().width;
                let height = window.inner_size().height;

                let currlist: Vec<ShareTargetView> = vec![];

                // let atlas_allocator = world.get_resource::<Share<AssetMgr::<RenderRes<wgpu::TextureView>>>>().unwrap();
                // let atlas_allocator = SafeAtlasAllocator::new(device.clone(), atlas_allocator.clone(), Arc::new(HomogeneousMgr::<RenderRes<UnuseTexture>, GarbageEmpty>));
                // let srt = atlas_allocator.allocate(
                //     width,
                //     height,
                //     atlas_allocator.get_or_create_type(TargetDescriptor {
                //         texture_descriptor: SmallVec::from_slice(
                //             &[
                //                 TextureDescriptor {
                //                     mip_level_count: 1,
                //                     sample_count: 1,
                //                     dimension: wgpu::TextureDimension::D2,
                //                     format: wgpu::TextureFormat::Rgba8UnormSrgb,
                //                     usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
                //                     base_mip_level: 0,
                //                     base_array_layer: 0,
                //                     array_layer_count: None,
                //                     view_dimension: None,
                //                 }
                //             ]
                //         ),
                //         need_depth: true,
                //         default_width: width,
                //         default_height: height,
                //     }),
                //     currlist.iter()
                // );

                // let target = srt.target().colors[0].0.as_ref();
                
                // Clear
                // {
                //     let ops = wgpu::Operations {
                //         load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                //         store: true,
                //     };
                //     let mut color_attachments = vec![];
                //     color_attachments.push(
                //         Some(
                //             wgpu::RenderPassColorAttachment {
                //                 resolve_target: None,
                //                 ops,
                //                 view: target,
                //             }
                //         )
                //     );
                //     let depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
                //         view: srt.target().depth.as_ref().unwrap().0.as_ref(),
                //         depth_ops: Some(wgpu::Operations::<f32> {
                //             load: wgpu::LoadOp::Clear(-1.),
                //             store: true
                //         }),
                //         stencil_ops: None,
                //     });
                //     let renderpass = commands.begin_render_pass(
                //         &wgpu::RenderPassDescriptor {
                //             label: Some("MainCameraClear"),
                //             color_attachments: color_attachments.as_slice(),
                //             depth_stencil_attachment: depth_stencil_attachment,
                //         }
                //     );
                // }

                // Draw Scene
                
                // let depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
                //     view: srt.target().depth.as_ref().unwrap().0.as_ref(),
                //     depth_ops: Some(wgpu::Operations::<f32> {
                //         load: wgpu::LoadOp::Load,
                //         store: true
                //     }),
                //     stencil_ops: None,
                // });

                let surface = world.get_resource::<ScreenTexture>().unwrap();
                let target = surface.view.as_ref().unwrap();

                let depth_stencil_attachment = None;
                

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
                            view: target.as_ref(),
                        }
                    )
                );

                let mut renderpass = commands.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("RenderNode"),
                        color_attachments: color_attachments.as_slice(),
                        depth_stencil_attachment: depth_stencil_attachment,
                    }
                );

                renderpass.set_viewport(0., 0., 800., 600., 0., 1.);

                renderer.draws.render(&mut renderpass);

                // // To Screen
                // {
                //     let surface = world.get_resource::<ScreenTexture>().unwrap();
                // }

                // output.srt = Some(srt);
                output.w = width;
                output.h = height;
            },
            None => {
                
            },
        }

        let time1 = Instant::now();
        log::debug!("MainCameraRenderNode: {:?}", time1 - time);

        async move {
            Ok(())
        }.boxed()
    }
}

// pub fn main_camera_renderer<'a>(
//     renderer: &'a Renderer,
//     commands: &'a mut wgpu::CommandEncoder,
//     target_view: &wgpu::TextureView,
//     depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment>,
//     bindgrouppool: &RenderBindGroupPool,
//     vbpool: &VertexBufferPool,
// ) {
//     renderer.opaque_draws.render(commands, target_view, depth_stencil_attachment.clone(), bindgrouppool, vbpool);
//     renderer.transparent_draws.render(commands, target_view, depth_stencil_attachment, bindgrouppool, vbpool);
// }