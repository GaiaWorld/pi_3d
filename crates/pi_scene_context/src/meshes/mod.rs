
use pi_scene_shell::prelude::*;

use crate::{
    cameras::prelude::StageCamera, flags::StageEnable, geometry::prelude::*, layer_mask::StageLayerMask, light::prelude::*, object::sys_dispose_ready, prelude::{sys_tick_viewer_culling, LayerMask, SceneDirectLightsQueue, StageViewer}, scene::StageScene, transforms::{prelude::*, transform_node_sys::sys_world_matrix_calc}
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
        app.insert_resource(ActionListAbstractMeshPose::default());

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update, 
            (
                StageModel::MeshCreate.after(StageCamera::_CameraCreate),
                StageModel::_InitMesh.after(StageModel::MeshCreate).before(StageLayerMask::Command).before(StageEnable::Command),
                StageModel::InstanceCreate.after(StageModel::_InitMesh),
                StageModel::_InitInstance.after(StageModel::InstanceCreate).before(StageEnable::Command).before(StageTransform::TransformCommand),
                StageModel::AbstructMeshCommand.in_set(FrameDataPrepare).after(StageModel::_InitInstance).before(ERunStageChap::Collect).before(EStageAnimation::Create),
                StageModel::RenderMatrix.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageTransform::TransformCalcMatrix),
                StageModel::InstanceEffectMesh.in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageModel::RenderMatrix),
                StageModel::InstanceEffectGeometry.in_set(FrameDataPrepare).after(StageModel::InstanceEffectMesh).after(StageViewer::Culling).after(EStageAnimation::Running).before(ERunStageChap::Collect),
                StageModel::LightingCollect.in_set(FrameDataPrepare).after(StageLighting::LightingCommand).after(StageModel::InstanceEffectGeometry).before(ERunStageChap::Collect),
            )
        );
#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(StageD3, StageModel::MeshCreate            .in_set(ERunStageChap::Create).after(StageCamera::_CameraCreate))
        .configure_set(StageD3, StageModel::_InitMesh             .in_set(ERunStageChap::Create).after(StageModel::MeshCreate).before(StageLayerMask::Command).before(StageEnable::Command))
        .configure_set(StageD3, StageModel::InstanceCreate        .in_set(ERunStageChap::Create).after(StageModel::_InitMesh))
        .configure_set(StageD3, StageModel::_InitInstance         .in_set(ERunStageChap::Create).after(StageModel::InstanceCreate))
        .configure_set(StageD3, StageModel::AbstructMeshCommand   .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageModel::_InitInstance).before(ERunStageChap::Collect).before(EStageAnimation::Create))
        .configure_set(StageD3, StageModel::RenderMatrix          .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageModel::AbstructMeshCommand).after(StageTransform::TransformCalcMatrix))
        .configure_set(StageD3, StageModel::InstanceEffectMesh    .in_set(ERunStageChap::Culled).in_set(FrameDataPrepare).after(StageModel::RenderMatrix))
        .configure_set(StageD3, StageModel::InstanceEffectGeometry.in_set(ERunStageChap::Culled).in_set(FrameDataPrepare).after(StageModel::InstanceEffectMesh).before(StageGeometry::GeometryLoaded))
        .configure_set(StageD3, StageModel::LightingCollect       .in_set(ERunStageChap::Culled).in_set(FrameDataPrepare).after(StageModel::InstanceEffectGeometry))
        .configure_set(StageD3, StageModel::MeshDispose           .in_set(ERunStageChap::Dispose).before(StageScene::SceneDispose))
        ;

