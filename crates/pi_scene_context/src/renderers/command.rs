
use pi_scene_shell::prelude::*;
use serde::{Deserialize, Serialize};
use super::renderer::*;

pub struct OpsSubGraphCreate(pub(crate) Entity, pub(crate) String);
impl OpsSubGraphCreate {
    pub fn ops(entity: Entity, name: String) -> Self {
        Self(entity, name)
    }
}
pub type ActionListSubGraphCreate = ActionList<OpsSubGraphCreate>;

pub struct OpsRendererCreate(pub(crate) Entity, pub(crate) String, pub(crate) Entity, pub(crate) PassTag, pub(crate) bool, pub(crate) bool, pub(crate) bool);
impl OpsRendererCreate {
    pub fn ops(entity: Entity, name: String, idviewer: Entity, passtag: PassTag, transparent: bool, recordinput: bool, crossrender: bool) -> Self {
        Self(entity, name, idviewer, passtag, transparent, recordinput, crossrender)
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ERendererTarget {
    Custom(KeyCustomRenderTarget, bool),
    Auto(u16, u16, ColorFormat, DepthStencilFormat, bool),
}

pub struct OpsRendererTarget(pub(crate) Entity, pub(crate) ERendererTarget);
impl OpsRendererTarget {
    pub fn ops(idrenderer: Entity, key: KeyCustomRenderTarget) -> Self {
        Self(idrenderer, ERendererTarget::Custom(key, false))
    }
    pub fn new(idrenderer: Entity, key: ERendererTarget) -> Self {
        Self(idrenderer, key)
    }
}
pub type ActionListRendererTarget = ActionList<OpsRendererTarget>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ERendererCommand {
    Active(bool),
    Blend(bool),
    ColorClear(RenderColorClear),
    DepthClear(RenderDepthClear),
    StencilClear(RenderStencilClear),
    AutoClearColor(bool),
    AutoClearDepth(bool),
    AutoClearStencil(bool),
    Viewport(f32, f32, f32, f32, f32, f32),
    ClearLinkMesh(Entity),
}

pub struct OpsRendererCommand(pub(crate) Entity, pub(crate) ERendererCommand);
impl OpsRendererCommand {
    pub fn ops(entity: Entity, cmd: ERendererCommand) -> Self {
        Self(entity, cmd)
    }
}

pub type ActionListRendererModify = ActionList<OpsRendererCommand>;