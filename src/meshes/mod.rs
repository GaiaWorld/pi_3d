
use pi_ecs::prelude::Setup;

use crate::object::ObjectID;

use self::{model::BuildinModelBind, command::{SysMeshCommand, SingleMeshCommandList}};

pub mod cube;
pub mod plane;
pub mod model;
pub mod command;
pub mod interface;

pub struct Mesh {
    materials: Vec<ObjectID>,
}
impl Default for Mesh {
    fn default() -> Self {
        Self {
            materials: vec![],
        }
    }
}

pub struct MeshID(pub ObjectID);

pub struct PluginMesh;
impl crate::Plugin for PluginMesh {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysMeshCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleMeshCommandList::default());

        Ok(())
    }
}
