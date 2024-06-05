
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
        app.insert_resource(ActionListInstanceAttr::default());
        app.insert_resource(ActionListMeshForceLighting::default());
        app.insert_resource(ActionListTargetAnimationAttribute::default());

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update, 
            (
                StageModel::MeshCreate.after(StageCamera::_Create),
                StageModel::_InitMesh.after(StageModel::MeshCreate).before(StageLayerMask::Command).before(StageEnable::Command),
                StageModel::InstanceCreate.after(StageModel::_InitMesh),
                StageModel::_InitInstance.after(StageModel::InstanceCreate).before(StageEnable::Command).before(StageTransform::TransformCommand),
                StageModel::AbstructMeshCommand.in_set(FrameDataPrepare).after(StageModel::_InitInstance).before(ERunStageChap::Uniform).before(EStageAnimation::Create),
                StageModel::RenderMatrix.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageTransform::TransformCalcMatrix),
                StageModel::InstanceEffectMesh.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageModel::RenderMatrix),
                StageModel::InstanceEffectGeometry.in_set(FrameDataPrepare).after(StageModel::InstanceEffectMesh).after(StageCamera::CameraCulling).after(EStageAnimation::Running).before(ERunStageChap::Uniform),
                StageModel::LightingCollect.in_set(FrameDataPrepare).after(StageLighting::LightingCommand).after(StageModel::InstanceEffectGeometry).before(ERunStageChap::Uniform),
            )
        );
#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(Update, StageModel::MeshCreate.after(StageCamera::_Create))
        .configure_set(Update, StageModel::_InitMesh.after(StageModel::MeshCreate).before(StageLayerMask::Command).before(StageEnable::Command))
        .configure_set(Update, StageModel::InstanceCreate.after(StageModel::_InitMesh))
        .configure_set(Update, StageModel::_InitInstance.after(StageModel::InstanceCreate).before(StageEnable::Command).before(StageTransform::TransformCommand))
        .configure_set(Update, StageModel::AbstructMeshCommand.in_set(FrameDataPrepare).after(StageModel::_InitInstance).before(ERunStageChap::Uniform).before(EStageAnimation::Create))
        .configure_set(Update, StageModel::RenderMatrix.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageTransform::TransformCalcMatrix))
        .configure_set(Update, StageModel::InstanceEffectMesh.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageModel::RenderMatrix))
        .configure_set(Update, StageModel::InstanceEffectGeometry.in_set(FrameDataPrepare).after(StageModel::InstanceEffectMesh).after(StageCamera::CameraCulling).after(EStageAnimation::Running).before(ERunStageChap::Uniform))
        .configure_set(Update, StageModel::LightingCollect.in_set(FrameDataPrepare).after(StageLighting::LightingCommand).after(StageModel::InstanceEffectGeometry).before(ERunStageChap::Uniform))
        ;

