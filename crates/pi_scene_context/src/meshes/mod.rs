
use pi_engine_shell::prelude::*;

use crate::{
    geometry::prelude::*,
    object::sys_dispose_ready,
    transforms::prelude::*,
    cameras::prelude::StageCamera
};

use self::{
    command::*, 
    command_sys::*, 
    model::*,
    system::*, sys_lighting::*,
};

mod system;
mod sys_lighting;
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
        app.insert_resource(ActionListInstanceFloat::default());
        app.insert_resource(ActionListInstanceWorldMatrixs::default());
        app.insert_resource(ActionListMeshRenderIndiceRange::default());
        app.insert_resource(ActionListMeshRenderVertexRange::default());
        app.insert_resource(ActionListBoneOffset::default());
        app.insert_resource(ActionListMeshForcePointLighting::default());
        app.insert_resource(ActionListMeshForceSpotLighting::default());
        app.insert_resource(ActionListMeshForceHemiLighting::default());

        app.configure_set(Update, StageModel::InstanceInit.after(ERunStageChap::_InitialApply));
        app.configure_set(Update, StageModel::InstanceInitApply.after(StageModel::InstanceInit).before(StageTransform::TransformCommand));
        app.configure_set(Update, StageModel::AbstructMeshCommand.after(StageModel::InstanceInitApply).before(ERunStageChap::Uniform));
        app.configure_set(Update, StageModel::InstanceEffectMesh.after(StageModel::AbstructMeshCommand).after(StageTransform::TransformCalcMatrix));
        app.configure_set(Update, StageModel::InstanceEffectGeometry.after(StageModel::InstanceEffectMesh).after(StageCamera::CameraCulling).before(ERunStageChap::Uniform));
        app.configure_set(Update, StageModel::LightingCollect.after(StageModel::InstanceEffectGeometry).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageModel::InstanceInitApply));

        app.add_systems(Update, 
            sys_create_mesh.in_set(ERunStageChap::Initial)
        );
        app.add_systems(Update, 
            sys_act_instanced_mesh_create.in_set(StageModel::InstanceInit)
        );
        app.add_systems(Update, 
            sys_instance_color.in_set(StageModel::AbstructMeshCommand)
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
                sys_act_instance_float,
                sys_act_mesh_render_indice,
                sys_act_mesh_render_vertex_range,
                sys_act_mesh_force_point_lighting,
                sys_act_mesh_force_spot_lighting,
                sys_act_mesh_force_hemi_lighting,
            ).before(sys_instance_color).in_set(StageModel::AbstructMeshCommand)
        );
        app.add_systems(Update, 
            sys_enable_about_instance.in_set(StageModel::InstanceEffectMesh)
        );
        app.add_systems(Update, 
            sys_calc_render_matrix.in_set(StageModel::InstanceEffectMesh)
        );
        app.add_systems(Update, 
            sys_calc_render_matrix_instance.after(sys_calc_render_matrix).in_set(StageModel::InstanceEffectMesh)
        );
        app.add_systems(
			Update,
            (
                sys_render_matrix_for_uniform,
                sys_velocity_for_uniform,
                sys_skinoffset_for_uniform,
            ).in_set(ERunStageChap::Uniform)
        );
        app.add_systems(
			Update,
            (
                sys_tick_instanced_buffer_update.run_if(should_run),
                sys_tick_instanced_buffer_update_single,
            ).chain().in_set(StageModel::InstanceEffectGeometry)
        );

        app.add_systems(
			Update,
            (
                sys_model_direct_lighting_modify_by_light.run_if(should_run),
                sys_model_direct_lighting_modify_by_model.run_if(should_run),
                sys_model_point_lighting_modify_by_model.run_if(should_run),
                sys_model_spot_lighting_modify_by_model.run_if(should_run),
                sys_tick_instanced_buffer_update_single,
            ).chain().in_set(StageModel::LightingCollect)
        );

        app.add_systems(
			Update,
            (
                sys_dispose_about_mesh,
                sys_dispose_about_instance,
                sys_dispose_about_pass,
            ).after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        );
    }
}