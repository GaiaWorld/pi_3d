
use pi_engine_shell::prelude::*;

use crate::prelude::*;

use self::{
    command::*,
    transform_node_sys::*,
    animation::*, transform_node::{LocalScaling, LocalPosition, LocalEulerAngles}
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

#[derive(SystemParam)]
pub struct ActionSetTransform<'w> {
    pub create: ResMut<'w, ActionListTransformNodeCreate>,
    pub localpos: ResMut<'w, ActionListTransformNodeLocalPosition>,
    pub localscl: ResMut<'w, ActionListTransformNodeLocalScaling>,
    pub localrot: ResMut<'w, ActionListTransformNodeLocalEuler>,
    pub tree: ResMut<'w, ActionListTransformNodeParent>,
}

#[derive(SystemParam)]
pub struct ActionSetLocalPositionAnime<'w> {
    pub ctx: ResMut<'w, TypeAnimeContext<LocalPosition>>,
    pub curves: Res<'w, ShareAssetMgr<TypeFrameCurve<LocalPosition>>>,
}

#[derive(SystemParam)]
pub struct ActionSetLocalScalingAnime<'w> {
    pub ctx: ResMut<'w, TypeAnimeContext<LocalScaling>>,
    pub curves: Res<'w, ShareAssetMgr<TypeFrameCurve<LocalScaling>>>,
}

#[derive(SystemParam)]
pub struct ActionSetLocalEulerAnime<'w> {
    pub ctx: ResMut<'w, TypeAnimeContext<LocalEulerAngles>>,
    pub curves: Res<'w, ShareAssetMgr<TypeFrameCurve<LocalEulerAngles>>>,
}

#[derive(SystemParam)]
pub struct ActionSetTransformNodeAnime<'w> {
    pub position: ActionSetLocalPositionAnime<'w>,
    pub scaling: ActionSetLocalScalingAnime<'w>,
    pub euler: ActionSetLocalEulerAnime<'w>,
}
