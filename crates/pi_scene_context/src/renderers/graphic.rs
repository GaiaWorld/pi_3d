

use pi_atom::Atom;
use pi_bevy_render_plugin::SimpleInOut;
use pi_engine_shell::prelude::*;
use pi_futures::BoxFuture;
use pi_share::{ShareRefCell};

use crate::{renderers::{renderer::*}, pass::PassTagOrders};


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

#[derive(Debug, Clone)]
pub struct RendererGraphicDesc {
    pub pre: Option<Entity>,
    pub curr: String,
    pub next: Option<Entity>,
    pub passorders: PassTagOrders,
}

#[derive(SystemParam)]
pub struct QueryParam<'w, 's> (
    Res<'w, PiRenderWindow>,
    Res<'w, PiRenderDevice>,
    Res<'w, PiRenderQueue>,
    Res<'w, WindowRenderer>,
    Res<'w, PiSafeAtlasAllocator>,
    Query<
        'w,
        's,
        (
            &'static RendererEnable, &'static Renderer, &'static RenderSize,
            &'static RenderColorFormat, &'static RenderColorClear,
            &'static RenderDepthFormat, &'static RenderDepthClear,
            &'static RenderStencilClear,
            &'static RenderAutoClearColor,&'static RenderAutoClearDepth, &'static RenderAutoClearStencil,
            &'static RenderToFinalTarget
        ),
    >,
);

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
    type Input = SimpleInOut;

    type Output = SimpleInOut;

    type Param = QueryParam<'static, 'static>;

    fn run<'a>(
        &'a mut self,
        world: &'a World,
        param: &'a mut SystemState<Self::Param>,
        _: RenderContext,
        mut commands: ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        _: &'a ParamUsage,
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        let time = pi_time::Instant::now();

        let mut output = SimpleInOut::default();

        // let window = world.get_resource::<RenderWindow>().unwrap();

        // let query = QueryState::<(&Renderer, &RenderSize, &RenderColorFormat, &RenderColorClear, &RenderDepthFormat, &RenderDepthClear)>::from_world(world);
        // let query2 = QueryState::<GameObject, &RenderSize>::new(&mut context.world);
        // let query3 = QueryState::<GameObject, &RenderColorFormat>::new(&mut context.world);
        // let query4 = QueryState::<GameObject, &RenderColorClear>::new(&mut context.world);
        // let query5 = QueryState::<GameObject, &RenderDepthFormat>::new(&mut context.world);
        // let query6 = QueryState::<GameObject, &RenderDepthClear>::new(&mut context.world);
        //  log::debug!("SingleMainCameraOpaqueRenderNode ............. {:?}", self.renderer_id);
        // if let Some((renderer , rendersize , rendercolor , rendercolorclear , renderdepth , renderdepthclear)) = query.get(&context.world, self.renderer_id) {

        // log::warn!("Draws: Graphic");

        let param: QueryParam = param.get(world);
        let (window, device, queue, final_render_target, atlas_allocator, query) = (param.0, param.1, param.2, param.3, param.4, param.5);
        if let Ok((
            enable, renderer, rendersize, format, color_clear, depth, depth_clear, stencil_clear, auto_clear_color, auto_clear_depth, auto_clear_stencil, to_final_target
        )) = query.get(self.renderer_id) {
            // query.
    
            // log::warn!("Draws: Graphic {:?}", enable.0);
            if !enable.0 {
                return Box::pin(
                    async move {
                        Ok(output)
                    }
                );
            }
    
            let (mut x, mut y, mut w, mut h, min_depth, max_depth) = renderer.draws.viewport;
            let need_depth = depth.need_depth();
            
            let clear_color_ops = if auto_clear_color.0 {
                wgpu::Operations { load: wgpu::LoadOp::Clear(color_clear.color()), store: true }
            } else {
                wgpu::Operations { load: wgpu::LoadOp::Load, store: false }
            };
            let clear_depth_ops = if auto_clear_depth.0 {
                Some(wgpu::Operations { load: wgpu::LoadOp::Clear(depth_clear.0), store: true, })
            } else { None };
            let clear_stencil_ops = if auto_clear_stencil.0 {
                Some(wgpu::Operations { load: wgpu::LoadOp::Clear(stencil_clear.0), store: true, })
            } else {
                None
            };
    
            // log::warn!("Draws: to_final_target {:?}", to_final_target.0);
            if to_final_target.0 {
                let width = rendersize.width();
                let height = rendersize.height();
                x = width as f32 * x;
                y = height as f32 * y;
                w = width as f32 * w;
                h = height as f32 * h;
                if let Some(view) =  final_render_target.view() {
                    // let mut vx = 0;
                    // let mut vy = 0;
                    // let mut vw = final_render_target.;
                    // let mut vh = srt.rect().max.y - vy;
                    // x = vw as f32 * x + vx;
                    // y = vh as f32 * y + vy;
                    // w = vw as f32 * w;
                    // h = vh as f32 * h;
    
                    // Clear
                    if auto_clear_color.0 || auto_clear_depth.0 || auto_clear_stencil.0 {
                        let mut renderpass = commands.begin_render_pass(
                            &wgpu::RenderPassDescriptor {
                                label: None,
                                color_attachments: &[
                                    Some(
                                        wgpu::RenderPassColorAttachment {
                                            view: view,
                                            resolve_target: None,
                                            ops: clear_color_ops,
                                        }
                                    )
                                ],
                                depth_stencil_attachment: Some(
                                    wgpu::RenderPassDepthStencilAttachment {
                                        view: final_render_target.depth_view().unwrap(),
                                        depth_ops: clear_depth_ops,
                                        stencil_ops: clear_stencil_ops,
                                    }
                                )
                            }
                        );
                        renderpass.set_viewport(x, y, w, h, min_depth, max_depth);
                        renderpass.set_scissor_rect(x as u32, y as u32, w as u32, h as u32);
                    }
    
                    if renderer.draws.list.len() > 0 {
                        let mut color_attachments = vec![];
                        color_attachments.push(
                            Some(
                                wgpu::RenderPassColorAttachment {
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Load,
                                        store: true,
                                    },
                                    view: view,
                                }
                            )
                        );
            
                        let mut renderpass = commands.begin_render_pass(
                            &wgpu::RenderPassDescriptor {
                                label: Some("RenderNode"),
                                color_attachments: color_attachments.as_slice(),
                                depth_stencil_attachment: Some(
                                    wgpu::RenderPassDepthStencilAttachment {
                                        view: final_render_target.depth_view().unwrap(),
                                        depth_ops: Some(
                                            wgpu::Operations {
                                                load: wgpu::LoadOp::Load,
                                                store: true,
                                            }
                                        ),
                                        stencil_ops: Some(
                                            wgpu::Operations {
                                                load: wgpu::LoadOp::Load,
                                                store: true,
                                            }
                                        ),
                                    }
                                ),
                            }
                        );
            
                        renderpass.set_viewport(x, y, w, h, 0., max_depth);
                        renderpass.set_scissor_rect(x as u32, y as u32, w as u32, h as u32);
                        // log::warn!("Draws: {:?}", renderer.draws.list.len());
                        DrawList::render(renderer.draws.list.as_slice(), &mut renderpass);
        
                        let time1 = pi_time::Instant::now();
                        // log::debug!("MainCameraRenderNode: {:?}", time1 - time);
                    }
            
                    Box::pin(
                        async move {
                            Ok(output)
                        }
                    )
                } else {
                    let time1 = pi_time::Instant::now();
                    // log::debug!("MainCameraRenderNode: {:?}", time1 - time);
            
                    Box::pin(
                        async move {
                            Ok(output)
                        }
                    )
                }
            } else {
                let currlist: Vec<ShareTargetView> = vec![];
                let srt = if let Some(srt) = input.target.clone() {
                    if srt.target().depth.is_none() && need_depth {
                        // log::warn!("Render Input Not Get Depth!");
                        None
                    } else {
                        Some(srt)
                    }
                } else {
                    None
                };
                let srt = if let Some(srt) = srt {
                    srt
                } else {
                    let width = rendersize.width();
                    let height = rendersize.height();
                    let target_type = atlas_allocator.get_or_create_type(
                        TargetDescriptor {
                            colors_descriptor: format.desc(),
                            need_depth: need_depth,
                            default_width: 2048,
                            default_height: 2048,
                            depth_descriptor: depth.desc()
                        }
                    );
                    
                    // log::warn!("Render Size: {:?}", (format.desc(), depth.desc()));
                    atlas_allocator.allocate(
                        width,
                        height,
                        target_type.clone(),
                        currlist.iter()
                    )
                };
                let vx = srt.rect().min.x;
                let vy = srt.rect().min.y;
                let vw = srt.rect().max.x - srt.rect().min.x;
                let vh = srt.rect().max.y - vy;
                x = vw as f32 * x + vx as f32;
                y = vh as f32 * y + vy as f32;
                w = vw as f32 * w;
                h = vh as f32 * h;
                // log::warn!("Render Size: {:?}", (x, y, w, h));
    
                let (
                    depth_stencil_attachment,
                    clear_depth
                ) = if let Some(depth) = &srt.target().depth {
                    let depth_stencil_attachment = Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: depth.0.as_ref(),
                            depth_ops: Some(
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                }
                            ),
                            stencil_ops: Some(
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                }
                            ),
                        }
                    );
                    let clear_depth = Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: depth.0.as_ref(),
                            depth_ops: clear_depth_ops,
                            stencil_ops: clear_stencil_ops,
                        }
                    );
                    (depth_stencil_attachment, clear_depth)
                } else {
                    (None, None)
                };

                // log::warn!("Render color: {:?}", srt.target().colors);
                // log::warn!("Render depth: {:?}", srt.target().depth);
    
                if auto_clear_color.0 || (auto_clear_depth.0 || auto_clear_stencil.0 && srt.target().depth.is_some()) {
                    let mut renderpass = commands.begin_render_pass(
                        &wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[
                                Some(
                                    wgpu::RenderPassColorAttachment {
                                        view: &srt.target().colors[0].0,
                                        resolve_target: None,
                                        ops: clear_color_ops
                                    }
                                )
                            ],
                            depth_stencil_attachment: clear_depth
                        }
                    );
                    renderpass.set_viewport(x, y, w, h, 0., max_depth);
                    renderpass.set_scissor_rect(x as u32, y as u32, w as u32, h as u32);
                }
                
                if renderer.draws.list.len() > 0 {
                    let mut color_attachments = vec![];
                    color_attachments.push(
                        Some(
                            wgpu::RenderPassColorAttachment {
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                },
                                view: srt.target().colors[0].0.as_ref(),
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
        
                    renderpass.set_viewport(x, y, w, h, 0., max_depth);
                    renderpass.set_scissor_rect(x as u32, y as u32, w as u32, h as u32);
                    // log::warn!("Draws: {:?}", renderer.draws.list.len());
                    DrawList::render(renderer.draws.list.as_slice(), &mut renderpass);
        
                    let time1 = pi_time::Instant::now();
                    // log::debug!("MainCameraRenderNode: {:?}", time1 - time);
                }
    
                output.target = Some(srt.clone());
        
                Box::pin(
                    async move {
                        Ok(output)
                    }
                )
            }
        } else {
            Box::pin(
                async move {
                    Ok(output)
                }
            )
        }

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