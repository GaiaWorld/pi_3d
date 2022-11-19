use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::object::{ObjectID, GameObject};
use pi_render::rhi::dyn_uniform_buffer::Uniform;
use pi_scene_math::Number;

use crate::bytes_write_to_memory;


#[derive(Debug, Clone, Copy)]
pub struct Texture2DScaleOffset {
    pub u_tiling: Number,
    pub v_tiling: Number,
    pub u_offset: Number,
    pub v_offset: Number,
}
impl Default for Texture2DScaleOffset {
    fn default() -> Self {
        Self { u_tiling: 1., v_tiling: 1., u_offset: 0., v_offset: 0. }
    }
}
