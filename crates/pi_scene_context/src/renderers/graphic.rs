
use std::ops::Deref;

use pi_bevy_render_plugin::SimpleInOut;
use pi_engine_shell::prelude::*;
use pi_futures::BoxFuture;
use pi_share::ShareRefCell;

use crate::{
    pass::PassTagOrders,
    commands::DisposeReady
};

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

#[derive(Debug, Clone)]
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
            &'static RendererEnable, &'static DisposeReady, &'static Renderer, &'static RenderSize,
            &'static RenderColorFormat, &'static RenderColorClear,
            &'static RenderDepthFormat, &'static RenderDepthClear,
            &'static RenderStencilClear,
            &'static RenderAutoClearColor, &'static RenderAutoClearDepth, &'static RenderAutoClearStencil,
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

    type Param = QueryParam<'static, 'static>;

    fn run<'a>(
        &'a mut self,
        world: &'a World,
        param: &'a mut SystemState<Self::Param>,
        _: RenderContext,
        mut commands: ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        _: &'a ParamUsage,
		_id: NodeId,
		_from: &[NodeId],
		_to: &[NodeId],
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        // let time = pi_time::Instant::now();

        let mut output = SimpleInOut::default();

        let param: QueryParam = param.get(world);
        let (screen, atlas_allocator, query) = (param.0, param.1, param.2);
        if let Ok((
            enable, disposed, renderer, rendersize, colorformat, color_clear, depthstencilformat, depth_clear, stencil_clear, auto_clear_color, auto_clear_depth, auto_clear_stencil, to_final_target
        )) = query.get(self.renderer_id) {
            // query.
    
            // log::warn!("Draws: Graphic {:?}", enable.0);
            if !enable.0 || disposed.0 {
                return Box::pin(
                    async move {
                        Ok(output)
                    }
                );
            }
    
            let (mut x, mut y, mut w, mut h, min_depth, max_depth) = renderer.draws.viewport;
            let need_depth = depthstencilformat.need_depth();
            
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
                                return Box::pin(
                                    async move {
                                        Ok(output)
                                    }
                                );
                            },
                        }
                    } else {
                        return Box::pin(
                            async move {
                                Ok(output)
                            }
                        );
                    }
                },
                RendererRenderTarget::Custom(srt) => {
                    // log::warn!("Graphic: Custom");
                    let width = rendersize.width();
                    let height = rendersize.height();
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
                },
                RendererRenderTarget::None => {
                    // can_render = false;
                    // return Box::pin(
                    //     async move {
                    //         Ok(output)
                    //     }
                    // );

                    let currlist: Vec<ShareTargetView> = vec![];
                    let srt = if let Some(srt) = input.target.clone() {
                        match (depthstencilformat.0.val(), &srt.target().depth) {
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
                            if srt.target().colors[0].1.format() == colorformat.0.val() {
                                Some(srt)
                            } else {
                                None
                            }
                        },
                        None => {
                            None
                        },
                    };

                    let srt = if let Some(srt) = srt {
                        srt
                    } else {
                        let width = rendersize.width();
                        let height = rendersize.height();
                        let target_type = atlas_allocator.get_or_create_type(
                            TargetDescriptor { colors_descriptor: colorformat.desc(), need_depth: need_depth,  default_width: 2048,  default_height: 2048, depth_descriptor: depthstencilformat.desc() }
                        );
                        
                        // log::warn!("New RenderTarget: {:?}", (format.desc(), depth.desc()));
                        atlas_allocator.allocate( width, height, target_type.clone(), currlist.iter() )
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
                    Some( wgpu::RenderPassColorAttachment { resolve_target: None,  ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: true, }, view: render_color_view, })
                ];
                if let Some(depth) = render_depth_view {
                    depth_stencil_attachment = Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: depth,
                            depth_ops: Some(
                                wgpu::Operations { load: wgpu::LoadOp::Load, store: true, }
                            ),
                            stencil_ops: Some( wgpu::Operations { load: wgpu::LoadOp::Load, store: true, } ),
                        }
                    );
                    clear_depth_stencil_attachment = Some(
                        wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: clear_depth_ops, stencil_ops: clear_stencil_ops, }
                    );
                } else {
                    clear_depth_stencil_attachment = None;
                    depth_stencil_attachment = None;
                };

                if auto_clear_color.0 || auto_clear_depth.0 || auto_clear_stencil.0 {
                    let mut renderpass = commands.begin_render_pass(
                        &wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: clear_color_attachments.as_slice(),
                            depth_stencil_attachment: clear_depth_stencil_attachment
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
                        }
                    );
        
                    renderpass.set_viewport(x, y, w, h, 0., max_depth);
                    renderpass.set_scissor_rect(x as u32, y as u32, w as u32, h as u32);
                    DrawList::render(renderer.draws.list.as_slice(), &mut renderpass);
    
                    // let time1 = pi_time::Instant::now();
                    // log::debug!("MainCameraRenderNode: {:?}", time1 - time);
                }
        
                Box::pin(
                    async move {
                        Ok(output)
                    }
                )
            } else {
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