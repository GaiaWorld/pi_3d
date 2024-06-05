
use pi_scene_shell::prelude::*;

use crate::{object::sys_dispose_ready, scene::StageScene, flags::StageEnable};

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
            .insert_resource(TransformDirtyRoots::default())
            ;

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update, 
            (
                StageTransform::TransformCreate.after(StageScene::_Create),
                StageTransform::_TransformCreate.after(StageTransform::TransformCreate).before(StageEnable::Command),
                StageTransform::TransformCommand.after(StageTransform::_TransformCreate).before(EStageAnimation::Create),
                StageTransform::TransformCalcMatrix.after(StageTransform::TransformCommand).after(EStageAnimation::Running).before(ERunStageChap::Uniform),
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(Update, StageTransform::TransformCreate.after(StageScene::_Create))
        .configure_set(Update, StageTransform::_TransformCreate.after(StageTransform::TransformCreate).before(StageEnable::Command))
        .configure_set(Update, StageTransform::TransformCommand.after(StageTransform::_TransformCreate).before(EStageAnimation::Create))
        .configure_set(Update, StageTransform::TransformCalcMatrix.after(StageTransform::TransformCommand).after(EStageAnimation::Running).before(ERunStageChap::Uniform))
        ;

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update, 
            (
                apply_deferred.in_set(StageTransform::_TransformCreate),
                sys_create_transform_node.in_set(StageTransform::TransformCreate),
                sys_act_transform_parent.in_set(StageTransform::TransformCommand),
                (
                    sys_act_local,
                ).in_set(StageTransform::TransformCommand),
                (
                    sys_local_euler_calc_rotation,
                    sys_act_local_rotation,
                    sys_local_quaternion_calc_rotation,
                    sys_local_matrix_calc,
                    sys_tree_layer_changed,
                    sys_world_matrix_calc,
                    sys_world_matrix_calc2,
                ).chain().in_set(StageTransform::TransformCalcMatrix),
                sys_dispose_about_transform_node.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            )
        );

#[cfg(not(feature = "use_bevy"))]
{
    app
        .add_systems(Update, sys_create_transform_node   .in_set(StageTransform::TransformCreate))
        .add_systems(Update, sys_act_transform_parent    .in_set(StageTransform::TransformCommand))
        .add_systems(Update, sys_act_local               .in_set(StageTransform::TransformCommand))
        .add_systems(Update, sys_local_euler_calc_rotation                                                           .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(Update, sys_act_local_rotation              .after(sys_local_euler_calc_rotation)       .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(Update, sys_local_quaternion_calc_rotation  .after(sys_act_local_rotation)                      .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(Update, sys_local_matrix_calc               .after(sys_local_quaternion_calc_rotation)          .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(Update, sys_tree_layer_changed              .after(sys_local_matrix_calc)                       .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(Update, sys_world_matrix_calc               .after(sys_tree_layer_changed)                      .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(Update, sys_world_matrix_calc2              .after(sys_world_matrix_calc)               .in_set(StageTransform::TransformCalcMatrix))
        .add_systems(Update, sys_dispose_about_transform_node    .in_set(StageTransform::TransformCreate))
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

