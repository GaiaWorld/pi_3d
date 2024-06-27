
use pi_scene_shell::prelude::*;

use crate::{flags::*, cullings::prelude::Collider};

use super::transform_node::*;

pub struct OpsTransformNode(pub Entity, pub Entity);
impl OpsTransformNode {
    pub fn ops(scene: Entity, node: Entity) -> Self {
        Self(scene, node)
    }
}
pub type ActionListTransformNodeCreate = ActionList<OpsTransformNode>;

pub struct OpsTransformNodeParent(pub Entity, pub Entity);
impl OpsTransformNodeParent {
    pub fn ops(node: Entity, parent: Entity) -> Self {
        Self(node, parent)
    }
}
pub type ActionListTransformNodeParent = ActionList<OpsTransformNodeParent>;

pub enum ETransformSRT {
    Euler(f32, f32, f32),
    Translation(f32, f32, f32),
    Scaling(f32, f32, f32),
}

pub struct OpsTransformNodeLocal(pub Entity, pub ETransformSRT);
impl OpsTransformNodeLocal {
    pub fn ops(node: Entity, val: ETransformSRT) -> Self {
        Self(node, val)
    }
}
pub type ActionListTransformNodeLocal = ActionList<OpsTransformNodeLocal>;

pub struct OpsTransformNodeLocalRotationQuaternion(pub Entity, pub f32, pub f32, pub f32, pub f32);
impl OpsTransformNodeLocalRotationQuaternion {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(node, x, y, z, w)
    }
}
pub type ActionListTransformNodeLocalRotationQuaternion = ActionList<OpsTransformNodeLocalRotationQuaternion>;

pub type BundleTransformNode = (
    Enable,
    TransformNodeDirty,
    CullingFlag,
    GlobalEnable,
    LocalPosition,
    LocalScaling,
    LocalRotationQuaternion,
    LocalEulerAngles,
    RecordLocalPosition,
    RecordLocalScaling,
    RecordLocalEulerAngles,
    RecordLocalRotationQuaternion,
    LocalRotation,
    LocalMatrix,
    GlobalMatrix,
    AbsoluteTransform,
);

#[derive(Default, Resource)]
pub struct TransformPerformance {
    pub wmcompute: XHashMap<Entity, u32>,
    pub all_wmcompute: u32,
}
