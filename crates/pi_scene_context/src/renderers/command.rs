
use pi_engine_shell::prelude::*;
use super::renderer::*;

pub struct OpsRendererCreate(pub(crate) Entity, pub(crate) String, pub(crate) Entity, pub(crate) PassTag, pub(crate) bool);
impl OpsRendererCreate {
    pub fn ops(entity: Entity, name: String, idviewer: Entity, passtag: PassTag, transparent: bool) -> Self {
        Self(entity, name, idviewer, passtag, transparent)
    }
}
pub type ActionListRendererCreate = ActionList<OpsRendererCreate>;

pub struct OpsRendererConnect(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
impl OpsRendererConnect {
    pub fn ops(before: Entity, after: Entity, isdisconnect: bool) -> Self {
        Self(before, after, isdisconnect)
    }
}
pub type ActionListRendererConnect = ActionList<OpsRendererConnect>;

pub enum OpsRendererTarget {
    Custom(Entity, KeyCustomRenderTarget),
    Auto(Entity, u16, u16, ColorFormat, DepthStencilFormat),
}
impl OpsRendererTarget {
    pub fn ops(idrenderer: Entity, key: KeyCustomRenderTarget) -> Self {
        Self::Custom(idrenderer, key)
    }
}
pub type ActionListRendererTarget = ActionList<OpsRendererTarget>;


#[derive(Debug, Clone, Copy)]
pub enum OpsRendererCommand {
    Active(Entity, bool),
    Blend(Entity, bool),
    ColorClear(Entity, RenderColorClear),
    DepthClear(Entity, RenderDepthClear),
    StencilClear(Entity, RenderStencilClear),
    AutoClearColor(Entity, bool),
    AutoClearDepth(Entity, bool),
    AutoClearStencil(Entity, bool),
    Viewport(Entity, f32, f32, f32, f32),
}

pub type ActionListRendererModify = ActionList<OpsRendererCommand>;