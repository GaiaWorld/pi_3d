
use futures::FutureExt;
use pi_animation::animation_group_manager::AnimationGroupManager;
use pi_ecs::prelude::GraphNode;
use pi_render::{graph::{node::{Node, ParamUsage}, RenderContext}, depend_graph, components::view::target_alloc::{ShareTargetView, SafeAtlasAllocator, TargetDescriptor, TextureDescriptor}, rhi::texture::ScreenTexture };
use render_derive::NodeParam;
use smallvec::SmallVec;

#[derive(NodeParam, Clone, Default)]
pub struct RT {
    pub rt: Option<ShareTargetView>,
    pub width: u32,
    pub height: u32,
    pub depth: bool,
}

pub struct CreateRTNode {
    pub width: u32,
    pub height: u32,
    pub depth: bool,
}
impl Node for CreateRTNode {
    type Input = ();

    type Output = RT;

    fn run<'a>(
        &'a mut self,
        context: pi_render::graph::RenderContext,
        commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        usage: &'a pi_render::graph::node::ParamUsage,
    ) -> pi_futures::BoxFuture<'a, Result<Self::Output, String>> {
        let tex_alloc = context.world.get_resource_mut::<SafeAtlasAllocator>().unwrap();
        let desc = TextureDescriptor {
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            base_mip_level: 0,
            base_array_layer: 0,
            array_layer_count: None,
            view_dimension: None,
        };
        let mut list = SmallVec::new();
        list.push(desc);
        let desc = TargetDescriptor {
            texture_descriptor: list,
            need_depth: true,
            default_width: 1024,
            default_height: 1024,
        };
        let target_type = tex_alloc.get_or_create_type(desc);
        let ignores: Vec<ShareTargetView> = vec![];
        let srt = tex_alloc.allocate(self.width, self.height, target_type, ignores.iter());
        async move {
            Ok(
                RT {
                    rt: Some(srt),
                    width: self.width,
                    height: self.height,
                    depth: self.depth,
                }
            )
        }.boxed()
    }
}

#[derive(NodeParam, Clone, Default)]
pub struct RenderTarget {
    pub rt: Option<ShareTargetView>,
}

pub struct ScreenClearNode {
    pub color: (f64, f64, f64, f64),
    pub depth: f64,
    // pub flag: 
}
impl Node for ScreenClearNode {
    type Input = ();

    type Output = ();

    fn run<'a>(
        &'a mut self,
        context: pi_render::graph::RenderContext,
        mut commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        usage: &'a ParamUsage,
    ) -> pi_futures::BoxFuture<'a, Result<Self::Output, String>> {
        // println!("Clear ");

        let RenderContext {
            world, device, queue, ..
        } = context;

        let surface = world.get_resource::<ScreenTexture>().unwrap();

        let ops = wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color{r: self.color.0, g: self.color.1, b: self.color.2, a: self.color.3}),
            store: true,
        };
        let mut color_attachments = vec![];
        let mut depth_stencil_attachment = None;
        // match input.rt.as_ref() {
        //     Some(rt) => {
        //         let view = rt.target().colors.get(0).unwrap();
        //         color_attachments.push(
        //             Some(
        //                 wgpu::RenderPassColorAttachment {
        //                     resolve_target: None,
        //                     ops,
        //                     view: &view.0,
        //                 }
        //             )
        //         );
        //         depth_stencil_attachment = Some(
        //             wgpu::RenderPassDepthStencilAttachment {
        //                 stencil_ops: None,
        //                 depth_ops: Some(
        //                         wgpu::Operations {
        //                         load: wgpu::LoadOp::Load,
        //                         store: true,
        //                     }
        //                 ),
        //                 view: &view.0,
        //             }
        //         );
        //     },
        //     None => {
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
        //     },
        // }
        let renderpass = commands.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: color_attachments.as_slice(),
                depth_stencil_attachment: depth_stencil_attachment,
            }
        );

        async move {
            Ok(())
        }.boxed()
    }
    
}