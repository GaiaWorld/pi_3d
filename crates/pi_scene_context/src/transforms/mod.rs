
use pi_scene_shell::prelude::*;

use crate::{scene::StageScene, flags::StageEnable};

use self::{
    command::*,
    command_sys::*,
    transform_node_sys::*,
    animation::*,
    prelude::*,
};

pub mod transform_node;
pub mod transform_node_sys;
pub mod command;
pub mod command_sys;
pub mod interface;
pub mod animation;
pub mod tree_left_right;
pub mod object;
pub mod prelude;
mod system;

pub struct PluginTransformNode;
impl Plugin for PluginTransformNode {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ActionListTransformNodeCreate::default())
            .insert_resource(ActionListTransformNodeLocal::default())
            .insert_resource(ActionListTransformNodeLocalRotationQuaternion::default())
            .insert_resource(ActionListTransformNodeParent::default())
            .insert_resource(StateTransform::default())
            .insert_resource(TmpTransformWorldCalc0::default())
            .insert_resource(TmpTransformWorldCalc1::default())
            ;

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update, 
            (
                StageTransform::TransformCreate.after(StageScene::_SceneCreate),
                StageTransform::_TransformCreate.after(StageTransform::TransformCreate).before(StageEnable::Command),
                StageTransform::TransformCommand.after(StageTransform::_TransformCreate).before(EStageAnimation::Create),
                StageTransform::TransformCalcMatrix.after(StageTransform::TransformCommand).after(EStageAnimation::Running).before(ERunStageChap::Collect),
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(StageD3, StageTransform::TransformCreate      .in_set(ERunStageChap::Create).after(StageScene::_SceneCreate))
        .configure_set(StageD3, StageTransform::_TransformCreate     .in_set(ERunStageChap::Create).after(StageTransform::TransformCreate).before(StageEnable::Command))
        .configure_set(StageD3, StageTransform::TransformCommand     .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageTransform::_TransformCreate).before(EStageAnimation::Create))
        .configure_set(StageD3, StageTransform::TransformCalcMatrix  .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageTransform::TransformCommand).after(EStageAnimation::Running).before(ERunStageChap::Collect))
        .configure_set(StageD3, StageTransform::TransformDispose     .in_set(ERunStageChap::Dispose).before(StageScene::SceneDispose))
        ;

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update, 
            (
                apply_deferred.in_set(StageTransform::_TransformCreate),
                sys_create_transform_node.in_set(StageTransform::TransformCreate),
                (
                    sys_act_local,
                ).in_set(StageTransform::TransformCommand),
                (
                    sys_local_euler_calc_rotation,
                    sys_act_local_rotation,
                    sys_local_quaternion_calc_rotation,
                    sys_local_matrix_calc,
                    sys_transform_dirty,
                    sys_world_matrix_calc,
                ).chain().in_set(StageTransform::TransformCalcMatrix),
                sys_dispose_about_transform_node.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            )
        );

#[cfg(not(feature = "use_bevy"))]
{
    app
        .add_systems(StageD3, sys_create_transform_node
            // .run_if(runif_acts::<OpsTransformNode>)  
            .in_set(StageTransform::TransformCreate))
        .add_systems(StageD3, sys_act_local
            // .run_if(runif_acts2::<OpsTransformNodeLocal, OpsTransformNodeParent>)               
            .in_set(StageTransform::TransformCommand))
        .add_systems(StageD3, sys_local_euler_calc_rotation                                                           .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(StageD3, sys_act_local_rotation              .after(sys_local_euler_calc_rotation)       .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(StageD3, sys_local_quaternion_calc_rotation  .after(sys_act_local_rotation)                      .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(StageD3, sys_local_matrix_calc
            // .run_if(runif_changes::<FlagLocalMatrix>)               
            .after(sys_local_quaternion_calc_rotation)          .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(StageD3, sys_transform_dirty              .after(sys_local_matrix_calc)                       .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(StageD3, sys_world_matrix_calc
            // .run_if(runif_changes::<TransformNodeDirty>)              
            .after(sys_transform_dirty).in_set(StageTransform::TransformCalcMatrix))
        .add_systems(StageD3, sys_dispose_about_transform_node    .in_set(StageTransform::TransformDispose))
        ;
}

    }
}

pub struct PluginGroupTransformNode;
impl PluginGroupTransformNode {
    pub fn add(group: &mut App) -> &mut App {
        group
            .add_plugins(PluginTransformNode)
            .add_plugins(PluginAnimeLocalPosition::new())
            .add_plugins(PluginAnimeLocalEuler::new())
            .add_plugins(PluginAnimeLocalQuaternion::new())
            .add_plugins(PluginAnimeLocalScaling::new())
    }
}
