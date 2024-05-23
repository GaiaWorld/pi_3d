use pi_scene_shell::{add_component, prelude::{pi_world::editor::EntityEditor, *}};

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
    // mut commands: Commands,
    mut editor: EntityEditor,
    mut cmds: ResMut<ActionListRendererCreate>,
    mut graphic: ResMut<PiRenderGraph>,
    mut viewers: Query<(&SceneID, &mut ViewerRenderersInfo, &mut DirtyViewerRenderersInfo)>,
    mut error: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|OpsRendererCreate(entity, name, id_viewer, passtag, transparent)| {
        if let Ok((sceneid, mut viewerrenderinfo, mut viewerflag)) = viewers.get_mut(id_viewer) {
            let render_node = RenderNode::new(entity);
            match graphic.add_node(name, render_node, NodeId::null()) {
                Ok(nodeid) => {
                    if editor.contains_entity(entity) {
                        let components = [editor.init_component::<GraphId>(), editor.init_component::<SceneID>()];
                        editor.add_components(entity, &components);
                        *editor.get_component_unchecked_mut_by_id(entity, components[0]) =GraphId(nodeid);
                        *editor.get_component_unchecked_mut_by_id(entity, components[1]) =sceneid.clone();
            
                        // editor.add_components(entity, (GraphId(nodeid), sceneid.clone()));
                        ActionRenderer::init(entity, &mut editor, id_viewer, passtag, transparent);
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
                    *rendertarget = RendererRenderTarget::None(None);
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
            // render_graphic.dump_graphviz();
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
type ActionRendererBundle = (
    PassTag,
    Renderer,
    RenderViewport,
    RenderSize,
    RendererEnable,
    RenderColorClear,
    RenderColorFormat,
    RenderDepthClear,
    RenderDepthFormat,
    RenderStencilClear,
    RenderAutoClearColor,
    RenderAutoClearDepth,
    RenderAutoClearStencil,
    RendererRenderTarget,
    RendererBlend,
    ViewerID,
    Postprocess
);
pub struct ActionRenderer;
impl ActionRenderer {
    pub(crate) fn init(
        // commands_renderer: &mut EntityCommands,
        entity: Entity,  
        editor: &mut EntityEditor,
        id_viewer: Entity,
        passtag: PassTag,
        transparent: bool,
    ) {
        ActionEntity::init(entity, editor);
        let components = [
            editor.init_component::<PassTag>(),
            editor.init_component::<Renderer>(),
            editor.init_component::<RenderViewport>(),
            editor.init_component::<RenderSize>(),
            editor.init_component::<RendererEnable>(),
            editor.init_component::<RenderColorClear>(),
            editor.init_component::<RenderColorFormat>(),
            editor.init_component::<RenderDepthClear>(),
            editor.init_component::<RenderDepthFormat>(),
            editor.init_component::<RenderStencilClear>(),
            editor.init_component::<RenderAutoClearColor>(),
            editor.init_component::<RenderAutoClearDepth>(),
            editor.init_component::<RenderAutoClearStencil>(),
            editor.init_component::<RendererRenderTarget>(),
            editor.init_component::<RendererBlend>(),
            editor.init_component::<ViewerID>(),
            editor.init_component::<Postprocess>(),
        ];
        editor.add_components(entity, &components);
       
        *editor.get_component_unchecked_mut_by_id(entity, components[0]) = passtag;
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) =Renderer::new();
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) =RenderViewport::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) =RenderSize::new(100, 100);
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) =RendererEnable(true);
        *editor.get_component_unchecked_mut_by_id(entity, components[5]) =RenderColorClear::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[6]) =RenderColorFormat::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[7]) =RenderDepthClear::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[8]) =RenderDepthFormat::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[9]) =RenderStencilClear::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[10]) =RenderAutoClearColor::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[11]) =RenderAutoClearDepth::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[12]) =RenderAutoClearStencil::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[13]) =RendererRenderTarget::None(None);
        *editor.get_component_unchecked_mut_by_id(entity, components[14]) =RendererBlend(transparent);
        *editor.get_component_unchecked_mut_by_id(entity, components[15]) =ViewerID(id_viewer);
        *editor.get_component_unchecked_mut_by_id(entity, components[16]) =Postprocess::default();

    }
    pub fn create_graphic_node(
        // commands: &mut Commands,
        editor: &mut EntityEditor,
        render_graphic: &mut PiRenderGraph,
        error: &mut ErrorRecord,
        name: String,
    ) -> Entity {
        let entity = editor.alloc_entity();
        let render_node = RenderNode::new(entity);
        match render_graphic.add_node(name, render_node, NodeId::null()) {
            Ok(nodeid) => {
                if editor.contains_entity(entity) {
                    add_component(editor, entity,GraphId(nodeid));
                }
            },
            Err(err) => {
                error.graphic(entity, err);
            },
        }

        entity
    }
    pub fn apply_graph_id(
        // entitycmd: &mut EntityCommands,
        entity: Entity,  
        editor: &mut EntityEditor,
        node: NodeId,
    ) {
        add_component(editor, entity, GraphId(node));
        // alter1.alter(entity, (GraphId(node),));
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
