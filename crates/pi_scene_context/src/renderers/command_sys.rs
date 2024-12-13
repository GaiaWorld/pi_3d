use std::ops::Deref;

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
    mut cmds: ResMut<ActionListRendererCreate>,
    mut graphic: ResMut<PiRenderGraph>,
    mut viewers: Query<(&SceneID, &mut ViewerRenderersInfo, &mut DirtyViewerRenderersInfo)>,
    mut error: ResMut<ErrorRecord>,
    mut alter: Alter<(), (), (GraphId, SceneID, RendererBundle), ()>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_create_renderer"));
    cmds.drain().for_each(|OpsRendererCreate(entity, name, id_viewer, passtag, transparent)| {
        if let Ok((sceneid, mut viewerrenderinfo, mut viewerflag)) = viewers.get_mut(id_viewer) {
            let render_node = RenderNode::new(entity);
            match graphic.add_node(name, render_node, NodeId::null()) {
                Ok(nodeid) => {
                    // if let Some(mut cmd) = commands.get_entity(entity) {
                        viewerrenderinfo.add(entity, passtag);
                        *viewerflag = DirtyViewerRenderersInfo;
                        // log::error!("CreateRenderer {:?}", (nodeid, id_viewer, entity, viewerrenderinfo.len()));

                        let bundle = (
                            GraphId(nodeid), sceneid.clone(),
                            ActionRenderer::init(id_viewer, passtag, transparent)
                        );
                        // commands.entity(entity).insert(bundle);
                        let _ = alter.alter(entity, bundle);

                    // }
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

pub fn sys_act_renderer_modify(
    mut cmds: ResMut<ActionListRendererTarget>,
    mut renderers: Query<(&mut RendererParam, &mut RendererRenderTarget, &GraphId, &mut FlagRendererParamForPipeline)>,
    targets: Res<CustomRenderTargets>,
    mut graphic: ResMut<PiRenderGraph>,
    mut error: ResMut<ErrorRecord>,
    mut cmdmodifys: ResMut<ActionListRendererModify>,
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_renderer_modify"));
    cmds.drain().for_each(|cmd| {
        match cmd {
            OpsRendererTarget::Custom(entity, keytarget) => {
                if let Ok((mut renderparam, mut rendertarget, nodeid, mut flag)) = renderers.get_mut(entity) {

                    match keytarget {
                        KeyCustomRenderTarget::Custom(key) => {
                            if let Some(srt) = targets.get(key) {
                                renderparam.rendersize = RenderSize::new(srt.width, srt.height);
                                *rendertarget = RendererRenderTarget::Custom(srt.rt.clone());
                                if renderparam.colorformat.0 != srt.color_format || renderparam.depthstencilformat.0 != srt.depth_stencil_format {
                                    renderparam.colorformat = RenderColorFormat(srt.color_format);
                                    renderparam.depthstencilformat = RenderDepthFormat(srt.depth_stencil_format);
                                    *flag = FlagRendererParamForPipeline;
                                }
                                // log::warn!("sys_act_renderer_target Custom {:?}", srt.color_format);
                            } else {
                                *rendertarget = RendererRenderTarget::None(None);
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

                            if renderparam.colorformat.0 != format || renderparam.depthstencilformat.0 != DepthStencilFormat::None {
                                renderparam.colorformat = RenderColorFormat(format);
                                renderparam.depthstencilformat = RenderDepthFormat(DepthStencilFormat::None);
                                *flag = FlagRendererParamForPipeline;
                            }
                            *rendertarget = RendererRenderTarget::FinalRender;
    
                            if let Err(err) = graphic.set_finish(nodeid.0, true) {
                                error.graphic(entity, err);
                            }
                        },
                    }
                };
            },
            OpsRendererTarget::Auto(entity, width, height, colorformat, depthstencilformat) => {
                if let Ok((mut renderparam, mut rendertarget, _nodeid, mut flag)) = renderers.get_mut(entity) {
                    renderparam.rendersize = RenderSize::new(width as u32, height as u32);
                    if renderparam.colorformat.0 != colorformat || renderparam.depthstencilformat.0 != depthstencilformat {
                        renderparam.colorformat = RenderColorFormat(colorformat);
                        renderparam.depthstencilformat = RenderDepthFormat(depthstencilformat);
                        *flag = FlagRendererParamForPipeline;
                    }
                    *rendertarget = RendererRenderTarget::None(None);
                }
            },
        }
    });
    cmdmodifys.drain().for_each(|cmd| {
        match cmd {
            OpsRendererCommand::Active(entity, val) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.enable = RendererEnable(val);
                }
                // else { cmdmodifys.push(cmd) }
            },
            OpsRendererCommand::Blend(entity, val) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.blend = RendererBlend(val);
                }
                // else { cmdmodifys.push(cmd) }
            },
            OpsRendererCommand::ColorClear(entity, val) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.color_clear = val;
                }
                // else { cmdmodifys.push(cmd) }
            },
            OpsRendererCommand::DepthClear(entity, val) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.depth_clear = val;
                }
                // else { cmdmodifys.push(cmd) }
            },
            OpsRendererCommand::StencilClear(entity, val) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.stencil_clear = val;
                }
                // else { cmdmodifys.push(cmd) }
            },
            OpsRendererCommand::AutoClearColor(entity, val) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.auto_clear_color = RenderAutoClearColor(val);
                }
                // else { cmdmodifys.push(cmd) }
            },
            OpsRendererCommand::AutoClearDepth(entity, val) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.auto_clear_depth = RenderAutoClearDepth(val);
                }
                // else { cmdmodifys.push(cmd) }
            },
            OpsRendererCommand::AutoClearStencil(entity, val) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.auto_clear_stencil = RenderAutoClearStencil(val);
                }
                // else { cmdmodifys.push(cmd) }
            },
            OpsRendererCommand::Viewport(entity, x, y, z, w) => {
                if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                    comp.viewport = RenderViewport(x, y, z, w, 0., 1.);
                }
                // else { cmdmodifys.push(cmd) }
            }
        }
    });
}