#[cfg(feature = "use_bevy")]
        app.add_systems(Update, 
            (
                apply_deferred.in_set(StageModel::_InitMesh),
                apply_deferred.in_set(StageModel::_InitInstance),
                sys_create_mesh.in_set(StageModel::MeshCreate),
                sys_create_instanced_mesh.in_set(StageModel::InstanceCreate),
                (
                    sys_act_target_animation_attribute,
                    sys_act_instance_attribute.after(sys_act_target_animation_attribute),
                    sys_act_mesh_modify,
                ).in_set(StageModel::AbstructMeshCommand),
                sys_enable_about_instance.in_set(StageModel::InstanceEffectMesh),
                (
                    sys_calc_render_matrix,
                    sys_calc_render_matrix_instance
                ).chain().in_set(StageModel::RenderMatrix),
                (
                    sys_render_matrix_for_uniform,
                    sys_velocity_for_uniform,
                ).in_set(ERunStageChap::Uniform),
                (
                    sys_animator_update_instance_attribute  , // .run_if(should_run),
                    sys_tick_instanced_buffer_update        , // .run_if(should_run),
                    sys_tick_instanced_buffer_update_single , // .run_if(should_run),
                    sys_tick_culling_box                    , // .run_if(should_run),
                ).chain().in_set(StageModel::InstanceEffectGeometry),
                (
                    sys_model_direct_lighting_modify_by_light       , // .run_if(should_run_with_lighting),
                    sys_model_direct_lighting_modify_by_model       , // .run_if(should_run_with_lighting),
                    sys_model_point_lighting_modify_by_model        , // .run_if(should_run_with_lighting),
                    sys_model_spot_lighting_modify_by_model         , // .run_if(should_run_with_lighting),
                ).chain().in_set(StageModel::LightingCollect),
                (
                    sys_dispose_about_mesh,
                    sys_dispose_about_instance,
                    sys_dispose_about_pass,
                ).after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(Update, sys_create_mesh     .in_set(StageModel::MeshCreate))
        .add_systems(Update, sys_create_instanced_mesh       .in_set(StageModel::InstanceCreate))
        
        .add_systems(Update, sys_act_mesh_modify                                                                     .in_set(StageModel::AbstructMeshCommand))
        .add_systems(Update, sys_act_target_animation_attribute                                                      .in_set(StageModel::AbstructMeshCommand))
        .add_systems(Update, sys_act_instance_attribute          .after(sys_act_target_animation_attribute)  .in_set(StageModel::AbstructMeshCommand))
        .add_systems(Update, sys_enable_about_instance               .in_set(StageModel::InstanceEffectMesh))
        .add_systems(Update, sys_calc_render_matrix                                                      .in_set(StageModel::RenderMatrix))
        .add_systems(Update, sys_calc_render_matrix_instance     .after(sys_calc_render_matrix)  .in_set(StageModel::RenderMatrix))
        .add_systems(Update, sys_render_matrix_for_uniform       .in_set(ERunStageChap::Uniform))
        .add_systems(Update, sys_velocity_for_uniform            .in_set(ERunStageChap::Uniform))
        .add_systems(Update, sys_animator_update_instance_attribute                                                        .in_set(StageModel::InstanceEffectGeometry))  // .run_if(should_run),
        .add_systems(Update, sys_tick_instanced_buffer_update       .after(sys_animator_update_instance_attribute ).in_set(StageModel::InstanceEffectGeometry))  // .run_if(should_run),
        .add_systems(Update, sys_tick_instanced_buffer_update_single.after(sys_tick_instanced_buffer_update       ).in_set(StageModel::InstanceEffectGeometry))  // .run_if(should_run),
        .add_systems(Update, sys_tick_culling_box                   .after(sys_tick_instanced_buffer_update_single).in_set(StageModel::InstanceEffectGeometry))  // .run_if(should_run),
        .add_systems(Update, sys_model_direct_lighting_modify_by_light                                                                       .in_set(StageModel::LightingCollect)) // .run_if(should_run_with_lighting),
        .add_systems(Update, sys_model_direct_lighting_modify_by_model       .after(sys_model_direct_lighting_modify_by_light       ).in_set(StageModel::LightingCollect)) // .run_if(should_run_with_lighting),
        .add_systems(Update, sys_model_point_lighting_modify_by_model        .after(sys_model_direct_lighting_modify_by_model       ).in_set(StageModel::LightingCollect)) // .run_if(should_run_with_lighting),
        .add_systems(Update, sys_model_spot_lighting_modify_by_model         .after(sys_model_point_lighting_modify_by_model        ).in_set(StageModel::LightingCollect)) // .run_if(should_run_with_lighting),
        .add_systems(Update, sys_dispose_about_mesh      .after(sys_dispose_ready).in_set(ERunStageChap::Dispose))
        .add_systems(Update, sys_dispose_about_instance  .after(sys_dispose_ready).in_set(ERunStageChap::Dispose))
        .add_systems(Update, sys_dispose_about_pass      .after(sys_dispose_ready).in_set(ERunStageChap::Dispose))
        ;

    }
}