
use pi_bevy_render_plugin::component::GraphId;
use pi_scene_shell::prelude::*;

use crate::{
    viewer::prelude::*,
    postprocess::*,
};

use super::{
    renderer::*,
    render_object::RendererID,
    graphic::*,
    command::*,
};

pub fn sys_create_renderer(
    mut commands: Commands,
    mut cmds: ResMut<ActionListRendererCreate>,
    mut graphic: ResMut<PiRenderGraph>,
    mut viewers: Query<(&mut ViewerRenderersInfo, &mut DirtyViewerRenderersInfo)>,
    mut error: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|OpsRendererCreate(entity, name, id_viewer, passtag, transparent)| {
        if let Ok((mut viewerrenderinfo, mut viewerflag)) = viewers.get_mut(id_viewer) {
            let render_node = RenderNode::new(entity);
            match graphic.add_node(name, render_node) {
                Ok(nodeid) => {
                    if let Some(mut cmd) = commands.get_entity(entity) {
                        cmd.insert(GraphId(nodeid));
                        ActionRenderer::init(&mut cmd, id_viewer, passtag, transparent);
                        viewerrenderinfo.add(entity, passtag);
                        *viewerflag = DirtyViewerRenderersInfo;
                    }
                },
                Err(err) => {
                    // log::error!("CreateRenderer Fail Graphic Error");
                    error.graphic(entity, err);
                },
            }
        } else {
            // log::error!("CreateRenderer Fail Not Found Viewer");
        }
    });
}

pub fn sys_act_renderer_target(
    mut cmds: ResMut<ActionListRendererTarget>,
    mut renderers: Query<(&mut RenderSize, &mut RenderColorFormat, &mut RenderDepthFormat, &mut RendererRenderTarget, &GraphId)>,
    targets: Res<CustomRenderTargets>,
    mut graphic: ResMut<PiRenderGraph>,
    mut error: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsRendererTarget::Custom(entity, keytarget) => {
                if let Ok((mut rendersize, mut color_format, mut depth_stencil_format, mut rendertarget, nodeid)) = renderers.get_mut(entity) {

                    match keytarget {
                        KeyCustomRenderTarget::Custom(key) => {
                            if let Some(srt) = targets.get(key) {
                                *rendersize = RenderSize::new(srt.width, srt.height);
                                *rendertarget = RendererRenderTarget::Custom(srt.rt.clone());
                                *color_format = RenderColorFormat(srt.color_format);
                                *depth_stencil_format = RenderDepthFormat(srt.depth_stencil_format);
                                // log::warn!("sys_act_renderer_target Custom {:?}", srt.color_format);
                            } else {
                                *rendertarget = RendererRenderTarget::None;
                            }
                            if let Err(err) = graphic.set_finish(nodeid.0, false) {
                                error.graphic(entity, err);
                            }
                        },
                        KeyCustomRenderTarget::FinalRender => {
                            let format = match ColorFormat::new(wgpu::TextureFormat::pi_render_default()) {
                                Some(format) => format,
                                _ => ColorFormat::Rgba8Unorm
                            };
    
                            *color_format = RenderColorFormat(format);
                            *depth_stencil_format = RenderDepthFormat(DepthStencilFormat::None);
                            *rendertarget = RendererRenderTarget::FinalRender;
    
                            if let Err(err) = graphic.set_finish(nodeid.0, true) {
                                error.graphic(entity, err);
                            }
                        },
                    }
                };
            },
            OpsRendererTarget::Auto(entity, width, height, colorformat, depthstencilformat) => {
                if let Ok((mut rendersize, mut color_format, mut depth_stencil_format, mut rendertarget, _nodeid)) = renderers.get_mut(entity) {
                    *rendersize = RenderSize::new(width as u32, height as u32);
                    *color_format = RenderColorFormat(colorformat);
                    *depth_stencil_format = RenderDepthFormat(depthstencilformat);
                    *rendertarget = RendererRenderTarget::None;
                }
            },
        }
    });
}

