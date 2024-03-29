
use pi_scene_shell::prelude::*;

pub use super::renderer::*;
pub use super::command::*;
pub use super::opaque::*;
pub use super::render_blend::*;
pub use super::render_depth_and_stencil::*;
pub use super::render_mode::*;
pub use super::render_primitive::*;
pub use super::render_sort::*;
pub use super::render_target_state::*;
pub use super::render_object::*;
pub use super::graphic::*;
pub use super::base::*;

#[derive(SystemParam)]
pub struct ActionSetRenderer<'w> {
    pub create: ResMut<'w, ActionListRendererCreate>,
    pub connect: ResMut<'w, ActionListRendererConnect>,
    pub modify: ResMut<'w, ActionListRendererModify>,
    pub target: ResMut<'w, ActionListRendererTarget>,
}

// pub type StateTransformQuery = QueryState<(&'static SceneID, &'static Enable, &'static GlobalEnable)>;