pub fn sys_act_renderer_connect(
    mut cmds: ResMut<ActionListRendererConnect>,
    mut render_graphic: ResMut<PiRenderGraph>,
    renderers: Query<&GraphId>,
    mut error: ResMut<ErrorRecord>,
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_renderer_connect"));
    cmds.drain().for_each(|OpsRendererConnect(before, after, isdisconnect)| {
        if let (Ok(nbefore), Ok(nafter)) = (renderers.get(before), renderers.get(after)) {
            if isdisconnect {
                if let Err(err) = render_graphic.remove_depend(nbefore.0, nafter.0) {
                    // log::error!("3D Disconnect: {:?}", (nbefore.0, nafter.0));
                    error.graphic(before, err);
                }
            } else {
                if let Err(err) = render_graphic.add_depend(nbefore.0, nafter.0) {
                    // log::error!("3D Connect: {:?}", (nbefore.0, nafter.0));
                    error.graphic(before, err);
                }
            }
            // render_graphic.dump_graphviz();
        }
    });
}
pub fn sys_dispose_renderer(
    mut render_graphic: ResMut<PiRenderGraph>,
    renderers: Query<(Entity, &GraphId, &DisposeCan), Changed<DisposeCan>>,
    mut error: ResMut<ErrorRecord>,
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_dispose_renderer"));
    renderers.iter().for_each(|(entity, nodeid, flag)| {
        if flag.0 == false { return; }
        if let Err(err) = render_graphic.remove_node(nodeid.0) {
            error.graphic(entity, err);
        }
    });
}

pub type RendererBundle = (
    BundleEntity,
    (
        PassTag,
        Renderer,
        RendererParam,
        FlagRendererParamForPipeline,
        RendererRenderTarget,
        ViewerID,
        Postprocess,
    )
);

pub struct ActionRenderer;
impl ActionRenderer {
    pub(crate) fn init(
        id_viewer: Entity,
        passtag: PassTag,
        transparent: bool,
    ) -> RendererBundle {
        (
            ActionEntity::init(),
            (
                passtag,
                Renderer::new(),
                RendererParam::new(transparent),
                FlagRendererParamForPipeline,
                RendererRenderTarget::None(None),
                ViewerID(id_viewer),
                Postprocess::default(),
            )
        )
    }
    pub fn apply_graph_id(
        node: NodeId,
    ) -> GraphId {
        GraphId(node)
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
