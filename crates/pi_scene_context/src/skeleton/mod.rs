use pi_ecs::prelude::Setup;
use pi_engine_shell::{plugin::Plugin, run_stage::ERunStageChap};

use self::sys::SysSkinTextureUpdate;

pub mod row;
pub mod frames;
pub mod bone;
pub mod skeleton;
pub mod skin_texture;
pub mod sys;
pub mod command;

pub struct PluginSkeleton;
impl Plugin for PluginSkeleton {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();


        SysSkinTextureUpdate::setup(world, stages.query_stage::<SysSkinTextureUpdate>(ERunStageChap::Command));
        
        Ok(())
    }
}