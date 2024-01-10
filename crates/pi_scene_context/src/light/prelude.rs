
use pi_scene_shell::prelude::*;

pub use super::{
    command::*,
    base::*,
    spot::*,
};


#[derive(SystemParam)]
pub struct ActionSetLighting<'w> {
    pub create: ResMut<'w, ActionListLightCreate>,
    pub param: ResMut<'w, ActionListLightParam>,
    pub color: ResMut<'w, ActionListLightColor>,
    pub strength: ResMut<'w, ActionListLightStrength>,
    pub radius: ResMut<'w, ActionListLightRadius>,
    pub spotangle: ResMut<'w, ActionListSpotLightAngle>,
}