
use pi_engine_shell::prelude::*;
use super::renderer::*;

pub struct OpsRendererCreate(pub(crate) Entity, pub(crate) String);
impl OpsRendererCreate {
    pub fn ops(entity: Entity, name: String) -> Self {
        Self(entity, name)
    }
}
pub type ActionListRendererCreate = ActionList<OpsRendererCreate>;

pub struct OpsRendererConnect(pub(crate) Entity, pub(crate) Entity, pub(crate) u8);
impl OpsRendererConnect {
    pub fn ops(before: Entity, after: Entity) -> Self {
        Self(before, after, 0)
    }
}
pub type ActionListRendererConnect = ActionList<OpsRendererConnect>;


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