use pi_ecs::{prelude::{Setup, }};
use pi_engine_shell::{run_stage::ERunStageChap};

use super::{uniform::{SysUpdateValueUniform, SysEffectValueUniformUpdate}, vec2::Vec2Uniform};


pub struct PluginVec2Slot;
impl pi_engine_shell::plugin::Plugin for PluginVec2Slot {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysUpdateValueUniform::<Vec2Uniform>::setup(world, stages.query_stage::<SysEffectValueUniformUpdate>(ERunStageChap::Command));

        Ok(())
    }
}