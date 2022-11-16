use pi_ecs::{prelude::{ArchetypeId, StageBuilder}, world::World};

use crate::{engine::Engine, run_stage::RunStage};

pub enum ErrorPlugin {
    StageError(&'static str),
    ResourceError(&'static str),
}

pub trait Plugin {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut Engine,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin>;
}