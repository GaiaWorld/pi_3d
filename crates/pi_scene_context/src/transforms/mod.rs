
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
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ActionListTransformNodeCreate::default())
            .insert_resource(ActionListTransformNodeLocalPosition::default())
            // .insert_resource(ActionListTransformNodeLocalRotation::default())
            .insert_resource(ActionListTransformNodeLocalEuler::default())
            .insert_resource(ActionListTransformNodeLocalRotationQuaternion::default())
            .insert_resource(ActionListTransformNodeLocalScaling::default())
            .insert_resource(ActionListTransformNodeParent::default())
            .insert_resource(StateTransform::default())
            .insert_resource(TransformDirtyRoots::default())
            ;

        app.configure_set(Update, StageTransform::TransformCreate.after(StageScene::Create));
        app.configure_set(Update, StageTransform::_TransformCreate.after(StageTransform::TransformCreate).before(StageEnable::Command));
        app.configure_set(Update, StageTransform::TransformCommand.after(StageTransform::_TransformCreate).before(EStageAnimation::Create));
        // app.configure_set(Update, StageTransform::TransformCommandApply.after(StageTransform::TransformCommand));
        app.configure_set(Update, StageTransform::TransformCalcMatrix.after(StageTransform::TransformCommand).after(EStageAnimation::Running).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageTransform::_TransformCreate));

        app.add_systems(Update, 
            sys_create_transform_node.in_set(StageTransform::TransformCreate),
        );
        app.add_systems(Update, 
            sys_act_transform_parent.in_set(StageTransform::TransformCommand),
        );
        app.add_systems(
			Update,
            (
                sys_act_local_euler,
                sys_act_local_position,
                sys_act_local_scaling,
            ).in_set(StageTransform::TransformCommand)
        );
        app.add_systems(
			Update,
            (
                sys_local_euler_calc_rotation,
                sys_act_local_rotation,
                sys_local_quaternion_calc_rotation,
                sys_local_matrix_calc,
                sys_tree_layer_changed,
                sys_world_matrix_calc,
                sys_world_matrix_calc2,
            ).chain().run_if(should_run).in_set(StageTransform::TransformCalcMatrix)
        );

        app.add_systems(
			Update,
            sys_dispose_about_transform_node.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        );
    }
}

pub struct PluginGroupTransformNode;
impl PluginGroupTransformNode {
    pub fn add(group: PluginGroupBuilder) -> PluginGroupBuilder {
        group
            .add(PluginTransformNode)
            .add(PluginAnimeLocalPosition::new())
            .add(PluginAnimeLocalEuler::new())
            .add(PluginAnimeLocalQuaternion::new())
            .add(PluginAnimeLocalScaling::new())
    }
}
