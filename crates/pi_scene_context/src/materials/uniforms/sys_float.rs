use pi_ecs::{prelude::{Setup, }};
use pi_engine_shell::{plugin::Plugin};

use super::{uniform::SysUpdateValueUniform, float::FloatUniform};

pub struct PluginFloatSlot;
impl Plugin for PluginFloatSlot {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysUpdateValueUniform::<FloatUniform>::setup(world, stages.uniform_update());

        Ok(())
    }
}