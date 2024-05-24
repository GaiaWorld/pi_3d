
use pi_scene_shell::prelude::*;

use self::{system::*, command::ActionListPropertyTargetAnimation, command_sys::sys_act_add_property_target_animation};

pub mod base;
pub mod command;
pub mod command_sys;
pub mod system;
pub mod interface;
pub mod listen;
pub mod prelude;

pub struct PluginSceneAnimation;
impl Plugin for PluginSceneAnimation {
    fn build(&self, app: &mut App) {
        app.world.insert_single_res(ActionListPropertyTargetAnimation::default());
        app.add_system(
            Update, sys_scene_anime_ctx  // .run_if(should_run)
            .in_set(EStageAnimation::Running));
        app.add_system(Update, sys_act_add_property_target_animation.before(sys_act_animation_group_action).in_set(EStageAnimation::Command));
        // app.add_system(Update, sys_dispose_about_animationgroup.in_set(ERunStageChap::Dispose));
    }
}
