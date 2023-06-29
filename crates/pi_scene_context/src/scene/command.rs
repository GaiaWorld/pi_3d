
use pi_engine_shell::prelude::*;

use super::{
    ScenePassRenderCfg,
};

pub struct OpsSceneCreation(pub(crate) Entity, pub(crate) ScenePassRenderCfg);
impl OpsSceneCreation {
    pub fn ops(scene: Entity, passes_cfg: ScenePassRenderCfg) -> Self {
        Self(scene, passes_cfg)
    }
}
pub type ActionListSceneCreate = ActionList<OpsSceneCreation>;

pub struct OpsSceneDeltaTime(pub(crate) Entity, pub(crate) f32);
impl OpsSceneDeltaTime {
    pub fn ops(scene: Entity, val: f32) -> Self {
        Self(scene, val)
    }
}
pub type ActionListSceneDeltaTime = ActionList<OpsSceneDeltaTime>;

pub struct OpsSceneAnimationEnable(pub(crate) Entity, pub(crate) bool);
impl OpsSceneAnimationEnable {
    pub fn ops(scene: Entity, val: bool) -> Self {
        Self(scene, val)
    }
}
pub type ActionListSceneAnimationEnable = ActionList<OpsSceneAnimationEnable>;
