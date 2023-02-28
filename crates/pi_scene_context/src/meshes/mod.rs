
use pi_ecs::prelude::Setup;
use pi_engine_shell::run_stage::ERunStageChap;

use crate::{object::ObjectID, geometry::instance::{InstanceSourceRecord}};

use self::{
    command::{SysMeshCreateCommand, SingleMeshCreateCommandList, SysMeshModifyCommand, SysInstanceMeshCreateCommand, SysInstanceMeshModifyCommand, SingleMeshModifyCommandList, SingleInstanceMeshCreateCommandList, SingleInstanceMeshModifyCommandList}, 
    model::{SysRenderMatrixUniformUpdate, SysRenderMatrixUpdate},
    instance::{
        world_matrix::{SysInstanceBufferWorldMatrixUpdate},
        instance_color::{SysInstanceBufferColorUpdate},
        instance_tilloff::{SysInstanceBufferTillOffUpdate}
    },
    alpha_index::PluginAlphaIndex
};

pub mod model;
pub mod command;
pub mod interface;
pub mod alpha_index;
pub mod render_group;
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

        SysMeshCreateCommand::setup(world, stages.query_stage::<SysMeshCreateCommand>(ERunStageChap::Initial));
        SysMeshModifyCommand::setup(world, stages.query_stage::<SysMeshModifyCommand>(ERunStageChap::Initial));
        SysInstanceMeshCreateCommand::setup(world, stages.query_stage::<SysInstanceMeshCreateCommand>(ERunStageChap::Initial));
        SysInstanceMeshModifyCommand::setup(world, stages.query_stage::<SysInstanceMeshModifyCommand>(ERunStageChap::Initial));
    
        // SysModelAboutUpdate::setup(world, stages.query_stage::<SysModelAboutUpdate>(ERunStageChap::Command));
        SysRenderMatrixUpdate::setup(world, stages.query_stage::<SysRenderMatrixUpdate>(ERunStageChap::Command));
        SysRenderMatrixUniformUpdate::setup(world, stages.query_stage::<SysRenderMatrixUniformUpdate>(ERunStageChap::Command));

        SysInstanceBufferWorldMatrixUpdate::setup(world, stages.query_stage::<SysInstanceBufferWorldMatrixUpdate>(ERunStageChap::Uniform));
        
        SysInstanceBufferColorUpdate::setup(world, stages.query_stage::<SysInstanceBufferColorUpdate>(ERunStageChap::Uniform));

        SysInstanceBufferTillOffUpdate::setup(world, stages.query_stage::<SysInstanceBufferTillOffUpdate>(ERunStageChap::Uniform));

        // SysModelAboutBindGroup::setup(world, stages.query_stage::<SysModelAboutBindGroup>(ERunStageChap::Uniform));

        world.insert_resource(SingleMeshCreateCommandList::default());
        world.insert_resource(SingleMeshModifyCommandList::default());
        world.insert_resource(SingleInstanceMeshCreateCommandList::default());
        world.insert_resource(SingleInstanceMeshModifyCommandList::default());
        world.insert_resource(InstanceSourceRecord { counter: 0 });

        PluginAlphaIndex.init(engine, stages);

        Ok(())
    }
}
