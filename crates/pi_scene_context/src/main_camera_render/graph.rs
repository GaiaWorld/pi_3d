use std::{time::Instant, sync::Arc};

use futures::FutureExt;
use pi_assets::mgr::AssetMgr;
use pi_ecs::query::QueryState;
use pi_futures::BoxFuture;
use pi_render::{graph::{node::Node, RenderContext, param::OutParam}, rhi::{texture::ScreenTexture, asset::RenderRes, device::RenderDevice}, components::view::target_alloc::{SafeAtlasAllocator, TargetDescriptor, TextureDescriptor, ShareTargetView}};
use pi_share::Share;
use render_data_container::VertexBufferPool;
use smallvec::SmallVec;

use crate::{
    renderers::{render_object_list::DrawList, renderer::Renderer, graphic::RendererGraphicParam},
    object::{ObjectID, GameObject},
    bindgroup::{RenderBindGroupPool}
};

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
                {
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
    
                    let bindgrouppool = world.get_resource::<RenderBindGroupPool>().unwrap();
                    let vbpool = world.get_resource::<VertexBufferPool>().unwrap();

                    main_camera_renderer(
                        renderer,
                        &mut commands,
                        target.as_ref(),
                        depth_stencil_attachment,
                        bindgrouppool, vbpool
                    );
                }

                // To Screen
                {
                    let surface = world.get_resource::<ScreenTexture>().unwrap();
                }

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

pub fn main_camera_renderer<'a>(
    renderer: &'a Renderer,
    commands: &'a mut wgpu::CommandEncoder,
    target_view: &wgpu::TextureView,
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment>,
    bindgrouppool: &RenderBindGroupPool,
    vbpool: &VertexBufferPool,
) {
    renderer.opaque_draws.render(commands, target_view, depth_stencil_attachment.clone(), bindgrouppool, vbpool);
    renderer.transparent_draws.render(commands, target_view, depth_stencil_attachment, bindgrouppool, vbpool);
}