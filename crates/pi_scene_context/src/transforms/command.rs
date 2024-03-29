
use pi_scene_shell::prelude::*;

use crate::flags::*;

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

pub struct OpsTransformNodeLocalPosition(pub Entity, pub Vector3);
impl OpsTransformNodeLocalPosition {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32) -> Self {
        Self(node, Vector3::new(x, y, z))
    }
}
pub type ActionListTransformNodeLocalPosition = ActionList<OpsTransformNodeLocalPosition>;

pub struct OpsTransformNodeLocalRotationQuaternion(pub Entity, pub f32, pub f32, pub f32, pub f32);
impl OpsTransformNodeLocalRotationQuaternion {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(node, x, y, z, w)
    }
}
pub type ActionListTransformNodeLocalRotationQuaternion = ActionList<OpsTransformNodeLocalRotationQuaternion>;

pub struct OpsTransformNodeLocalEuler(pub Entity, pub Vector3);
impl OpsTransformNodeLocalEuler {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32) -> Self {
        Self(node, Vector3::new(x, y, z))
    }
}
pub type ActionListTransformNodeLocalEuler = ActionList<OpsTransformNodeLocalEuler>;

pub struct OpsTransformNodeLocalScaling(pub Entity, pub Vector3);
impl OpsTransformNodeLocalScaling {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32) -> Self {
        Self(node, Vector3::new(x, y, z))
    }
}
pub type ActionListTransformNodeLocalScaling = ActionList<OpsTransformNodeLocalScaling>;

pub struct BundleTransformNode(
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
    RecordLocalRotationQuaternion,
    RecordLocalEulerAngles,
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
