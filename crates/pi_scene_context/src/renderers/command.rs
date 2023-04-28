
use pi_bevy_render_plugin::component::GraphId;
use pi_engine_shell::prelude::*;

use crate::{viewer::ViewerID, postprocess::Postprocess};

use super::{renderer::*, render_object::RendererID, graphic::{RendererGraphicDesc, RenderNode}};

pub enum ERendererCommand {
    Active(bool),
    Size(RenderSize),
    ColorFormat(RenderColorFormat),
    ColorClear(RenderColorClear),
    DepthFormat(RenderDepthFormat),
    DepthClear(RenderDepthClear),
    StencilClear(RenderStencilClear),
    AutoClearColor(bool),
    AutoClearDepth(bool),
    AutoClearStencil(bool),
    RenderToFinal(bool),
}

#[derive(Default)]
pub struct SingleRendererCommandList {
    pub list: Vec<ERendererCommand>,
}

pub struct ActionRenderer;
impl ActionRenderer {
    pub(crate) fn as_renderer(
        commands_renderer: &mut EntityCommands,
        node: NodeId,
        id_viewer: Entity,
        graphic_desc: RendererGraphicDesc,
    ) {
        commands_renderer
            .insert(GraphId(node))
            .insert(graphic_desc.passorders.clone())
            .insert(Renderer::new())
            .insert(RendererEnable(false))
            .insert(RenderColorClear::default())
            .insert(RenderColorFormat::default())
            .insert(RenderDepthClear::default())
            .insert(RenderDepthFormat::default())
            .insert(RenderStencilClear::default())
            .insert(RenderAutoClearColor::default())
            .insert(RenderAutoClearDepth::default())
            .insert(RenderAutoClearStencil::default())
            .insert(RenderToFinalTarget(true))
            .insert(ViewerID(id_viewer))
            .insert(Postprocess::default());
    }
    pub fn create_graphic_node(
        render_graphic: &mut PiRenderGraph,
        name: String,
        id_viewer: Entity,
        id_renderer: RendererID,
        graphic_desc: &RendererGraphicDesc,
    ) -> Result<NodeId, GraphError> {
        let entity = id_renderer.0;
        let render_node = RenderNode::new(entity);

        match render_graphic.add_node(String::from(name.as_str()), render_node) {
            Ok(nodeid) => {
                if let Some(key_pre) = &graphic_desc.pre {
                    log::debug!("Add Node {:?} > {:?}", key_pre.to_string(), graphic_desc.curr.to_string());
                    render_graphic.add_depend(key_pre.to_string(), graphic_desc.curr.to_string());
                }
                if let Some(key_next) = &graphic_desc.next {
                    log::debug!("Add Node {:?} > {:?}", graphic_desc.curr.to_string(), key_next.to_string());
                    render_graphic.add_depend(graphic_desc.curr.to_string(), key_next.to_string());
                } else {
                    render_graphic.set_finish(graphic_desc.curr.to_string(), true);
                }
                render_graphic.dump_graphviz();
                Ok(nodeid)
            },
            Err(e) => Err(e),
        }
    }
    
    pub fn modify(
        commands: &mut EntityCommands,
        cmd: ERendererCommand,
    ) {
        match cmd {
            ERendererCommand::Active(val) => {
                commands.insert(RendererEnable(val));
            },
            ERendererCommand::Size(val) => {
                commands.insert(val);
            },
            ERendererCommand::ColorFormat(val) => {
                commands.insert(val);
            },
            ERendererCommand::ColorClear(val) => {
                commands.insert(val);
            },
            ERendererCommand::DepthFormat(val) => {
                commands.insert(val);
            },
            ERendererCommand::DepthClear(val) => {
                commands.insert(val);
            },
            ERendererCommand::StencilClear(val) => {
                commands.insert(val);
            },
            ERendererCommand::AutoClearColor(val) => {
                commands.insert(RenderAutoClearColor(val));
            },
            ERendererCommand::AutoClearDepth(val) => {
                commands.insert(RenderAutoClearDepth(val));
            },
            ERendererCommand::AutoClearStencil(val) => {
                commands.insert(RenderAutoClearStencil(val));
            },
            ERendererCommand::RenderToFinal(val) => {
                commands.insert(RenderToFinalTarget(val));
            },
        }
    }
}
