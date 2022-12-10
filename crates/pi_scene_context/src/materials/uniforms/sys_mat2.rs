use pi_ecs::{prelude::{Setup, }};
use pi_engine_shell::{plugin::Plugin};

use super::{mat2::Mat2Uniform, uniform::SysUpdateValueUniform};


pub struct PluginMat2Slot;
impl Plugin for PluginMat2Slot {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysUpdateValueUniform::<Mat2Uniform>::setup(world, stages.uniform_update());

        Ok(())
    }
}