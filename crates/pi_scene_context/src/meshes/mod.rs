
use pi_ecs::prelude::Setup;
use pi_engine_shell::run_stage::ERunStageChap;

use crate::{object::ObjectID, geometry::instance::{instance_world_matrix::SysInstanceBufferWorldMatrixInit, instance_color::SysInstanceBufferColorInit, instance_tilloff::SysInstanceBufferTillOffInit, InstanceSourceRecord}};

use self::{command::{SysMeshCommand, SingleMeshCommandList}, model::{SysRenderMatrixUniformUpdate, SysRenderMatrixUpdate, SysModelAboutUpdate}, instance::{world_matrix::{SysInstanceBufferWorldMatrixUpdate}, instance_color::{SysInstanceBufferColorUpdate}, instance_tilloff::{SysInstanceBufferTillOffUpdate}}, bind_group::SysModelAboutBindGroup};

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
pub mod bind_group;

pub struct Mesh;

pub struct MeshID(pub ObjectID);

pub struct PluginMesh;
impl crate::Plugin for PluginMesh {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysMeshCommand::setup(world, stages.query_stage::<SysMeshCommand>(ERunStageChap::Command));
        SysModelAboutUpdate::setup(world, stages.query_stage::<SysModelAboutUpdate>(ERunStageChap::Command));
        SysRenderMatrixUpdate::setup(world, stages.query_stage::<SysRenderMatrixUpdate>(ERunStageChap::Command));
        SysRenderMatrixUniformUpdate::setup(world, stages.query_stage::<SysRenderMatrixUniformUpdate>(ERunStageChap::Command));

        SysInstanceBufferWorldMatrixInit::setup(world, stages.query_stage::<SysInstanceBufferWorldMatrixInit>(ERunStageChap::Command));
        SysInstanceBufferWorldMatrixUpdate::setup(world, stages.query_stage::<SysInstanceBufferWorldMatrixUpdate>(ERunStageChap::Uniform));
        
        SysInstanceBufferColorInit::setup(world, stages.query_stage::<SysInstanceBufferColorInit>(ERunStageChap::Command));
        SysInstanceBufferColorUpdate::setup(world, stages.query_stage::<SysInstanceBufferColorUpdate>(ERunStageChap::Uniform));

        SysInstanceBufferTillOffInit::setup(world, stages.query_stage::<SysInstanceBufferTillOffInit>(ERunStageChap::Command));
        SysInstanceBufferTillOffUpdate::setup(world, stages.query_stage::<SysInstanceBufferTillOffUpdate>(ERunStageChap::Uniform));

        SysModelAboutBindGroup::setup(world, stages.query_stage::<SysModelAboutBindGroup>(ERunStageChap::Uniform));

        world.insert_resource(SingleMeshCommandList::default());
        world.insert_resource(InstanceSourceRecord { counter: 0 });

        Ok(())
    }
}
