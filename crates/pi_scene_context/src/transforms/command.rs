
use pi_animation::animation::AnimationInfo;
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_scene_math::*;
use pi_slotmap::DefaultKey;

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


// pub enum OpsTransformNodeLocalRotation {
//     Euler(Entity, Number, Number, Number),
//     Quaternion(Entity, Number, Number, Number, Number),
// }
// pub type ActionListTransformNodeLocalRotation = ActionList<OpsTransformNodeLocalRotation>;

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
    GlobalTransform,
);

#[derive(Default, Resource)]
pub struct TransformPerformance {
    pub wmcompute: XHashMap<Entity, u32>,
    pub all_wmcompute: u32,
}
