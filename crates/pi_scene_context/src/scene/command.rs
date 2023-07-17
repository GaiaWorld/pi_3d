
use pi_engine_shell::prelude::*;

use crate::prelude::FogParam;

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

pub struct OpsSceneTime(pub(crate) Entity, pub(crate) u64);
impl OpsSceneTime {
    pub fn ops(scene: Entity, val: u64) -> Self {
        Self(scene, val)
    }
}
pub type ActionListSceneTime = ActionList<OpsSceneTime>;

pub struct OpsSceneAmbientColor(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) u8);
impl OpsSceneAmbientColor {
    pub fn ops(scene: Entity, r: f32, g: f32, b: f32) -> Self {
        Self(scene, r, g, b, 0)
    }
}
pub type ActionListSceneAmbientColor = ActionList<OpsSceneAmbientColor>;

pub struct OpsSceneAmbientIntensity(pub(crate) Entity, pub(crate) f32, pub(crate) u8);
impl OpsSceneAmbientIntensity {
    pub fn ops(scene: Entity, intensity: f32) -> Self {
        Self(scene, intensity, 0)
    }
}
pub type ActionListSceneAmbientIntensity = ActionList<OpsSceneAmbientIntensity>;

pub struct OpsSceneFogColor(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) u8);
impl OpsSceneFogColor {
    pub fn ops(scene: Entity, r: f32, g: f32, b: f32) -> Self {
        Self(scene, r, g, b, 0)
    }
}
pub type ActionListSceneFogColor = ActionList<OpsSceneFogColor>;

pub struct OpsSceneFogParam(pub(crate) Entity, pub(crate) FogParam, pub(crate) u8);
impl OpsSceneFogParam {
    pub fn ops(scene: Entity, param: FogParam) -> Self {
        Self(scene, param, 0)
    }
}
pub type ActionListSceneFogParam = ActionList<OpsSceneFogParam>;

pub struct OpsSceneAnimationEnable(pub(crate) Entity, pub(crate) bool, pub(crate) u8);
impl OpsSceneAnimationEnable {
    pub fn ops(scene: Entity, val: bool) -> Self {
        Self(scene, val, 0)
    }
}
pub type ActionListSceneAnimationEnable = ActionList<OpsSceneAnimationEnable>;
