use pi_scene_shell::prelude::*;
use crate::{
    object::{ObjectID},
    prelude::*,
};

#[derive(Clone, Copy, Component)]
pub struct RendererID(pub Entity);
impl Default for RendererID {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}

#[derive(Clone, Component, Default)]
pub struct RenderState {
    pub depth: DepthState,
    pub stencil: StencilState,
    pub primitive: PrimitiveState,
    pub blend: ModelBlend,
}