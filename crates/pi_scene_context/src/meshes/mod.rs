
use pi_engine_shell::prelude::*;

use crate::{
    geometry::prelude::*,
};

use self::{
    command::*, 
    command_sys::*, 
    model::*,
};

mod model;
mod command;
pub mod command_sys;
mod interface;
// pub mod alpha_index;
mod render_group;
mod instance;
mod abstract_mesh;
mod skeleton;
mod shader_about;
mod bind_group;
mod lighting;
pub mod prelude;

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
        app.insert_resource(ActionListMeshShadow::default());
        app.insert_resource(ActionListInstanceColor::default());
        app.insert_resource(ActionListInstanceTillOff::default());
        app.insert_resource(ActionListMeshRenderAlignment::default());
        app.insert_resource(ActionListAbstructMeshScalingMode::default());
        app.insert_resource(ActionListAbstructMeshVelocity::default());
        app.insert_resource(ActionListInstanceColors::default());
        app.insert_resource(ActionListInstanceTilloffs::default());
        app.insert_resource(ActionListInstanceWorldMatrixs::default());
        app.insert_resource(ActionListAbstructMeshEnable::default());

        app.add_system(
            sys_act_mesh_create.in_set(ERunStageChap::Initial)
        );
        app.add_system(
            sys_act_instanced_mesh_create.in_set(ERunStageChap::Initial)
        );
        app.add_systems(
            (
                sys_act_mesh_modify,
                sys_act_abstruct_mesh_enable,
                sys_act_abstruct_mesh_render_alignment,
                sys_act_abstruct_mesh_scaling_mode,
                sys_act_abstruct_mesh_velocity,
                sys_act_instance_color,
                sys_act_instance_tilloff,
            ).in_set(ERunStageChap::Command)
        );
        app.add_system(
            sys_calc_render_matrix.run_if(should_run).in_set(ERunStageChap::CalcRenderMatrix)
        );
        app.add_system(
            sys_calc_render_matrix_instance.run_if(should_run).after(sys_calc_render_matrix)
        );
        app.add_system(
            sys_render_matrix_for_uniform.run_if(should_run).in_set(ERunStageChap::Uniform)
        );
        app.add_systems(
            (
                sys_tick_instance_buffer_update::<InstanceColor, InstanceBufferColor, InstanceColorDirty>.run_if(should_run),
                sys_tick_instance_buffer_update::<InstanceTillOff, InstanceBufferTillOff, InstanceTillOffDirty>.run_if(should_run),
                sys_tick_instance_buffer_update::<RenderWorldMatrix, InstanceBufferWorldMatrix, InstanceWorldMatrixDirty>.run_if(should_run),
            ).in_set(ERunStageChap::Uniform)
        );
        app.add_systems(
            (
                sys_act_geomettry_instance_world_matrix.run_if(should_run),
                sys_act_geomettry_instance_color.run_if(should_run),
                sys_act_geomettry_instance_tilloff.run_if(should_run),
            ).before(sys_tick_instance_buffer_update::<InstanceColor, InstanceBufferColor, InstanceColorDirty>)
        );
    }
}