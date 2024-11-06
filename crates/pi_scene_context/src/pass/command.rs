use pi_scene_shell::prelude::*;

use crate::prelude::{EDepthState, EPrimitiveState, EStencilState, RenderQueueSortParam};



pub struct OpsPassObject(pub(crate) Entity, pub(crate) Entity, pub(crate) PassTag);
impl OpsPassObject {
    pub fn ops(model: Entity, material: Entity, pass: PassTag) -> Self {
        Self(model, material, pass)
    }
}
pub type ActionListPassObject = ActionList<OpsPassObject>;

pub enum OpsRenderState {
    RenderQueue(Entity, RenderQueueSortParam),
    Blend(Entity, PassTag, ModelBlend),
    DepthState(Entity, PassTag, EDepthState),
    StencilState(Entity, PassTag, EStencilState),
    PrimitiveState(Entity, PassTag, EPrimitiveState),
}
impl OpsRenderState {
    pub fn blend(mesh: Entity, pass: PassTag, mode: ModelBlend) -> Self {
        Self::Blend(mesh, pass, mode)
    }
    pub fn depth_state(mesh: Entity, pass: PassTag, val: EDepthState) -> Self {
        Self::DepthState(mesh, pass, val)
    }
    pub fn stencil_state(mesh: Entity, pass: PassTag, val: EStencilState) -> Self {
        Self::StencilState(mesh, pass, val)
    }
    pub fn primitive_state(model: Entity, passtag: PassTag, cmd: EPrimitiveState) -> Self {
        Self::PrimitiveState(model, passtag, cmd)
    }
    pub fn render_queue(mesh: Entity, group: i32, index: i32) -> Self {
        Self::RenderQueue(mesh, RenderQueueSortParam { group, index })
    }
}
pub type ActionListRenderState = ActionList<OpsRenderState>;