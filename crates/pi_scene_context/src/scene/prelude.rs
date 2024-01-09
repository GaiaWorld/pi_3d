
use pi_engine_shell::prelude::*;

use crate::prelude::ActionListBoundingBoxDisplay;

pub use super::base::*;
pub use super::command::*;
pub use super::passes_cfg::*;
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
    pub time: ResMut<'w, ActionListSceneTime>,
    pub fogcolor: ResMut<'w, ActionListSceneFogColor>,
    pub fogparam: ResMut<'w, ActionListSceneFogParam>,
    pub ambientcolor: ResMut<'w, ActionListSceneAmbientColor>,
    pub ambientintensity: ResMut<'w, ActionListSceneAmbientIntensity>,
    pub animeenable: ResMut<'w, ActionListSceneAnimationEnable>,
    pub brdf: ResMut<'w, ActionListSceneBRDF>,
    pub env: ResMut<'w, ActionListSceneEnvTexture>,
    pub shadowmap: ResMut<'w, ActionListSceneShadowMap>,
    pub boundingboxdisplay: ResMut<'w, ActionListBoundingBoxDisplay>,
}

