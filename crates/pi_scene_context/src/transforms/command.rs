
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_scene_math::{Vector3};

use crate::{flags::UniqueName, prelude::{Enable, GlobalEnable}};

use super::{transform_node::*};

pub struct OpsTransformNode(pub Entity, pub Entity, pub String);
impl OpsTransformNode {
    pub fn ops(scene: Entity, node: Entity, name: String) -> Self {
        Self(scene, node, name)
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
    UniqueName,
    Enable,
    GlobalEnable,
    LocalPosition,
    LocalScaling,
    LocalRotation,
    LocalRotationQuaternion,
    LocalEulerAngles,
    LocalMatrix,
    WorldMatrix,
    GlobalTransform,
);

#[derive(Default, Resource)]
pub struct TransformRecord {
    pub wmcompute: XHashMap<Entity, u32>,
    pub all_wmcompute: u32,
}