pub fn sys_renderer_modify(
    mut cmds: ResMut<ActionListRendererModify>,
    mut enables: Query<&mut RendererEnable>,
    // mut rendersizes: Query<&mut RenderSize>,
    mut colorclear: Query<&mut RenderColorClear>,
    // mut colorformat: Query<&mut RenderColorFormat>,
    mut depthclear: Query<&mut RenderDepthClear>,
    // mut depthformat: Query<&mut RenderDepthFormat>,
    mut stencilclear: Query<&mut RenderStencilClear>,
    mut autoclearcolor: Query<&mut RenderAutoClearColor>,
    mut autocleardepth: Query<&mut RenderAutoClearDepth>,
    mut autoclearstencil: Query<&mut RenderAutoClearStencil>,
    mut viewport: Query<&mut RenderViewport>,
    mut renderblend: Query<&mut RendererBlend>,
    // mut tofinals: Query<&mut RendererRenderTarget>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsRendererCommand::Active(entity, val) => {
                if let Ok(mut comp) = enables.get_mut(entity) {
                    *comp = RendererEnable(val);
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::Blend(entity, val) => {
                if let Ok(mut comp) = renderblend.get_mut(entity) {
                    *comp = RendererBlend(val);
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::ColorClear(entity, val) => {
                if let Ok(mut comp) = colorclear.get_mut(entity) {
                    *comp = val;
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::DepthClear(entity, val) => {
                if let Ok(mut comp) = depthclear.get_mut(entity) {
                    *comp = val;
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::StencilClear(entity, val) => {
                if let Ok(mut comp) = stencilclear.get_mut(entity) {
                    *comp = val;
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::AutoClearColor(entity, val) => {
                if let Ok(mut comp) = autoclearcolor.get_mut(entity) {
                    *comp = RenderAutoClearColor(val);
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::AutoClearDepth(entity, val) => {
                if let Ok(mut comp) = autocleardepth.get_mut(entity) {
                    *comp = RenderAutoClearDepth(val);
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::AutoClearStencil(entity, val) => {
                if let Ok(mut comp) = autoclearstencil.get_mut(entity) {
                    *comp = RenderAutoClearStencil(val);
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::Viewport(entity, x, y, z, w) => {
                if let Ok(mut comp) = viewport.get_mut(entity) {
                    *comp = RenderViewport(x, y, z, w, 0., 1.);
                } else { cmds.push(cmd) }
            }
        }
    });
}

pub fn sys_act_renderer_connect(
    mut cmds: ResMut<ActionListRendererConnect>,
    mut render_graphic: ResMut<PiRenderGraph>,
    renderers: Query<&GraphId>,
    mut error: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|OpsRendererConnect(before, after, isdisconnect)| {
        if let (Ok(nbefore), Ok(nafter)) = (renderers.get(before), renderers.get(after)) {
            if isdisconnect {
                if let Err(err) = render_graphic.remove_depend(nbefore.0, nafter.0) {
                    error.graphic(before, err);
                }
            } else {
                if let Err(err) = render_graphic.add_depend(nbefore.0, nafter.0) {
                    error.graphic(before, err);
                }
            }
            render_graphic.dump_graphviz();
        }
    });
}

pub fn sys_dispose_renderer(
    mut render_graphic: ResMut<PiRenderGraph>,
    renderers: Query<(Entity, &GraphId, &RendererEnable, &DisposeCan, &ViewerID), Changed<DisposeCan>>,
    mut viewers: Query<&mut ViewerRenderersInfo>,
    mut error: ResMut<ErrorRecord>,
) {
    renderers.iter().for_each(|(entity, nodeid, _, flag, idviewer)| {
        if flag.0 == false { return; }
        
        if let Err(err) = render_graphic.remove_node(nodeid.0) {
            error.graphic(entity, err);
        }
        if let Ok(mut renderinfos) = viewers.get_mut(idviewer.0) {
            renderinfos.remove(entity);
        }
    });
}

pub struct ActionRenderer;
impl ActionRenderer {
    pub(crate) fn init(
        commands_renderer: &mut EntityCommands,
        id_viewer: Entity,
        passtag: PassTag,
        transparent: bool,
    ) {
        ActionEntity::init(commands_renderer);
        commands_renderer
            .insert(passtag)
            .insert(Renderer::new())
            .insert(RenderViewport::default())
            .insert(RenderSize::new(100, 100))
            .insert(RendererEnable(true))
            .insert(RenderColorClear::default())
            .insert(RenderColorFormat::default())
            .insert(RenderDepthClear::default())
            .insert(RenderDepthFormat::default())
            .insert(RenderStencilClear::default())
            .insert(RenderAutoClearColor::default())
            .insert(RenderAutoClearDepth::default())
            .insert(RenderAutoClearStencil::default())
            .insert(RendererRenderTarget::None)
            .insert(RendererBlend(transparent))
            .insert(ViewerID(id_viewer))
            .insert(Postprocess::default())
            ;
    }
    pub fn create_graphic_node(
        commands: &mut Commands,
        render_graphic: &mut PiRenderGraph,
        error: &mut ErrorRecord,
        name: String,
    ) -> Entity {
        let entity = commands.spawn_empty().id();
        let render_node = RenderNode::new(entity);
        match render_graphic.add_node(name, render_node) {
            Ok(nodeid) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(GraphId(nodeid));  
                }
            },
            Err(err) => {
                error.graphic(entity, err);
            },
        }

        entity
    }
    pub fn apply_graph_id(
        entitycmd: &mut EntityCommands,
        node: NodeId,
    ) {
        entitycmd.insert(GraphId(node));
    }
    pub fn init_graphic_node(
        render_graphic: &mut PiRenderGraph,
        error: &mut ErrorRecord,
        _id_renderer: RendererID,
        nodeid: NodeId,
        pre: Option<NodeId>,
        next: Option<NodeId>,
    ) {
        if let Some(key_pre) = pre {
            // log::warn!("Add Node {:?} > {:?}", key_pre, nodeid);
            if let Err(err) = render_graphic.add_depend(key_pre, nodeid) {
                error.graphic(_id_renderer.0, err);
            }
        }
        if let Some(key_next) = next {
            // log::warn!("Add Node {:?} > {:?}", nodeid, key_next);
            if let Err(err) = render_graphic.add_depend(nodeid, key_next) {
                error.graphic(_id_renderer.0, err);
            }
        } else {
            // if let Err(e) = render_graphic.set_finish(nodeid, true) {
            //     log::debug!("{:?}", e);
            // }
        }
        // render_graphic.dump_graphviz();
    }
}
