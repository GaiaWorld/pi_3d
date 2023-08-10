
use pi_engine_shell::prelude::*;


use crate::object::sys_dispose_ready;

use self::{
    environment::sys::*,
    command_sys::*,
    prelude::*,
    system::*,
};

pub mod coordinate_system;
pub mod command;
pub mod command_sys;
pub mod interface;
pub mod environment;
pub mod light;
pub mod passes_cfg;
mod base;
mod system;
pub mod prelude;


pub struct PluginScene;
impl Plugin for PluginScene {
    fn build(&self, app: &mut App) {
        let id = app.world.spawn_empty().id();
        app.insert_resource(SingleEmptyEntity::new(id));

        app.insert_resource(ActionListSceneCreate::default());
        app.insert_resource(ActionListSceneTime::default());
        app.insert_resource(ActionListSceneAmbientColor::default());
        app.insert_resource(ActionListSceneAmbientIntensity::default());
        app.insert_resource(ActionListSceneFogColor::default());
        app.insert_resource(ActionListSceneFogParam::default());
        app.insert_resource(ActionListSceneAnimationEnable::default());

        app.add_system(
            sys_act_scene_create.in_set(ERunStageChap::Initial)
        );
        
        app.add_systems(
            (
                sys_act_scene_time,
                sys_act_scene_ambientcolor,
                sys_act_scene_ambientintensity,
                sys_act_scene_fogcolor,
                sys_act_scene_fogparam,
                sys_act_scene_animation_enable,
            ).in_set(ERunStageChap::Command)
        );

        app.add_systems(
            (
                sys_bind_update_scene_ambient,
                sys_bind_update_scene_fog,
                sys_bind_update_scene_time
            ).in_set(ERunStageChap::Uniform)
        );

        app.add_system(sys_dispose_about_scene.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));
    }
    
}
