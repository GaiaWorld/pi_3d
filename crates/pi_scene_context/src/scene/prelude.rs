
use pi_scene_shell::prelude::*;

use crate::prelude::ActionListBoundingBoxDisplay;
use crate::prelude::ActionListCollider;

pub use super::base::*;
pub use super::command::*;
pub use super::coordinate_system::*;
pub use super::environment::{
    BindSceneEffect,
    fog::*,
    scene_time::*,
    ambient_light::*,
    brdf::*,
    environment_texture::*,
};
pub use super::pass_render_target::*;

#[derive(SystemParam)]
pub struct ActionSetScene<'w> {
    pub create: ResMut<'w, ActionListSceneCreate>,
    pub options: ResMut<'w, ActionListSceneOption>,
    pub boundingboxdisplay: ResMut<'w, ActionListBoundingBoxDisplay>,
    pub collider: ResMut<'w, ActionListCollider>,
}

