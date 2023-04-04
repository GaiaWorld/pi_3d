use std::{time::Instant, sync::Arc, num::NonZeroU32};

use futures::FutureExt;
use pi_assets::mgr::AssetMgr;
use pi_atom::Atom;
use pi_ecs::query::QueryState;
use pi_engine_shell::object::{ObjectID, GameObject};
use pi_futures::BoxFuture;
use pi_postprocess::{image_effect::{EffectCopy, SingleImageEffectResource}, effect::CopyIntensity, temprory_render_target::PostprocessTexture, IDENTITY_MATRIX};
use pi_render::{components::view::target_alloc::{ShareTargetView, SafeAtlasAllocator, TargetDescriptor, TextureDescriptor}, graph::{param::{OutParam, InParam}, node::Node, RenderContext}, rhi::{device::RenderDevice, texture::ScreenTexture, RenderQueue, asset::RenderRes, pipeline::RenderPipeline}, renderer::{draw_obj_list::DrawList, texture::texture_view::ETextureViewUsage}};
use pi_share::Share;
use smallvec::SmallVec;

use crate::{renderers::{renderer::{Renderer, RenderSize, RenderColorFormat, RenderColorClear, RenderDepthFormat, RenderDepthClear}}, pass::PassTagOrders};


#[derive(Clone)]
pub struct RendererGraphicParam {
    pub srt: Option<ShareTargetView>,
    pub depth: bool,
}
impl Default for RendererGraphicParam {
    fn default() -> Self {
        Self {
            srt: None,
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
impl InParam for RendererGraphicParam {
    fn can_fill<O: OutParam + ?Sized>(
        &self,
        map: &mut pi_hash::XHashMap<std::any::TypeId, Vec<pi_render::graph::NodeId>>,
        pre_id: pi_render::graph::NodeId,
        out_param: &O,
    ) -> bool {
        true
    }

    fn fill_from<O: OutParam + ?Sized>(&mut self, pre_id: pi_render::graph::NodeId, out_param: &O) -> bool {
        true
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
        mut context: pi_render::graph::RenderContext,
        mut commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        usage: &'a pi_render::graph::node::ParamUsage,
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        let time = Instant::now();

        let mut output = RendererGraphicParam::default();
        

        // let window = world.get_resource::<RenderWindow>().unwrap();

        let query = QueryState::<GameObject, (&Renderer, &RenderSize, &RenderColorFormat, &RenderColorClear, &RenderDepthFormat, &RenderDepthClear)>::new(&mut context.world);
        // let query2 = QueryState::<GameObject, &RenderSize>::new(&mut context.world);
        // let query3 = QueryState::<GameObject, &RenderColorFormat>::new(&mut context.world);
        // let query4 = QueryState::<GameObject, &RenderColorClear>::new(&mut context.world);
        // let query5 = QueryState::<GameObject, &RenderDepthFormat>::new(&mut context.world);
        // let query6 = QueryState::<GameObject, &RenderDepthClear>::new(&mut context.world);
        //  log::debug!("SingleMainCameraOpaqueRenderNode ............. {:?}", self.renderer_id);
        if let Some((renderer , rendersize , rendercolor , rendercolorclear , renderdepth , renderdepthclear)) = query.get(&context.world, self.renderer_id) {

            let currlist: Vec<ShareTargetView> = vec![];

            let window = context.world.get_resource::<Arc<winit::window::Window>>().unwrap();
            let device = context.world.get_resource::<RenderDevice>().unwrap();
            let queue = context.world.get_resource::<RenderQueue>().unwrap();
            let pipelines = context.world.get_resource::<Share<AssetMgr<RenderRes<RenderPipeline>>>>().unwrap();
            let resources = context.world.get_resource::<SingleImageEffectResource>().unwrap();

            let width = rendersize.width();
            let height = rendersize.height();

            let renderformat = rendercolor.0;

            let need_depth = if renderdepth.0.is_some() {
                true
            } else {
                false
            };

            // let atlas_allocator = world.get_resource::<Share<AssetMgr::<RenderRes<wgpu::TextureView>>>>().unwrap();
            let atlas_allocator = context.world.get_resource::<SafeAtlasAllocator>().unwrap(); // SafeAtlasAllocator::new(device.clone(), atlas_allocator.clone(), Arc::new(HomogeneousMgr::<RenderRes<UnuseTexture>, GarbageEmpty>));
            
            let target_type = atlas_allocator.get_or_create_type(
                TargetDescriptor {
                    texture_descriptor: SmallVec::from_slice(
                        &[
                            TextureDescriptor {
                                mip_level_count: 1,
                                sample_count: 1,
                                dimension: wgpu::TextureDimension::D2,
                                format: renderformat,
                                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
                                base_mip_level: 0,
                                base_array_layer: 0,
                                array_layer_count: None,
                                view_dimension: None,
                            }
                        ]
                    ),
                    need_depth: need_depth,
                    default_width: 2048,
                    default_height: 2048,
                }
            );
            let srt = atlas_allocator.allocate(
                width,
                height,
                target_type.clone(),
                currlist.iter()
            );

            let (depth_stencil_attachment, clear_depth) = if let Some(depth) = &srt.target().depth {

                let depth_stencil_attachment = Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth.0.as_ref(),
                        depth_ops: Some(
                            wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            }
                        ),
                        stencil_ops: None,
                    }
                );
                
                let clear_depth = Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth.0.as_ref(),
                        depth_ops: Some(
                            wgpu::Operations {
                                load: wgpu::LoadOp::Clear(renderdepthclear.0),
                                store: true
                            }
                        ),
                        stencil_ops: None,
                    }
                );
                (depth_stencil_attachment, clear_depth)
            } else {
                (None, None)
            };
            {
                let renderpass = commands.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[
                            Some(
                                wgpu::RenderPassColorAttachment {
                                    view: &srt.target().colors[0].0,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(rendercolorclear.0.clone()),
                                        store: true
                                    }
                                }
                            )
                        ],
                        depth_stencil_attachment: clear_depth
                    }
                );
            }

            let mut vx = 0.;
            let mut vy = 0.;
            let mut vw = 0.;
            let mut vh = 0.;

            let surface = context.world.get_resource::<ScreenTexture>().unwrap();
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
                        view: srt.target().colors[0].0.as_ref(),
                    }
                )
            );

            vw = width as f32;
            vh = height as f32;

            {
                let mut renderpass = commands.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("RenderNode"),
                        color_attachments: color_attachments.as_slice(),
                        depth_stencil_attachment: depth_stencil_attachment,
                    }
                );

                let (mut x, mut y, mut w, mut h, min_depth, max_depth) = renderer.draws.viewport;

                x = vw * x + vx;
                y = vh * y + vy;
                w = vw * w;
                h = vh * h;

                renderpass.set_viewport(x, y, w, h, 0., max_depth);
                renderpass.set_scissor_rect(x as u32, y as u32, w as u32, h as u32);
                log::warn!("Draws: {:?}", renderer.draws.list.len());
                DrawList::render(renderer.draws.list.as_slice(), &mut renderpass);
            }

            let width = window.inner_size().width;
            let height = window.inner_size().height;

            if let Some(dst) = surface.texture() {
                let dst = dst.texture.create_view(
                    &wgpu::TextureViewDescriptor {
                        label: None,
                        format: Some(wgpu::TextureFormat::Bgra8Unorm),
                        dimension: Some(wgpu::TextureViewDimension::D2),
                        aspect: wgpu::TextureAspect::All,
                        base_mip_level: 0,
                        mip_level_count: None,
                        base_array_layer: 0,
                        array_layer_count: None,
                    }
                );
                let source = PostprocessTexture::from_share_target(srt, renderformat);
                let target = PostprocessTexture {
                    use_x: 0,
                    use_y: 0,
                    use_w: width,
                    use_h: height,
                    width,
                    height,
                    view: ETextureViewUsage::Temp(Arc::new(dst), 0),
                    format: wgpu::TextureFormat::Bgra8Unorm,
                };
                let param = CopyIntensity::default();
                let (draw, target) = EffectCopy::ready(
                    param, resources,
                    device, queue, 0,
                    (width, height), &IDENTITY_MATRIX, source.get_tilloff(), 1., 0.,
                    source, Some(target), atlas_allocator, target_type, pipelines,
                    wgpu::ColorTargetState { format: wgpu::TextureFormat::Bgra8Unorm, blend: None, write_mask: wgpu::ColorWrites::ALL }, None
                ).unwrap();
                draw.draw(Some(&mut commands), None);
                // commands.copy_texture_to_texture(srt.target().colors[0].1.as_image_copy(), dst.texture.as_image_copy(), wgpu::Extent3d { width, height, depth_or_array_layers: 1 });
            }
            
            // {
            //     let mut output = RendererGraphicParam::default();
            //     let surface = context.world.get_resource::<ScreenTexture>().unwrap();
            //     let ops = wgpu::Operations {
            //         load: wgpu::LoadOp::Load,
            //         store: true,
            //     };
            //     let mut color_attachments = vec![];
            //     color_attachments.push(
            //         Some(
            //             wgpu::RenderPassColorAttachment {
            //                 resolve_target: None,
            //                 ops,
            //                 view: surface.view.as_ref().unwrap(),
            //             }
            //         )
            //     );

            //     let mut renderpass = commands.begin_render_pass(
            //         &wgpu::RenderPassDescriptor {
            //             label: Some("RenderNode"),
            //             color_attachments: color_attachments.as_slice(),
            //             depth_stencil_attachment: None,
            //         }
            //     );

            //     let (mut x, mut y, mut w, mut h, min_depth, max_depth) = renderer.draws.viewport;

            //     // x = vw * x + vx;
            //     // y = vh * y + vy;
            //     // w = vw * w;
            //     // h = vh * h;

            //     renderpass.set_viewport(0., 0., width as f32, height as f32, min_depth, max_depth);
            //     DrawList::render(renderer.draws.list.as_slice(), &mut renderpass);
            // }

            // // To Screen
            // {
            //     let surface = world.get_resource::<ScreenTexture>().unwrap();
            // }

            output.srt = None;
        } else {
            
        };

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