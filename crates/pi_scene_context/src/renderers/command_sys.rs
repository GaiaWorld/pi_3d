
use pi_bevy_render_plugin::component::GraphId;
use pi_engine_shell::prelude::*;

use crate::{viewer::prelude::*, postprocess::Postprocess};

use super::{
    renderer::*,
    render_object::RendererID,
    graphic::{RendererGraphicDesc, RenderNode},
    command::*
};

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

pub struct ActionRenderer;
impl ActionRenderer {
    pub(crate) fn as_renderer(
        commands_renderer: &mut EntityCommands,
        node: NodeId,
        id_viewer: Entity,
        graphic_desc: RendererGraphicDesc,
        width: u32,
        height: u32,
    ) {
        commands_renderer
            .insert(GraphId(node))
            .insert(graphic_desc.passorders.clone())
            .insert(Renderer::new())
            .insert(RenderSize::new(width, height))
            .insert(RendererEnable(true))
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
}
