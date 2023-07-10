
use pi_engine_shell::prelude::*;

pub use super::base::*;
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
    pub time: ResMut<'w, ActionListSceneTime>,
    pub fogcolor: ResMut<'w, ActionListSceneFogColor>,
    pub fogparam: ResMut<'w, ActionListSceneFogParam>,
    pub ambientcolor: ResMut<'w, ActionListSceneAmbientColor>,
    pub ambientintensity: ResMut<'w, ActionListSceneAmbientIntensity>,
    pub animeenable: ResMut<'w, ActionListSceneAnimationEnable>,
}