#[cfg(feature = "use_bevy")]
        app.add_systems(Update, 
            (
                apply_deferred.in_set(StageModel::_InitMesh),
                apply_deferred.in_set(StageModel::_InitInstance),
                sys_create_mesh.in_set(StageModel::MeshCreate),
                sys_create_instanced_mesh.in_set(StageModel::InstanceCreate),
                sys_create_abstract_posematrix.in_set(StageModel::InstanceCreate),
                (
                    sys_act_target_animation_attribute,
                    sys_act_instance_attribute.after(sys_act_target_animation_attribute),
                    sys_act_mesh_modify,
                ).in_set(StageModel::AbstructMeshCommand),
                sys_enable_about_instance.in_set(StageModel::InstanceEffectMesh),
                (
                    sys_calc_render_matrix_pre,
                    sys_calc_render_matrix,
                    // sys_render_matrix_with_posematrix,
                    sys_render_matrix_dirty,
                ).chain().in_set(StageModel::RenderMatrix),
                (
                    sys_model_for_uniform,
                ).in_set(ERunStageChap::Collect),
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
        .add_systems(Update, sys_create_mesh
            // .run_if(runif_acts::<OpsMeshCreation>)    
            .in_set(StageModel::MeshCreate))
        .add_systems(Update, sys_create_instanced_mesh
            // .run_if(runif_acts::<OpsInstanceMeshCreation>)    
            .in_set(StageModel::InstanceCreate))

        .add_systems(Update, sys_create_abstract_posematrix     .in_set(StageModel::InstanceCreate))
        .add_systems(Update, sys_act_mesh_modify
            // .run_if(runif_acts2::<OpsMeshStateModify, OpsAbstructMeshValueStateModify>)                                                                     
            .in_set(StageModel::AbstructMeshCommand))
        .add_systems(Update, sys_act_target_animation_attribute
            // .run_if(runif_acts::<OpsTargetAnimationAttribute>) 
            .in_set(StageModel::AbstructMeshCommand))
        .add_systems(Update, sys_act_instance_attribute
            // .run_if(runif_acts2::<OpsMeshForceLighting, OpsInstanceAttr>)          
            .after(sys_act_target_animation_attribute)  .in_set(StageModel::AbstructMeshCommand))
        .add_systems(Update, sys_instance_matidxs
            .in_set(StageModel::InstanceEffectMesh))    
        .add_systems(Update, sys_enable_about_instance               .in_set(StageModel::InstanceEffectMesh))
        .add_systems(Update, sys_calc_render_matrix_pre.after(sys_world_matrix_calc).in_set(StageTransform::TransformCalcMatrix))
        .add_systems(Update, sys_calc_render_matrix
            // .run_if(runif_changes::<FlagRenderWorldMatrix>)
            .after(sys_calc_render_matrix_pre)               .in_set(StageModel::RenderMatrix))
        .add_systems(Update, sys_render_matrix_dirty
            // .run_if(runif_changes::<RenderWorldMatrix>)   
            .after(sys_calc_render_matrix)  .in_set(StageModel::RenderMatrix))
        .add_systems(Update, sys_model_for_uniform       .in_set(ERunStageChap::Collect))
        .add_systems(Update, sys_animator_update_instance_attribute
            // .run_if(runif_changes::<TargetAnimatorableIsRunning>)
            .in_set(StageModel::InstanceEffectGeometry))  // .run_if(should_run),
        .add_systems(Update, sys_tick_instanced_buffer_update
            // .run_if(runif_comp::<InstanceSourceRefs>)       
            .after(sys_animator_update_instance_attribute ).in_set(StageModel::InstanceEffectGeometry))  // .run_if(should_run),
        .add_systems(Update, sys_tick_instanced_buffer_update_single
            // .run_if(runif_comp::<InstanceSourceRefs>) 
            .after(sys_tick_instanced_buffer_update       ).in_set(StageModel::InstanceEffectGeometry))  // .run_if(should_run),
        // .add_systems(Update, sys_tick_culling_box                   .after(sys_tick_instanced_buffer_update_single).in_set(StageModel::InstanceEffectGeometry))  // .run_if(should_run),
        .add_systems(Update, sys_model_direct_lighting_modify_by_light
            // .run_if(runif_changes::<SceneDirectLightsQueue>)                                                                       
            .in_set(StageModel::LightingCollect)) // .run_if(should_run_with_lighting),
        .add_systems(Update, sys_model_direct_lighting_modify_by_model
            // .run_if(runif_comp::<LayerMask>)       
            .after(sys_model_direct_lighting_modify_by_light       ).in_set(StageModel::LightingCollect)) // .run_if(should_run_with_lighting),
        .add_systems(Update, sys_model_point_lighting_modify_by_model
            // .run_if(runif_changes2::<LayerMask, SceneDirectLightsQueue>)        
            .after(sys_model_direct_lighting_modify_by_model       ).in_set(StageModel::LightingCollect)) // .run_if(should_run_with_lighting),
        .add_systems(Update, sys_dispose_about_instance
            // .run_if(runif_changes::<DisposeReady>)  
            .after(sys_dispose_ready).in_set(StageModel::MeshDispose))
        .add_systems(Update, sys_dispose_about_pass
            // .run_if(runif_changes::<DisposeReady>)    
            .after(sys_dispose_about_instance).in_set(StageModel::MeshDispose))
        .add_systems(Update, sys_dispose_about_mesh
            // .run_if(runif_changes2::<DisposeReady, InstanceSourceRefs>)      
            .after(sys_dispose_about_pass).in_set(StageModel::MeshDispose))
        ;

    }
}