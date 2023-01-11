use pi_ecs::{prelude::{Setup, }};
use pi_engine_shell::{plugin::Plugin, run_stage::ERunStageChap};

use super::{int::IntUniform, uniform::{SysUpdateValueUniform, SysEffectValueUniformUpdate}};


pub struct PluginIntSlot;
impl Plugin for PluginIntSlot {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();
        SysUpdateValueUniform::<IntUniform>::setup(world, stages.query_stage::<SysEffectValueUniformUpdate>(ERunStageChap::Command));

        Ok(())
    }
}