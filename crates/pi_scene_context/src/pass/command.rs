use pi_scene_shell::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::{EDepthState, EPrimitiveState, EStencilState, RenderQueueSortParam};



pub struct OpsPassObject(pub(crate) Entity, pub(crate) Entity, pub(crate) PassTag);
impl OpsPassObject {
    pub fn ops(model: Entity, material: Entity, pass: PassTag) -> Self {
        Self(model, material, pass)
    }
}
pub type ActionListPassObject = ActionList<OpsPassObject>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ERenderState {
    RenderQueue( RenderQueueSortParam),
    Blend( PassTag, ModelBlend),
    DepthState( PassTag, EDepthState),
    StencilState( PassTag, EStencilState),
    PrimitiveState( PassTag, EPrimitiveState),
}

pub struct OpsRenderState(pub(crate) Entity, pub(crate) ERenderState);
impl OpsRenderState {
    pub fn ops(mesh: Entity, val: ERenderState) -> Self {
        Self(mesh, val)
    }
    pub fn blend(mesh: Entity, pass: PassTag, mode: ModelBlend) -> Self {
        Self(mesh, ERenderState::Blend(pass, mode))
    }
    pub fn depth_state(mesh: Entity, pass: PassTag, val: EDepthState) -> Self {
        Self(mesh, ERenderState::DepthState( pass, val))
    }
    pub fn stencil_state(mesh: Entity, pass: PassTag, val: EStencilState) -> Self {
        Self(mesh, ERenderState::StencilState( pass, val))
    }
    pub fn primitive_state(mesh: Entity, passtag: PassTag, cmd: EPrimitiveState) -> Self {
        Self(mesh, ERenderState::PrimitiveState( passtag, cmd))
    }
    pub fn render_queue(mesh: Entity, group: i32, index: i32) -> Self {
        Self(mesh, ERenderState::RenderQueue( RenderQueueSortParam { group, index }))
    }
}
pub type ActionListRenderState = ActionList<OpsRenderState>;