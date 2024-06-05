use pi_scene_shell::prelude::*;
use crate::{
    object::{ObjectID},
    prelude::*,
};

#[derive(Clone, Copy, Component, Default)]
pub struct RendererID(pub ObjectID);

#[derive(Clone, Component, Default)]
pub struct RenderState {
    pub depth: DepthState,
    pub stencil: StencilState,
    pub primitive: PrimitiveState,
    pub blend: ModelBlend,
}