
use std::ops::{Deref, DerefMut};

use pi_scene_shell::prelude::*;
use pi_futures::BoxFuture;
use wgpu::StoreOp;

use crate::pass::PassTagOrders;

use super::renderer::*;

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

#[derive(Clone)]
pub struct RendererGraphicDesc {
    pub pre: Option<Entity>,
    pub curr: String,
    pub next: Option<Entity>,
    pub passorders: PassTagOrders,
}

#[derive(SystemParam)]
pub struct QueryParam<'w, 's> (
    // Res<'w, PiRenderWindow>,
    // Res<'w, PiRenderDevice>,
    // Res<'w, PiRenderQueue>,
    Res<'w, PiScreenTexture>,
    Res<'w, PiSafeAtlasAllocator>,
    Query<
        'w,
        's,
        (
            &'static RendererParam, &'static DisposeReady, &'static Renderer,
            &'static RendererRenderTarget,
        ),
    >,
);

#[derive(SystemParam)]
pub struct QueryParam0<'w, 's> (
    Res<'w, PiSafeAtlasAllocator>,
    Query<
        'w,
        's,
        (
            &'static RendererParam, &'static DisposeReady, &'static Renderer,
            &'static RendererRenderTarget,
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

    type BuildParam = QueryParam0<'static, 'static>;
    type RunParam = QueryParam<'static, 'static>;

    fn build<'a>(
        &'a mut self,
        world: &'a mut World,
        param: &'a mut SystemState<Self::BuildParam>,
        _context: RenderContext,
        input: &'a Self::Input,
        _usage: &'a ParamUsage,
        _id: NodeId,
        _from: &'a [NodeId],
        _to: &'a [NodeId],
    ) -> Result<Self::Output, String> {
        
        let mut output = SimpleInOut::default();

        let mut param: QueryParam0 = param.get_mut(world);
        let (atlas_allocator, mut query) = (param.0, param.1);
        if let Ok((
            param, disposed, renderer, mut to_final_target
        )) = query.get_mut(self.renderer_id) {
    
            // log::warn!("Draws: Graphic {:?}", (enable.0, depth_clear, auto_clear_depth));
            if !param.enable.0 || disposed.0 {
                return Ok(output);
            }
    
            // let (mut x, mut y, mut w, mut h, min_depth, max_depth) = renderer.draws.viewport;
            let need_depth = param.depthstencilformat.need_depth();
            
            // let clear_color_ops = if auto_clear_color.0 {
            //     wgpu::Operations { load: wgpu::LoadOp::Clear(color_clear.color()), store: StoreOp::Store }
            // } else {
            //     wgpu::Operations { load: wgpu::LoadOp::Load, store: StoreOp::Discard }
            // };
            // let clear_depth_ops = if auto_clear_depth.0 {
            //     Some(wgpu::Operations { load: wgpu::LoadOp::Clear(depth_clear.0), store: StoreOp::Store, })
            // } else { None };
            // let clear_stencil_ops = if auto_clear_stencil.0 {
            //     Some(wgpu::Operations { load: wgpu::LoadOp::Clear(stencil_clear.0), store: StoreOp::Store, })
            // } else {
            //     None
            // };

            // let color_view = to_final_target.view();
            // let depth_view = to_final_target.depth_view();
            
            // let can_render: bool;
            // let clear_color_attachments;
            // let color_attachments;
            // let clear_depth_stencil_attachment;
            // let depth_stencil_attachment;
            // let render_color_view;
            // let render_depth_view;
            let to_final_target = to_final_target.deref_mut();

            match to_final_target {
                RendererRenderTarget::FinalRender => {},
                RendererRenderTarget::Custom(_srt) => output.target = input.target.clone(),
                RendererRenderTarget::None(val) => {
                    let currlist: Vec<ShareTargetView> = vec![];
                    let srt = if let Some(srt) = input.target.clone() {
                        match (param.depthstencilformat.0.val(), &srt.target().depth) {
                            (Some(format), Some(depthview)) => {
                                if depthview.1.format() == format {
                                    Some(srt)
                                } else { None }
                            },
                            (None, _) => { Some(srt) },
                            _ => { None }
                        }
                    } else {
                        None
                    };
                    let srt = match srt {
                        Some(srt) => {
                            if srt.target().colors[0].1.format() == param.colorformat.0.val() {
                                Some(srt)
                            } else {
                                None
                            }
                        },
                        None => { None },
                    };

                    let srt = if let Some(srt) = srt {
                        // log::warn!("SRT From Input.");
                        srt
                    } else {
                        // log::warn!("SRT Allocate by allocate.");
                        let width = param.rendersize.width();
                        let height = param.rendersize.height();
                        let target_type = atlas_allocator.get_or_create_type(
                            TargetDescriptor {
                                colors_descriptor: param.colorformat.desc(),
                                need_depth: need_depth, 
                                default_width: 2048,
                                default_height: 2048,
                                depth_descriptor: param.depthstencilformat.desc()
                            }
                        );

                        atlas_allocator.allocate( width, height, target_type.clone(), currlist.iter() )
                    };

                    *val = Some(srt.clone());
                    output.target = Some(srt.clone());
                },
            };
        }
        return Ok(output);
    }

    fn run<'a>(
        &'a mut self,
        world: &'a World,
        param: &'a mut SystemState<Self::RunParam>,
        _: RenderContext,
        mut commands: ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        _: &'a ParamUsage,
		_id: NodeId,
		_from: &[NodeId],
		_to: &[NodeId],
    ) -> BoxFuture<'a, Result<(), String>> {
        // let time = pi_time::Instant::now();

        let mut output = SimpleInOut::default();

        let param: QueryParam = param.get(world);
        let (screen, _atlas_allocator, query) = (param.0, param.1, param.2);
        if let Ok((
            param, disposed, renderer, to_final_target
        )) = query.get(self.renderer_id) {
            // query.
    
            // log::warn!("Draws: Graphic {:?}", (enable.0, depth_clear, auto_clear_depth));
            if !param.enable.0 || disposed.0 {
                return Box::pin(
                    async move {
                        Ok(())
                    }
                );
            }
    
            let (mut x, mut y, mut w, mut h, min_depth, max_depth) = renderer.draws.viewport;
            let need_depth = param.depthstencilformat.need_depth();
            
            let clear_color_ops = if param.auto_clear_color.0 {
                wgpu::Operations { load: wgpu::LoadOp::Clear(param.color_clear.color()), store: StoreOp::Store }
            } else {
                wgpu::Operations { load: wgpu::LoadOp::Load, store: StoreOp::Discard }
            };
            let clear_depth_ops = if param.auto_clear_depth.0 {
                Some(wgpu::Operations { load: wgpu::LoadOp::Clear(param.depth_clear.0), store: StoreOp::Store, })
            } else { None };
            let clear_stencil_ops = if param.auto_clear_stencil.0 {
                Some(wgpu::Operations { load: wgpu::LoadOp::Clear(param.stencil_clear.0), store: StoreOp::Store, })
            } else {
                None
            };

            // let color_view = to_final_target.view();
            let depth_view = to_final_target.depth_view();
            
            let can_render: bool;
            let clear_color_attachments;
            let color_attachments;
            let clear_depth_stencil_attachment;
            let depth_stencil_attachment;
            let render_color_view;
            let render_depth_view;

            match &to_final_target {
                RendererRenderTarget::FinalRender => {
                    // log::warn!("Graphic: FinalRender");
                    if let Some(screen) = &screen.0 {
                        match (&screen.view, screen.texture()) {
                            (Some(view), Some(texture)) => {
                                can_render = true;
                                let width = texture.texture.width();
                                let height = texture.texture.height();
                                x = width as f32 * x;
                                y = height as f32 * y;
                                w = width as f32 * w;
                                h = height as f32 * h;
                                render_color_view = view.deref();
                                render_depth_view = None;
                            },
                            _ => {
                                return Box::pin( async move { Ok(()) } );
                            },
                        }
                    } else {
                        return Box::pin( async move { Ok(()) } );
                    }
                },
                RendererRenderTarget::Custom(srt) => {
                    // log::warn!("Graphic: Custom");
                    let width = param.rendersize.width();
                    let height = param.rendersize.height();
                    x = srt.rect().min.x as f32 + width as f32 * x;
                    y = srt.rect().min.y as f32 + height as f32 * y;
                    w = width as f32 * w;
                    h = height as f32 * h;
                    can_render = need_depth == depth_view.is_some();
                    let view = srt.target().colors[0].0.as_ref();
                    render_color_view = view.deref().deref();

                    if let Some(view) = srt.target().depth.as_ref() {
                        let depth_view = view.0.as_ref();
                        render_depth_view = Some(depth_view.deref().deref());
                    } else {
                        render_depth_view = None;
                    };
                    
                    // output.target = Some(srt.clone());
                },
                RendererRenderTarget::None(srt) => {
                    let srt = match srt {
                        Some(srt) => {
                            if srt.target().colors[0].1.format() == param.colorformat.0.val() {
                                srt
                            } else {
                                return Box::pin( async move { Ok(()) } );
                            }
                        },
                        None => {
                            return Box::pin( async move { Ok(()) } );
                        },
                    };
                    let width = srt.rect().max.x - srt.rect().min.x;
                    let height = srt.rect().max.y - srt.rect().min.y;
                    x = srt.rect().min.x as f32 + width as f32 * x;
                    y = srt.rect().min.y as f32 + height as f32 * y;
                    w = width as f32 * w;
                    h = height as f32 * h;
                    // can_render = true;

                    output.target = Some(srt.clone());

                    let view = output.target.as_ref().unwrap().target().colors[0].0.as_ref();
                    render_color_view = view.deref().deref();

                    if let Some(view) = output.target.as_ref().unwrap().target().depth.as_ref() {
                        let depth_view = view.0.as_ref();
                        render_depth_view = Some(depth_view.deref().deref());
                    } else {
                        render_depth_view = None;
                    };

                    can_render = true;
                },
            };

            // log::warn!("Graphic: {:?}", (can_render, renderer.draws.list.len()));
            if can_render {
                clear_color_attachments = [
                    Some( wgpu::RenderPassColorAttachment { view: render_color_view, resolve_target: None, ops: clear_color_ops, } )
                ];
                color_attachments = [
                    Some( wgpu::RenderPassColorAttachment { resolve_target: None,  ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: StoreOp::Store, }, view: render_color_view, })
                ];
                if let Some(depth) = render_depth_view {
                    depth_stencil_attachment = Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: depth,
                            depth_ops: Some(
                                wgpu::Operations { load: wgpu::LoadOp::Load, store: StoreOp::Store, }
                            ),
                            stencil_ops: Some( wgpu::Operations { load: wgpu::LoadOp::Load, store: StoreOp::Store, } ),
                        }
                    );
                    clear_depth_stencil_attachment = Some(
                        wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: clear_depth_ops, stencil_ops: clear_stencil_ops, }
                    );
                } else {
                    clear_depth_stencil_attachment = None;
                    depth_stencil_attachment = None;
                };

                if param.auto_clear_color.0 || param.auto_clear_depth.0 || param.auto_clear_stencil.0 {
                    let mut renderpass = commands.begin_render_pass(
                        &wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: clear_color_attachments.as_slice(),
                            depth_stencil_attachment: clear_depth_stencil_attachment,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        }
                    );
                    renderpass.set_viewport(x, y, w, h, min_depth, max_depth);
                    renderpass.set_scissor_rect(x as u32, y as u32, w as u32, h as u32);
                }

                // log::warn!("Draws: {:?}", renderer.draws.list.len());
                if renderer.draws.list.len() > 0 {
                    let mut renderpass = commands.begin_render_pass(
                        &wgpu::RenderPassDescriptor {
                            label: Some(self.renderer_id.index().to_string().as_str()),
                            color_attachments: color_attachments.as_slice(),
                            depth_stencil_attachment: depth_stencil_attachment,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        }
                    );
        
                    renderpass.set_viewport(x, y, w, h, 0., max_depth);
                    renderpass.set_scissor_rect(x as u32, y as u32, w as u32, h as u32);
                    DrawList::render(renderer.draws.list.as_slice(), &mut renderpass);
    
                    // let time1 = pi_time::Instant::now();
                    // log::debug!("MainCameraRenderNode: {:?}", time1 - time);
                }
            }
        }
        
        return Box::pin( async move { Ok(()) } );
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