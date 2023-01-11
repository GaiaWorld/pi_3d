use pi_ecs::{prelude::{Setup, }};
use pi_engine_shell::{plugin::Plugin, run_stage::ERunStageChap};

use super::{uniform::{SysUpdateValueUniform, SysEffectValueUniformUpdate}, mat4::Mat4Uniform};

pub struct PluginMat4Slot;
impl pi_engine_shell::plugin::Plugin for PluginMat4Slot {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysUpdateValueUniform::<Mat4Uniform>::setup(world, stages.query_stage::<SysEffectValueUniformUpdate>(ERunStageChap::Command));

        Ok(())
    }
}