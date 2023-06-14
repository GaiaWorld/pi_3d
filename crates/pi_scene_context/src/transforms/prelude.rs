
use pi_engine_shell::prelude::*;

use crate::{
    animation::prelude::*
};

pub use super::transform_node::*;
pub use super::command::*;
pub use super::tree_left_right::*;


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
