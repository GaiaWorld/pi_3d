
use pi_engine_shell::prelude::*;

pub use super::command::*;
pub use super::passes_cfg::*;
pub use super::coordinate_system::*;
pub use super::environment::{
    BindSceneEffect,
    fog::*,
    scene_time::*,
    ambient_light::*,
};

#[derive(SystemParam)]
pub struct ActionSetScene<'w> {
    pub create: ResMut<'w, ActionListSceneCreate>,
}

