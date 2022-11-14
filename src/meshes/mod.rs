
use pi_ecs::prelude::Setup;

use crate::object::ObjectID;

use self::{model::BuildinModelBind, command::{SysMeshCommand, SingleMeshCommandList}};

pub mod cube;
pub mod plane;
pub mod model;
pub mod command;
pub mod interface;
pub mod alpha_index;
pub mod render_group;

pub trait Mesh {
    fn alpha_index(&self) -> usize;
    fn render_group(&self) -> u8;
}

pub struct MeshID(pub ObjectID);

pub struct PluginMesh;
impl crate::Plugin for PluginMesh {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        SysMeshCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleMeshCommandList::default());

        Ok(())
    }
}
