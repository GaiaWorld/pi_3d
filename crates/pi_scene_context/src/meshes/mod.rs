
use pi_engine_shell::prelude::*;

use crate::{
    geometry::prelude::*, object::sys_dispose_ready
};

use self::{
    command::*, 
    command_sys::*, 
    model::*,
    system::*,
};

mod system;
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
mod animation;
pub mod prelude;

pub struct PluginMesh;
impl crate::Plugin for PluginMesh {

    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ActionListMeshCreate::default());
        app.insert_resource(ActionListInstanceMeshCreate::default());
        app.insert_resource(ActionListMeshShadow::default());
        app.insert_resource(ActionListInstanceColor::default());
        app.insert_resource(ActionListInstanceAlpha::default());
        app.insert_resource(ActionListInstanceTillOff::default());
        app.insert_resource(ActionListMeshRenderAlignment::default());
        app.insert_resource(ActionListAbstructMeshScalingMode::default());
        app.insert_resource(ActionListAbstructMeshVelocity::default());
        app.insert_resource(ActionListInstanceColors::default());
        app.insert_resource(ActionListInstanceTilloffs::default());
        app.insert_resource(ActionListInstanceWorldMatrixs::default());
        app.insert_resource(ActionListMeshRenderIndiceRange::default());
        app.insert_resource(ActionListBoneOffset::default());

        app.add_systems(Update, 
            sys_create_mesh.in_set(ERunStageChap::Initial)
        );
        app.add_systems(Update, 
            sys_act_instanced_mesh_create.in_set(ERunStageChap::Initial)
        );
        app.add_systems(Update, 
            sys_instance_color.in_set(ERunStageChap::Command)
        );
        app.add_systems(
			Update,
            (
                sys_act_bone_offset,
                sys_act_mesh_modify,
                sys_act_abstruct_mesh_render_alignment,
                sys_act_abstruct_mesh_scaling_mode,
                sys_act_abstruct_mesh_velocity,
                sys_act_instance_color,
                sys_act_instance_alpha,
                sys_act_instance_tilloff,
                sys_act_mesh_render_indice,
            ).before(sys_instance_color).in_set(ERunStageChap::Command)
        );
        app.add_systems(Update, 
            sys_enable_about_instance.run_if(should_run).in_set(ERunStageChap::CalcRenderMatrix)
        );
        app.add_systems(Update, 
            sys_calc_render_matrix.run_if(should_run).in_set(ERunStageChap::CalcRenderMatrix)
        );
        app.add_systems(Update, 
            sys_calc_render_matrix_instance.run_if(should_run).after(sys_calc_render_matrix)
        );
        app.add_systems(
			Update,
            (
                sys_render_matrix_for_uniform.run_if(should_run),
                sys_velocity_for_uniform.run_if(should_run),
                sys_skinoffset_for_uniform.run_if(should_run),
            ).in_set(ERunStageChap::Uniform)
        );
        app.add_systems(
			Update,
            (
                sys_tick_instance_buffer_update::<InstanceColor, InstanceBufferColor, InstanceColorDirty>.run_if(should_run),
                sys_tick_instance_buffer_update::<InstanceTillOff, InstanceBufferTillOff, InstanceTillOffDirty>.run_if(should_run),
                sys_tick_instance_buffer_update::<RenderWorldMatrix, InstanceBufferWorldMatrix, InstanceWorldMatrixDirty>.run_if(should_run),
            ).chain().in_set(ERunStageChap::Uniform)
        );
        app.add_systems(
			Update,
            (
                sys_act_geomettry_instance_world_matrix.run_if(should_run),
                sys_act_geomettry_instance_color.run_if(should_run),
                sys_act_geomettry_instance_tilloff.run_if(should_run),
            ).chain().before(sys_tick_instance_buffer_update::<RenderWorldMatrix, InstanceBufferWorldMatrix, InstanceWorldMatrixDirty>)
        );

        app.add_systems(
			Update,
            (
                sys_dispose_about_mesh.run_if(should_run),
                sys_dispose_about_instance.run_if(should_run),
                sys_dispose_about_pass.run_if(should_run),
            ).after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        );
    }
}