
use pi_ecs::prelude::Setup;

use crate::object::ObjectID;

use self::{command::{SysMeshCommand, SingleMeshCommandList}, model::SysModelUniformUpdate};

pub mod cube;
pub mod plane;
pub mod model;
pub mod command;
pub mod interface;
pub mod alpha_index;
pub mod render_group;
pub mod ball;
pub mod quad;

pub trait Mesh {
    fn alpha_index(&self) -> usize;
    fn render_group(&self) -> u8;
}

pub struct MeshID(pub ObjectID);

pub struct PluginMesh;
impl crate::Plugin for PluginMesh {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysMeshCommand::setup(world, stages.command_stage());
        SysModelUniformUpdate::setup(world, stages.uniform_update());

        world.insert_resource(SingleMeshCommandList::default());

        Ok(())
    }
}
