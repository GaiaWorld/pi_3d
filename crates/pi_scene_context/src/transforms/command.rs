
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_scene_math::*;

use crate::prelude::{Enable, GlobalEnable, CullingFlag};

use super::transform_node::*;

pub struct OpsTransformNode(pub Entity, pub Entity);
impl OpsTransformNode {
    pub fn ops(scene: Entity, node: Entity) -> Self {
        Self(scene, node)
    }
}
pub type ActionListTransformNodeCreate = ActionList<OpsTransformNode>;

pub struct OpsTransformNodeParent(pub Entity, pub Entity, pub u8);
impl OpsTransformNodeParent {
    pub fn ops(node: Entity, parent: Entity) -> Self {
        Self(node, parent, 0)
    }
}
pub type ActionListTransformNodeParent = ActionList<OpsTransformNodeParent>;

pub struct OpsTransformNodeLocalPosition(pub Entity, pub Vector3, pub u8);
impl OpsTransformNodeLocalPosition {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32) -> Self {
        Self(node, Vector3::new(x, y, z), 0)
    }
}
pub type ActionListTransformNodeLocalPosition = ActionList<OpsTransformNodeLocalPosition>;

pub struct OpsTransformNodeLocalRotationQuaternion(pub Entity, pub f32, pub f32, pub f32, pub f32, pub u8);
impl OpsTransformNodeLocalRotationQuaternion {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(node, x, y, z, w, 0)
    }
}
pub type ActionListTransformNodeLocalRotationQuaternion = ActionList<OpsTransformNodeLocalRotationQuaternion>;

pub struct OpsTransformNodeLocalEuler(pub Entity, pub Vector3, pub u8);
impl OpsTransformNodeLocalEuler {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32) -> Self {
        Self(node, Vector3::new(x, y, z), 0)
    }
}
pub type ActionListTransformNodeLocalEuler = ActionList<OpsTransformNodeLocalEuler>;

pub struct OpsTransformNodeLocalScaling(pub Entity, pub Vector3, pub u8);
impl OpsTransformNodeLocalScaling {
    pub fn ops(node: Entity, x: f32, y: f32, z: f32) -> Self {
        Self(node, Vector3::new(x, y, z), 0)
    }
}
pub type ActionListTransformNodeLocalScaling = ActionList<OpsTransformNodeLocalScaling>;

pub struct BundleTransformNode(
    Enable,
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
    WorldMatrix,
    GlobalTransform,
    CullingFlag,
);

#[derive(Default, Resource)]
pub struct TransformPerformance {
    pub wmcompute: XHashMap<Entity, u32>,
    pub all_wmcompute: u32,
}
