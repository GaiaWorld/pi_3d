
use pi_engine_shell::prelude::*;

use crate::{
    object::ObjectID,
    geometry::instance::{
        sys_instance::*,
        instance_color::*,
        instance_tilloff::*,
        instance_world_matrix::{InstanceBufferWorldMatrix, InstanceWorldMatrixDirty},
        InstanceSourceID, DirtyInstanceSourceRefs
    }
};

use self::{
    command::*, 
    model::*,
    instance::*,
};

pub mod model;
pub mod command;
pub mod interface;
// pub mod alpha_index;
pub mod render_group;
pub mod instance;
pub mod abstract_mesh;
pub mod skeleton;
pub mod shader_about;
pub mod bind_group;
pub mod lighting;

#[derive(Component)]
pub struct Mesh;

#[derive(Component)]
pub struct MeshID(pub ObjectID);
impl TEntityRef for MeshID {
    fn id(&self) -> Entity {
        self.0
    }
}

#[derive(Debug, Clone, Default, Component)]
pub struct DirtyMeshRef;

pub type MeshRefs = EntityRefInfo<DirtyMeshRef, MeshID>;


pub struct PluginMesh;
impl crate::Plugin for PluginMesh {
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     SysMeshCreateCommand::setup(world, stages.query_stage::<SysMeshCreateCommand>(ERunStageChap::Initial));
    //     SysMeshModifyCommand::setup(world, stages.query_stage::<SysMeshModifyCommand>(ERunStageChap::Initial));
    //     SysInstanceMeshCreateCommand::setup(world, stages.query_stage::<SysInstanceMeshCreateCommand>(ERunStageChap::Initial));
    //     SysInstanceMeshModifyCommand::setup(world, stages.query_stage::<SysInstanceMeshModifyCommand>(ERunStageChap::Initial));
    
    //     // SysModelAboutUpdate::setup(world, stages.query_stage::<SysModelAboutUpdate>(ERunStageChap::Command));
    //     SysRenderMatrixUpdate::setup(world, stages.query_stage::<SysRenderMatrixUpdate>(ERunStageChap::Command));
    //     SysRenderMatrixUniformUpdate::setup(world, stages.query_stage::<SysRenderMatrixUniformUpdate>(ERunStageChap::Command));

    //     SysInstanceBufferWorldMatrixUpdate::setup(world, stages.query_stage::<SysInstanceBufferWorldMatrixUpdate>(ERunStageChap::Command));
    //     SysInstanceBufferColorUpdate::setup(world, stages.query_stage::<SysInstanceBufferColorUpdate>(ERunStageChap::Command));
    //     SysInstanceBufferTillOffUpdate::setup(world, stages.query_stage::<SysInstanceBufferTillOffUpdate>(ERunStageChap::Command));

    //     // SysModelAboutBindGroup::setup(world, stages.query_stage::<SysModelAboutBindGroup>(ERunStageChap::Uniform));

    //     world.insert_resource(SingleMeshCreateCommandList::default());
    //     world.insert_resource(SingleMeshModifyCommandList::default());
    //     world.insert_resource(SingleInstanceMeshCreateCommandList::default());
    //     world.insert_resource(SingleInstanceMeshModifyCommandList::default());
    //     world.insert_resource(InstanceSourceRecord { counter: 0 });

    //     PluginAlphaIndex.init(engine, stages);

    //     Ok(())
    // }

    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ActionListMeshCreate::default());
        app.insert_resource(ActionListInstanceMeshCreate::default());
        app.insert_resource(ActionListMeshModify::default());
        app.insert_resource(ActionListInstanceColor::default());
        app.insert_resource(ActionListInstanceTillOff::default());

        app.add_system(
            sys_act_mesh_create.in_set(ERunStageChap::Initial)
        );
        app.add_system(
            sys_act_instanced_mesh_create.in_set(ERunStageChap::Initial)
        );
        app.add_systems(
            (
                sys_act_mesh_modify,
                sys_act_instance_color,
                sys_act_instance_tilloff,
            ).in_set(ERunStageChap::Command)
        );
        app.add_system(
            sys_calc_render_matrix.in_set(ERunStageChap::CalcRenderMatrix)
        );
        app.add_system(
            sys_render_matrix_for_uniform.in_set(ERunStageChap::Uniform)
        );
        app.add_systems(
            (
                sys_tick_instance_buffer_update::<InstanceColor, InstanceBufferColor, InstanceColorDirty>,
                sys_tick_instance_buffer_update::<InstanceTillOff, InstanceBufferTillOff, InstanceTillOffDirty>,
                sys_tick_instance_buffer_update::<RenderWorldMatrix, InstanceBufferWorldMatrix, InstanceWorldMatrixDirty>,
            ).in_set(ERunStageChap::Uniform)
        );
    }
}
