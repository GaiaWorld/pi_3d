
use pi_engine_shell::prelude::*;

use self::{
    system::*,
};

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
        app.add_systems(Update, sys_scene_anime_ctx.run_if(should_run).in_set(ERunStageChap::AnimeAmount));
        app.add_systems(Update, sys_dispose_about_animationgroup.in_set(ERunStageChap::Dispose));
    }
}
