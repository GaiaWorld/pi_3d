use pi_ecs::prelude::{ArchetypeId, StageBuilder};

use crate::{engine::Engine, run_stage::RunStage};

pub enum ErrorPlugin {
    StageError(&'static str),
    ResourceError(&'static str),
}

pub trait Plugin {
    fn init(
        engine: &mut Engine,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin>;
}