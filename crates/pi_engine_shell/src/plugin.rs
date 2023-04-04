use pi_ecs::{prelude::{ArchetypeId, StageBuilder}, world::World};

use crate::{engine_shell::EnginShell, run_stage::RunStage};

pub enum ErrorPlugin {
    StageError(&'static str),
    ResourceError(&'static str),
}

pub use bevy::prelude::Plugin;