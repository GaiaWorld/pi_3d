use std::{marker::PhantomData, fmt::Debug};

use pi_animation::{type_animation_context::{TypeAnimationContext}};
use pi_assets::{asset::{GarbageEmpty}, mgr::AssetMgr};
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator}};

use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;

use self::{
    base::*,
    system::*,
    command_sys::*,
    command::*,
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
        app.add_system(sys_scene_anime_ctx.run_if(should_run).in_set(ERunStageChap::AnimeAmount));
    }
}
