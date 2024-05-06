
use pi_scene_shell::{prelude::*, run_stage::should_run_with_lighting};

use crate::{
    geometry::prelude::*,
    object::sys_dispose_ready,
    transforms::prelude::*,
    cameras::prelude::StageCamera,
    materials::prelude::*,
    flags::StageEnable,
    cullings::prelude::*,
    light::prelude::*,
    layer_mask::StageLayerMask
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

    fn build(&self, app: &mut App) {
        app.world.insert_single_res(ActionListMeshCreate::default());
        app.world.insert_single_res(ActionListInstanceMeshCreate::default());
        app.world.insert_single_res(ActionListMeshShadow::default());
        app.world.insert_single_res(ActionListMeshRenderAlignment::default());
        app.world.insert_single_res(ActionListAbstructMeshScalingMode::default());
        app.world.insert_single_res(ActionListAbstructMeshVelocity::default());
        app.world.insert_single_res(ActionListInstanceAttr::default());
        app.world.insert_single_res(ActionListMeshRenderIndiceRange::default());
        app.world.insert_single_res(ActionListMeshRenderVertexRange::default());
        app.world.insert_single_res(ActionListBoneOffset::default());
        app.world.insert_single_res(ActionListMeshForcePointLighting::default());
        app.world.insert_single_res(ActionListMeshForceSpotLighting::default());
        app.world.insert_single_res(ActionListMeshForceHemiLighting::default());
        app.world.insert_single_res(ActionListTargetAnimationAttribute::default());

        // app.configure_set(Update, StageModel::CreateMesh.after(StageMaterial::Create));
        // app.configure_set(Update, StageModel::_InitMesh.after(StageModel::CreateMesh).before(StageLayerMask::Command).before(StageEnable::Command));
        // app.configure_set(Update, StageModel::CreateInstance.after(StageModel::_InitMesh));
        // app.configure_set(Update, StageModel::_InitInstance.after(StageModel::CreateInstance).before(StageEnable::Command).before(StageTransform::TransformCommand));
        // app.configure_set(Update, StageModel::AbstructMeshCommand.in_set(FrameDataPrepare).after(StageModel::_InitInstance).before(ERunStageChap::Uniform).before(EStageAnimation::Create));
        // app.configure_set(Update, StageModel::RenderMatrix.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageTransform::TransformCalcMatrix));
        // app.configure_set(Update, StageModel::InstanceEffectMesh.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageModel::RenderMatrix));
        // app.configure_set(Update, StageModel::InstanceEffectGeometry.in_set(FrameDataPrepare).after(StageModel::InstanceEffectMesh).after(StageCamera::CameraCulling).after(EStageAnimation::Running).before(ERunStageChap::Uniform));
        // app.configure_set(Update, StageModel::LightingCollect.in_set(FrameDataPrepare).after(StageLighting::LightingCommand).after(StageModel::InstanceEffectGeometry).before(ERunStageChap::Uniform));
        // app.add_system(Update, apply_deferred.in_set(StageModel::_InitMesh));
        // app.add_system(Update, apply_deferred.in_set(StageModel::_InitInstance));

        app.add_system(Update, 
            sys_create_mesh
        );
        app.add_system(Update, 
            sys_create_instanced_mesh
        );
        app.add_system(
			Update,
                sys_act_target_animation_attribute,
        );
        app.add_system(
			Update,
                sys_act_instance_attribute
        );

        app.add_system(
			Update,
                sys_act_mesh_modify
        );

        app.add_system(Update, sys_enable_about_instance);
        app.add_system(Update, 
            sys_calc_render_matrix
        );
        app.add_system(Update, 
                sys_calc_render_matrix_instance
        );
        app.add_system(
			Update,
                sys_render_matrix_for_uniform
        );
        app.add_system(
			Update,
                sys_velocity_for_uniform,
        );
        app.add_system(
			Update,
            
                sys_animator_update_instance_attribute 
        );
        app.add_system(
			Update,
                sys_tick_instanced_buffer_update        , 
        );
        app.add_system(
			Update,
                sys_tick_instanced_buffer_update_single 
        );
        app.add_system(
			Update,
                sys_tick_culling_box   
        );

        app.add_system(
			Update,
                sys_model_direct_lighting_modify_by_light
        );
        app.add_system(
			Update,
                sys_model_direct_lighting_modify_by_model
        );
        app.add_system(
			Update,
                sys_model_point_lighting_modify_by_model
        );
        app.add_system(
			Update,
                sys_model_spot_lighting_modify_by_model  
        );

        app.add_system(
			Update,
            
                sys_dispose_about_mesh,
        );
        app.add_system(
			Update,
                sys_dispose_about_instance
        );
        app.add_system(
			Update,
                sys_dispose_about_pass,
        );
        app.add_system(
			Update,sys_dispose_ready
        );
    }
}