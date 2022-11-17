use pi_ecs::{prelude::{ArchetypeId, StageBuilder}, world::World};

use crate::{engine_shell::EnginShell, run_stage::RunStage};

pub enum ErrorPlugin {
    StageError(&'static str),
    ResourceError(&'static str),
}

pub trait Plugin {
    fn init(
        &mut self,
        engine: &mut EnginShell,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin>;
}