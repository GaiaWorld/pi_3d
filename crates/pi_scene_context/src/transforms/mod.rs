
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
        app.world.insert_single_res(ActionListTransformNodeCreate::default());
        app.world.insert_single_res(ActionListTransformNodeLocal::default());
        app.world.insert_single_res(ActionListTransformNodeLocalRotationQuaternion::default());
        app.world.insert_single_res(ActionListTransformNodeParent::default());
        app.world.insert_single_res(StateTransform::default());
        app.world.insert_single_res(TransformDirtyRoots::default());


        // app.configure_set(Update, StageTransform::TransformCreate.after(StageScene::Create));
        // app.configure_set(Update, StageTransform::_TransformCreate.after(StageTransform::TransformCreate).before(StageEnable::Command));
        // app.configure_set(Update, StageTransform::TransformCommand.after(StageTransform::_TransformCreate).before(EStageAnimation::Create));
        // // app.configure_set(Update, StageTransform::TransformCommandApply.after(StageTransform::TransformCommand));
        // app.configure_set(Update, StageTransform::TransformCalcMatrix.after(StageTransform::TransformCommand).after(EStageAnimation::Running).before(ERunStageChap::Uniform));
        // app.add_system(Update, apply_deferred.in_set(StageTransform::_TransformCreate));

        // app.add_system(Update, 
        //     sys_create_transform_node.in_set(StageTransform::TransformCreate),
        // );
        // app.add_system(Update, 
        //     sys_act_transform_parent.in_set(StageTransform::TransformCommand),
        // );
        // app.add_system(
		// 	Update,
        //     (
        //         sys_act_local,
        //     ).in_set(StageTransform::TransformCommand)
        // );
        // app.add_system(
		// 	Update,
        //     (
        //         sys_local_euler_calc_rotation,
        //         sys_act_local_rotation,
        //         sys_local_quaternion_calc_rotation,
        //         sys_local_matrix_calc,
        //         sys_tree_layer_changed,
        //         sys_world_matrix_calc,
        //         sys_world_matrix_calc2,
        //     ).chain().in_set(StageTransform::TransformCalcMatrix)
        // );

        // app.add_system(
		// 	Update,
        //     sys_dispose_about_transform_node.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        // );
        

        app.add_system(Update, sys_create_transform_node);
        app.add_system(Update, sys_act_transform_parent);
        app.add_system(Update,sys_act_local);
        app.add_system(Update,sys_local_euler_calc_rotation,);
        app.add_system(Update, sys_act_local_rotation,);
        app.add_system(Update,sys_local_quaternion_calc_rotation,);
        app.add_system(Update,sys_local_matrix_calc);
        app.add_system(Update,sys_tree_layer_changed,);
        app.add_system(Update,sys_world_matrix_calc,);
        app.add_system(Update,sys_world_matrix_calc2,);

        app.add_system(Update,sys_dispose_about_transform_node);
    }
}

pub struct PluginGroupTransformNode;
impl Plugin for PluginGroupTransformNode {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PluginTransformNode)
            .add_plugins(PluginAnimeLocalPosition::new())
            .add_plugins(PluginAnimeLocalEuler::new())
            .add_plugins(PluginAnimeLocalQuaternion::new())
            .add_plugins(PluginAnimeLocalScaling::new());
    }
    // pub fn add(group: ) -> PluginGroupBuilder {
    //     group
    //         .add(PluginTransformNode)
    //         .add(PluginAnimeLocalPosition::new())
    //         .add(PluginAnimeLocalEuler::new())
    //         .add(PluginAnimeLocalQuaternion::new())
    //         .add(PluginAnimeLocalScaling::new())
    // }
}
