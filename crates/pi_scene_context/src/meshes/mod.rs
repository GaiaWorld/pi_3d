
use pi_ecs::prelude::Setup;

use crate::object::ObjectID;

use self::{command::{SysMeshCommand, SingleMeshCommandList}, model::{SysModelUniformUpdate, SysModelMatrixUpdate, SysInstancedModelUpdate}, instance::{instanced_mesh::InstanceSourceRecord, world_matrix::{SysInstanceBufferWorldMatrixInit, SysInstanceBufferWorldMatrixUpdate}, instance_color::{SysInstanceBufferColorUpdate, SysInstanceBufferColorInit}, instance_tilloff::{SysInstanceBufferTillOffInit, SysInstanceBufferTillOffUpdate}}};

pub mod cube;
pub mod plane;
pub mod model;
pub mod command;
pub mod interface;
pub mod alpha_index;
pub mod render_group;
pub mod ball;
pub mod quad;
pub mod instance;
pub mod abstract_mesh;
pub mod skeleton;
pub mod shader_about;

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
        SysModelMatrixUpdate::setup(world, stages.command_stage());

        SysInstanceBufferWorldMatrixInit::setup(world, stages.command_stage());
        SysInstanceBufferWorldMatrixUpdate::setup(world, stages.uniform_update());
        
        SysInstanceBufferColorInit::setup(world, stages.command_stage());
        SysInstanceBufferColorUpdate::setup(world, stages.uniform_update());

        SysInstanceBufferTillOffInit::setup(world, stages.command_stage());
        SysInstanceBufferTillOffUpdate::setup(world, stages.uniform_update());

        SysModelUniformUpdate::setup(world, stages.uniform_update());
        SysInstancedModelUpdate::setup(world, stages.uniform_update());

        world.insert_resource(SingleMeshCommandList::default());
        world.insert_resource(InstanceSourceRecord { counter: 0 });

        Ok(())
    }
}
