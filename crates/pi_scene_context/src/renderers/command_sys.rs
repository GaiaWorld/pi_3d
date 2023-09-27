
use pi_bevy_render_plugin::component::GraphId;
use pi_engine_shell::prelude::*;

use crate::{viewer::prelude::*, postprocess::Postprocess, prelude::{PassTagOrders, DisposeCan}, object::ActionEntity};

use super::{
    renderer::*,
    render_object::RendererID,
    graphic::*,
    command::*
};

pub fn sys_create_renderer(
    mut cmds: ResMut<ActionListRendererCreate>,
    mut graphic: ResMut<PiRenderGraph>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsRendererCreate(entity, name)| {
        let render_node = RenderNode::new(entity);
        match graphic.add_node(name, render_node) {
            Ok(nodeid) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(GraphId(nodeid));
                }
            },
            Err(_e) => {
                // log::error!("Renderer Error: {:?}", e);
                log::debug!("Renderer Error:");
            },
        }
    });
}

pub fn sys_renderer_modify(
    mut cmds: ResMut<ActionListRendererModify>,
    mut enables: Query<&mut RendererEnable>,
    mut rendersizes: Query<&mut RenderSize>,
    mut colorclear: Query<&mut RenderColorClear>,
    mut colorformat: Query<&mut RenderColorFormat>,
    mut depthclear: Query<&mut RenderDepthClear>,
    mut depthformat: Query<&mut RenderDepthFormat>,
    mut stencilclear: Query<&mut RenderStencilClear>,
    mut autoclearcolor: Query<&mut RenderAutoClearColor>,
    mut autocleardepth: Query<&mut RenderAutoClearDepth>,
    mut autoclearstencil: Query<&mut RenderAutoClearStencil>,
    mut tofinals: Query<&mut RenderToFinalTarget>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsRendererCommand::Active(entity, val) => {
                if let Ok(mut comp) = enables.get_mut(entity) {
                    *comp = RendererEnable(val);
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::Size(entity, w, h) => {
                if let Ok(mut comp) = rendersizes.get_mut(entity) {
                    *comp = RenderSize::new(w as u32, h as u32);
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::ColorFormat(entity, val) => {
                if let Ok(mut comp) = colorformat.get_mut(entity) {
                    *comp = val;
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::ColorClear(entity, val) => {
                if let Ok(mut comp) = colorclear.get_mut(entity) {
                    *comp = val;
                } else { cmds.push(cmd) }
            },
            OpsRendererCommand::DepthFormat(entity, val) => {
                if let Ok(mut comp) = depthformat.get_mut(entity) {
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
            OpsRendererCommand::RenderToFinal(entity, val) => {
                if let Ok(mut comp) = tofinals.get_mut(entity) {
                    *comp = RenderToFinalTarget(val);
                } else { cmds.push(cmd) }
            },
        }
    });
}

pub fn sys_act_renderer_connect(
    mut cmds: ResMut<ActionListRendererConnect>,
    mut render_graphic: ResMut<PiRenderGraph>,
    renderers: Query<&GraphId>,
) {
    cmds.drain().drain(..).for_each(|OpsRendererConnect(before, after, count)| {
        if let (Ok(before), Ok(after)) = (renderers.get(before), renderers.get(after)) {
            if let Err(_e) = render_graphic.add_depend(before.0, after.0) {
                // log::error!("{:?}", e);
                log::debug!("sys_act_renderer_connect add_depend Error");
            }
        } else {
            if count < 4 {
                cmds.push(OpsRendererConnect(before, after, count + 1))
            }
        }
    });
}

pub fn sys_dispose_renderer(
    mut render_graphic: ResMut<PiRenderGraph>,
    renderers: Query<(&GraphId, &RendererEnable, &DisposeCan), Changed<DisposeCan>>,
) {
    renderers.iter().for_each(|(nodeid, _, flag)| {
        if flag.0 {
            render_graphic.remove_node(nodeid.0);
        }
    });
}

pub struct ActionRenderer;
impl ActionRenderer {
    pub(crate) fn init(
        commands_renderer: &mut EntityCommands,
        id_viewer: Entity,
        passorders: PassTagOrders,
        width: u32,
        height: u32,
        color_format: ColorFormat,
        depth_format: DepthStencilFormat,
        toscreen: bool,
    ) {
        ActionEntity::init(commands_renderer);
        commands_renderer
            .insert(passorders)
            .insert(Renderer::new())
            .insert(RenderSize::new(width, height))
            .insert(RendererEnable(true))
            .insert(RenderColorClear::default())
            .insert(RenderColorFormat(color_format))
            .insert(RenderDepthClear::default())
            .insert(RenderDepthFormat(depth_format))
            .insert(RenderStencilClear::default())
            .insert(RenderAutoClearColor::default())
            .insert(RenderAutoClearDepth::default())
            .insert(RenderAutoClearStencil::default())
            .insert(RenderToFinalTarget(toscreen))
            .insert(ViewerID(id_viewer))
            .insert(Postprocess::default())
            ;
    }
    pub fn create_graphic_node(
        commands: &mut Commands,
        render_graphic: &mut PiRenderGraph,
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
            Err(_e) => {
                // log::error!("{:?}", e);
                log::debug!("create_graphic_node fail");
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
        _id_renderer: RendererID,
        nodeid: NodeId,
        pre: Option<NodeId>,
        next: Option<NodeId>,
    ) {
        if let Some(key_pre) = pre {
            // log::warn!("Add Node {:?} > {:?}", key_pre, nodeid);
            if let Err(_e) = render_graphic.add_depend(key_pre, nodeid) {
                // log::error!("{:?}", e);
                log::debug!("render_graphic.add_depend faile");
            }
        }
        if let Some(key_next) = next {
            // log::warn!("Add Node {:?} > {:?}", nodeid, key_next);
            if let Err(_e) = render_graphic.add_depend(nodeid, key_next) {
                // log::error!("{:?}", e);
                log::debug!("render_graphic.add_depend faile");
            }
        } else {
            // if let Err(e) = render_graphic.set_finish(nodeid, true) {
            //     log::debug!("{:?}", e);
            // }
        }
        // render_graphic.dump_graphviz();
    }
}
