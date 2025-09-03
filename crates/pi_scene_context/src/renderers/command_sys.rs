
use pi_scene_shell::prelude::*;
use pi_slotmap::Key;

use crate::{
    postprocess::*, prelude::CrossDrawList, viewer::prelude::*
};

use super::{
    renderer::*,
    graphic::*,
    command::*,
};

pub fn sys_create_subgraph(
    mut cmds: ResMut<ActionListSubGraphCreate>,
    mut graphic: ResMut<PiRenderGraph>,
    mut error: ResMut<ResErrorRecord>,
    mut alter: Alter<(), (), (GraphId, BundleEntity), ()>,
) {
    cmds.drain().for_each(|OpsSubGraphCreate(entity, name)| {
        if let Ok(graph) = graphic.add_sub_graph(name.clone()) {
            let _ = alter.alter(entity, (GraphId(graph), ActionEntity::init()));
            // log::error!("SubGraph: {:?}", (entity, name));
        } else {
            error.record(entity.index(), ErrorRecord::ERROR_SUB_GRAPHIC_ERROR);
        }
    });
}

pub fn sys_create_renderer(
    mut cmds: ResMut<ActionListRendererCreate>,
    mut graphic: ResMut<PiRenderGraph>,
    mut viewers: Query<(&SceneID, &mut ViewerRenderersInfo, &mut DirtyViewerRenderersInfo, &ViewerGraphID)>,
    mut error: ResMut<ResErrorRecord>,
    mut alter: Alter<(), (), (GraphId, SceneID, RendererBundle, SimpleInOut), ()>,
    mut alter2: Alter<(), (), (GraphId, SceneID, CrossDrawList, RendererBundle, SimpleInOut), ()>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_create_renderer"));
    cmds.drain().for_each(|OpsRendererCreate(entity, name, id_viewer, passtag, transparent, recordinput, crossrender)| {
        if let Ok((sceneid, mut viewerrenderinfo, mut viewerflag, graph)) = viewers.get_mut(id_viewer) {
            let render_node = RenderNode::new(entity);
            
            match graphic.add_node(name.clone(), render_node, graph.0, entity) {
                Ok(nodeid) => {
                    // log::error!("Node: {:?} in Graph {:?}", (&name, &entity, &nodeid), &graph.0);
                    // if let Some(mut cmd) = commands.get_entity(entity) {
                        viewerrenderinfo.add(entity, passtag);
                        *viewerflag = DirtyViewerRenderersInfo;
                        // log::error!("CreateRenderer {:?}", (nodeid, id_viewer, entity, viewerrenderinfo.len()));
                        if crossrender {
                        let crosslist = CrossDrawList::default();
                        let bundle = (
                            GraphId(nodeid), sceneid.clone(), crosslist,
                            ActionRenderer::init(id_viewer, passtag, transparent, recordinput),
                            SimpleInOut::default(),
                        );
                        // commands.entity(entity).insert(bundle);
                        let _ = alter2.alter(entity, bundle);
                        } else {
                        let bundle = (
                            GraphId(nodeid), sceneid.clone(),
                            ActionRenderer::init(id_viewer, passtag, transparent, recordinput),
                            SimpleInOut::default(),
                        );
                        // commands.entity(entity).insert(bundle);
                        let _ = alter.alter(entity, bundle);
                        }

                    // }
                },
                Err(err) => {
                    // log::error!("CreateRenderer Fail Graphic Error");
                    error.graphic(entity.index(), err);
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
    mut rendererslink: Query<&mut Renderer>,
    targets: Res<CustomRenderTargets>,
    mut graphic: ResMut<PiRenderGraph>,
    mut error: ResMut<ResErrorRecord>,
    mut cmdmodifys: ResMut<ActionListRendererModify>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_renderer_modify"));
    cmds.drain().for_each(|OpsRendererTarget(entity, cmd)| {
        match cmd {
            ERendererTarget::Custom(keytarg, asout) => {
                if let Ok((mut renderparam, mut rendertarget, nodeid, mut flag)) = renderers.get_mut(entity) {

                    match keytarg {
                        KeyCustomRenderTarget::Custom(key) => {
                            if let Some(srt) = targets.get(key) {
                                renderparam.rendersize = RenderSize::new(srt.width, srt.height, true);
                                if asout {
                                    *rendertarget = RendererRenderTarget::Custom(srt.rt.clone());
                                } else {
                                    *rendertarget = RendererRenderTarget::CustomAndOut(srt.rt.clone());
                                }
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
                                error.graphic(entity.index(), err);
                            }
                        },
                        KeyCustomRenderTarget::FinalRender(realscreen) => {
                            let format = match ColorFormat::new(wgpu::TextureFormat::pi_render_default()) {
                                Some(format) => format,
                                _ => ColorFormat::Rgba8Unorm
                            };

                            if renderparam.colorformat.0 != format || renderparam.depthstencilformat.0 != DepthStencilFormat::None {
                                renderparam.colorformat = RenderColorFormat(format);
                                renderparam.depthstencilformat = RenderDepthFormat(DepthStencilFormat::None);
                                *flag = FlagRendererParamForPipeline;
                            }
                            *rendertarget = RendererRenderTarget::FinalRender(realscreen);
    
                            if let Err(err) = graphic.set_finish(nodeid.0, realscreen) {
                                error.graphic(entity.index(), err);
                            }
                        },
                    }
                };
            },
            ERendererTarget::Auto(width, height, colorformat, depthstencilformat, force) => {
                if let Ok((mut renderparam, mut rendertarget, _nodeid, mut flag)) = renderers.get_mut(entity) {
                    renderparam.rendersize = RenderSize::new(width as u32, height as u32, force);
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
    cmdmodifys.drain().for_each(|OpsRendererCommand(entity, cmd)| {
        match cmd {
            ERendererCommand::Active( val) => {
                        if let Ok((mut comp, _, nodeid, _)) = renderers.get_mut(entity) {
                            comp.enable = RendererEnable(val);
                            let _ = graphic.set_enable(nodeid.0, val);
                        }
                    },
            ERendererCommand::Blend( val) => {
                        if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                            comp.blend = RendererBlend(val);
                        }
                    },
            ERendererCommand::ColorClear( val) => {
                        if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                            comp.color_clear = val;
                        }
                    },
            ERendererCommand::DepthClear( val) => {
                        if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                            comp.depth_clear = val;
                        }
                    },
            ERendererCommand::StencilClear( val) => {
                        if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                            comp.stencil_clear = val;
                        }
                    },
            ERendererCommand::AutoClearColor( val) => {
                        if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                            comp.auto_clear_color = RenderAutoClearColor(val);
                        }
                    },
            ERendererCommand::AutoClearDepth( val) => {
                        if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                            comp.auto_clear_depth = RenderAutoClearDepth(val);
                        }
                    },
            ERendererCommand::AutoClearStencil( val) => {
                        if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                            comp.auto_clear_stencil = RenderAutoClearStencil(val);
                        }
                    },
            ERendererCommand::Viewport( x, y, z, w, mind, maxd) => {
                        if let Ok((mut comp, _, _, _)) = renderers.get_mut(entity) {
                            comp.viewport = RenderViewport(x, y, z, w, mind, maxd);
                        }
                    }
            ERendererCommand::ClearLinkMesh(entity1) => {
                    if let Ok(mut comp) = rendererslink.get_mut(entity) {
                        comp.mesh_as_clear = entity1;
                    }
            },
        }
    });
}

pub fn sys_act_renderer_connect(
    mut cmds: ResMut<ActionListRendererConnect>,
    mut render_graphic: ResMut<PiRenderGraph>,
    renderers: Query<&GraphId>,
    mut error: ResMut<ResErrorRecord>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_renderer_connect"));
    cmds.drain().for_each(|OpsRendererConnect(before, after, isdisconnect)| {
        if let (Ok(nbefore), Ok(nafter)) = (renderers.get(before), renderers.get(after)) {
            // log::error!("Connect: {:?} => {:?} {:?}", &nbefore.0, &nafter.0, isdisconnect);
            if isdisconnect {
                if let Err(err) = render_graphic.remove_depend(nbefore.0, nafter.0) {
                    // log::error!("3D Disconnect: {:?}", (nbefore.0, nafter.0));
                    error.graphic(before.index(), err);
                }
            } else {
                if let Err(err) = render_graphic.add_depend(nbefore.0, nafter.0) {
                    // log::error!("3D Connect: {:?}", (nbefore.0, nafter.0));
                    error.graphic(before.index(), err);
                }
            }
            // render_graphic.dump_graphviz();
        }
    });
}
pub fn sys_dispose_renderer(
    mut render_graphic: ResMut<PiRenderGraph>,
    graphs: Query<(Entity, &GraphId, &DisposeCan), Changed<DisposeCan>>,
    renderers: Query<&RendererRenderTargetKey>,
    mut error: ResMut<ResErrorRecord>,
    mut targets: ResMut<CustomRenderTargets>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_dispose_renderer"));
    graphs.iter().for_each(|(entity, nodeid, flag)| {
        if flag.0 == false { return; }
        if let Err(err) = render_graphic.remove_node(nodeid.0) {
            error.graphic(entity.index(), err);
        }
        if let Ok(rtkey) = renderers.get(entity) {
            if let Some(key) = rtkey.0 {
                targets.delete(key);
            }
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
        RendererRenderTargetKey,
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
        recordinput: bool,
    ) -> RendererBundle {
        (
            ActionEntity::init(),
            (
                passtag,
                Renderer::new(),
                RendererParam::new(transparent),
                FlagRendererParamForPipeline,
                RendererRenderTarget::None(None),
                RendererRenderTargetKey(None, recordinput),
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
}
