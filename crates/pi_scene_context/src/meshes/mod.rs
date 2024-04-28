
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
        app.insert_resource(ActionListMeshCreate::default());
        app.insert_resource(ActionListInstanceMeshCreate::default());
        app.insert_resource(ActionListMeshStateModify::default());
        app.insert_resource(ActionListAbstructMeshValueStateModify::default());
        // app.insert_resource(ActionListAbstructMeshScalingMode::default());
        // app.insert_resource(ActionListAbstructMeshVelocity::default());
        app.insert_resource(ActionListInstanceAttr::default());
        // app.insert_resource(ActionListMeshRenderIndiceRange::default());
        // app.insert_resource(ActionListMeshRenderVertexRange::default());
        // app.insert_resource(ActionListBoneOffset::default());
        app.insert_resource(ActionListMeshForceLighting::default());
        // app.insert_resource(ActionListMeshForceSpotLighting::default());
        // app.insert_resource(ActionListMeshForceHemiLighting::default());
        app.insert_resource(ActionListTargetAnimationAttribute::default());

        app.configure_set(Update, StageModel::CreateMesh.after(StageMaterial::Create));
        app.configure_set(Update, StageModel::_InitMesh.after(StageModel::CreateMesh).before(StageLayerMask::Command).before(StageEnable::Command));
        app.configure_set(Update, StageModel::CreateInstance.after(StageModel::_InitMesh));
        app.configure_set(Update, StageModel::_InitInstance.after(StageModel::CreateInstance).before(StageEnable::Command).before(StageTransform::TransformCommand));
        app.configure_set(Update, StageModel::AbstructMeshCommand.in_set(FrameDataPrepare).after(StageModel::_InitInstance).before(ERunStageChap::Uniform).before(EStageAnimation::Create));
        app.configure_set(Update, StageModel::RenderMatrix.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageTransform::TransformCalcMatrix));
        app.configure_set(Update, StageModel::InstanceEffectMesh.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageModel::RenderMatrix));
        app.configure_set(Update, StageModel::InstanceEffectGeometry.in_set(FrameDataPrepare).after(StageModel::InstanceEffectMesh).after(StageCamera::CameraCulling).after(EStageAnimation::Running).before(ERunStageChap::Uniform));
        app.configure_set(Update, StageModel::LightingCollect.in_set(FrameDataPrepare).after(StageLighting::LightingCommand).after(StageModel::InstanceEffectGeometry).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageModel::_InitMesh));
        app.add_systems(Update, apply_deferred.in_set(StageModel::_InitInstance));

        app.add_systems(Update, 
            sys_create_mesh.in_set(StageModel::CreateMesh)
        );
        app.add_systems(Update, 
            sys_create_instanced_mesh.in_set(StageModel::CreateInstance)
        );
        app.add_systems(
			Update,
            (
                sys_act_target_animation_attribute,
                sys_act_instance_attribute.after(sys_act_target_animation_attribute),
                sys_act_mesh_modify,
            ).in_set(StageModel::AbstructMeshCommand)
        );
        app.add_systems(Update, 
            sys_enable_about_instance.in_set(StageModel::InstanceEffectMesh)
        );
        app.add_systems(Update, 
            (
                sys_calc_render_matrix,
                sys_calc_render_matrix_instance
            ).chain().in_set(StageModel::RenderMatrix)
        );
        app.add_systems(
			Update,
            (
                sys_render_matrix_for_uniform,
                sys_velocity_for_uniform,
            ).in_set(ERunStageChap::Uniform)
        );
        app.add_systems(
			Update,
            (
                sys_animator_update_instance_attribute  , // .run_if(should_run),
                sys_tick_instanced_buffer_update        , // .run_if(should_run),
                sys_tick_instanced_buffer_update_single , // .run_if(should_run),
                sys_tick_culling_box                    , // .run_if(should_run),
            ).chain().in_set(StageModel::InstanceEffectGeometry)
        );

        app.add_systems(
			Update,
            (
                sys_model_direct_lighting_modify_by_light       , // .run_if(should_run_with_lighting),
                sys_model_direct_lighting_modify_by_model       , // .run_if(should_run_with_lighting),
                sys_model_point_lighting_modify_by_model        , // .run_if(should_run_with_lighting),
                sys_model_spot_lighting_modify_by_model         , // .run_if(should_run_with_lighting),
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