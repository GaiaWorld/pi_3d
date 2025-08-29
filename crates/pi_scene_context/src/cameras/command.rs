
use pi_scene_shell::prelude::*;
use pi_scene_math::{Number, Vector3};
use serde::{Deserialize, Serialize};

use super::camera::*;

pub struct OpsCameraCreation(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity);
impl OpsCameraCreation {
    pub fn ops(scene: Entity, entity: Entity, name: Entity) -> Self {
        Self(scene, entity, name)
    }
}
pub type ActionListCameraCreate = ActionList<OpsCameraCreation>;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ECameraModify {
    FreeMode(EFreeCameraMode),
    Active(bool),
    FixMode(EFixedMode),
    Fov(Number),
    OrthSize(Number),
    Aspect(Number),
    NearFar(Number, Number),
}
pub struct OpsCameraModify(pub(crate) Entity, pub(crate) ECameraModify);
impl OpsCameraModify {
    pub fn ops(camera: Entity, val: ECameraModify) -> Self {
        Self(camera, val)
    }
}
pub type ActionListCameraModify = ActionList<OpsCameraModify>;

pub struct OpsCameraTarget(pub(crate) Entity, pub(crate) Vector3);
impl OpsCameraTarget {
    pub fn ops(camera: Entity, x: Number, y: Number, z: Number) -> Self {
        Self(camera, Vector3::new(x, y, z))
    }
}
pub type ActionListCameraTarget = ActionList<OpsCameraTarget>;
