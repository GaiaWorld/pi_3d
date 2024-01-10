use pi_scene_shell::prelude::*;

pub use super::base::*;
pub use super::command::*;
pub use super::direct_light::*;
pub use super::shader::*;


#[derive(SystemParam)]
pub struct ActionSetShadow<'w> {
    pub param: ResMut<'w, ActionListShadowGeneratorParam>,
    pub create: ResMut<'w, ActionListShadowGenerator>,
}