
use pi_engine_shell::prelude::*;


use crate::{
    tree::*,
    flags::*
};

pub use super::transform_node::*;
pub use super::command::*;
pub use super::tree_left_right::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageTransform {
    TransformCommand,
    // TransformCommandApply,
    TransformCalcMatrix,
}

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

#[derive(Resource, Default)]
pub struct StateTransform {
    pub scene: Option<Entity>,
    pub count: usize,
    pub enable: usize,
    pub global_enable: usize,
    pub calc_world_time: u32,
    pub calc_local_time: u32,
    pub max_level: u32,
}

pub type StateTransformQuery = QueryState<(&'static SceneID, &'static Enable, &'static GlobalEnable)>;

pub fn sys_state_transform(
    items: Query<(&SceneID, &Enable, &GlobalEnable)>,
    mut state: ResMut<StateTransform>
) {
    state.count = 0;
    state.enable = 0;
    state.global_enable = 0;
    if let Some(scene) = state.scene {
        items.iter().for_each(|(idscene, enable, globalenable)| {
            if scene == idscene.0 {
                state.count += 1;
                if enable.bool() { state.enable += 1; }
                if globalenable.0 { state.global_enable += 1; }
            }
        });
    }
}
