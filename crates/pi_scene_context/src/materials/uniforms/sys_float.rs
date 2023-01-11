use pi_ecs::{prelude::{Setup, }};
use pi_engine_shell::{plugin::Plugin, run_stage::ERunStageChap};

use super::{uniform::{SysUpdateValueUniform, SysEffectValueUniformUpdate}, float::FloatUniform};

pub struct PluginFloatSlot;
impl Plugin for PluginFloatSlot {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysUpdateValueUniform::<FloatUniform>::setup(world, stages.query_stage::<SysEffectValueUniformUpdate>(ERunStageChap::Command));

        Ok(())
    }
}