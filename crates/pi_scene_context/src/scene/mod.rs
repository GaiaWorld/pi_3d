
use pi_engine_shell::prelude::*;


use self::{
    environment::sys::*,
    command_sys::*,
    prelude::*,
};

pub mod coordinate_system;
pub mod command;
pub mod command_sys;
pub mod interface;
pub mod environment;
pub mod light;
pub mod passes_cfg;
pub mod prelude;


pub struct PluginScene;
impl Plugin for PluginScene {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListSceneCreate::default());

        app.add_system(
            sys_act_scene_create.in_set(ERunStageChap::Initial)
        );

        app.add_systems(
            (
                sys_bind_update_scene_ambient,
                sys_bind_update_scene_fog,
                sys_bind_update_scene_time
            ).in_set(ERunStageChap::Uniform)
        );
    }
    
}
