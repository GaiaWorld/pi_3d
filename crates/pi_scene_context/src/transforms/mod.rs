
use pi_engine_shell::prelude::*;

use self::{
    command::*,
    transform_node_sys::*,
    animation::*
};

pub mod transform_node;
pub mod transform_node_sys;
pub mod command;
pub mod interface;
pub mod animation;
pub mod tree_left_right;
pub mod object;

pub struct PluginTransformNode;
impl Plugin for PluginTransformNode {
    fn build(&self, app: &mut bevy::prelude::App) {
        let id = app.world.spawn_empty().id();
        app.insert_resource(SingleEmptyEntity::new(id));
    
        app.insert_resource(ActionListTransformNodeCreate::default())
            .insert_resource(ActionListTransformNodeLocalEuler::default())
            .insert_resource(ActionListTransformNodeLocalPosition::default())
            .insert_resource(ActionListTransformNodeLocalScaling::default())
            .insert_resource(ActionListTransformNodeParent::default())
            ;

        app.add_system(
            sys_act_transform_node_create.in_set(ERunStageChap::Initial),
        );
        app.add_system(
            sys_act_transform_parent.in_set(ERunStageChap::SecondInitial),
        );
        app.add_systems(
            (
                sys_act_local_euler.in_set(ERunStageChap::Command),
                sys_act_local_position.in_set(ERunStageChap::Command),
                sys_act_local_scaling.in_set(ERunStageChap::Command),
            )
        );
        app.add_systems(
            (
                sys_local_euler_calc_rotation,
                sys_local_quaternion_calc_rotation,
                sys_local_matrix_calc,
                sys_world_matrix_calc,
                sys_world_matrix_calc2,
            ).chain().in_set(ERunStageChap::CalcWorldMatrix)
        );

    }
}

pub struct PluginGroupTransformNode;
impl PluginGroupTransformNode {
    pub fn add(group: PluginGroupBuilder) -> PluginGroupBuilder {
        group
            .add(PluginTransformNode)
            .add(PluginAnimeLocalPosition::new(false, 2 * 1024 * 1024, 1000))
            .add(PluginAnimeLocalEuler::new(false, 2 * 1024 * 1024, 1000))
            .add(PluginAnimeLocalQuaternion::new(false, 2 * 1024 * 1024, 1000))
            .add(PluginAnimeLocalScaling::new(false, 2 * 1024 * 1024, 1000))
    }
}