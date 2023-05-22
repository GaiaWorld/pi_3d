
use pi_engine_shell::prelude::*;
use super::renderer::*;

#[derive(Debug, Clone, Copy)]
pub enum OpsRendererCommand {
    Active(Entity, bool),
    Size(Entity, u16, u16),
    ColorFormat(Entity, RenderColorFormat),
    ColorClear(Entity, RenderColorClear),
    DepthFormat(Entity, RenderDepthFormat),
    DepthClear(Entity, RenderDepthClear),
    StencilClear(Entity, RenderStencilClear),
    AutoClearColor(Entity, bool),
    AutoClearDepth(Entity, bool),
    AutoClearStencil(Entity, bool),
    RenderToFinal(Entity, bool),
}

pub type ActionListRendererModify = ActionList<OpsRendererCommand>;