
use pi_engine_shell::prelude::*;


use crate::prelude::ActionListNodeEnable;

pub use super::transform_node::*;
pub use super::command::*;
pub use super::tree_left_right::*;


#[derive(SystemParam)]
pub struct ActionSetTransform<'w> {
    pub create: ResMut<'w, ActionListTransformNodeCreate>,
    pub localpos: ResMut<'w, ActionListTransformNodeLocalPosition>,
    pub localscl: ResMut<'w, ActionListTransformNodeLocalScaling>,
    pub localrot: ResMut<'w, ActionListTransformNodeLocalEuler>,
    pub localrotq: ResMut<'w, ActionListTransformNodeLocalRotationQuaternion>,
    pub tree: ResMut<'w, ActionListTransformNodeParent>,
    pub enable: ResMut<'w, ActionListNodeEnable>,
}
