
use pi_engine_shell::prelude::*;
use pi_scene_math::Number;

use crate::cullings::prelude::*;

use super::environment::fog::*;

pub struct OpsSceneCreation(pub(crate) Entity, pub(crate) SceneBoundingPool);
impl OpsSceneCreation {
    pub fn ops(scene: Entity, cullingmode: u8, param: [i32;9]) -> Self {
        let pool = match cullingmode {
            2 => {
                SceneBoundingPool::create_oct(
                    (param[0] as Number, param[1] as Number, param[2] as Number),
                    (param[3] as Number, param[4] as Number, param[5] as Number),
                    param[6] as usize,
                    param[7] as usize,
                    param[8] as usize
                )
            },
            _ => {
                SceneBoundingPool::create_vec()
            }
        };
        Self(scene, pool)
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

pub struct OpsSceneBRDF(pub(crate) Entity, pub(crate) Atom, pub(crate) bool);
impl OpsSceneBRDF {
    pub fn ops(scene: Entity, url: Atom, compressed: bool) -> Self {
        Self(scene, url, compressed)
    }
}
pub type ActionListSceneBRDF = ActionList<OpsSceneBRDF>;

pub struct OpsSceneOpaqueTexture(pub(crate) Entity, pub(crate) KeyRenderTarget);
impl OpsSceneOpaqueTexture {
    pub fn ops(scene: Entity, key: KeyRenderTarget) -> Self {
        Self(scene, key)
    }
}
pub type ActionListSceneOpaqueTexture = ActionList<OpsSceneOpaqueTexture>;

pub struct OpsSceneDepthTexture(pub(crate) Entity, pub(crate) KeyRenderTarget);
impl OpsSceneDepthTexture {
    pub fn ops(scene: Entity, key: KeyRenderTarget) -> Self {
        Self(scene, key)
    }
}
pub type ActionListSceneDepthTexture = ActionList<OpsSceneDepthTexture>;

pub struct OpsSceneEnvTexture(pub(crate) Entity, pub(crate) Option<Atom>, pub(crate) bool);
impl OpsSceneEnvTexture {
    pub fn ops(scene: Entity, path: Option<Atom>, data_is_image: bool) -> Self {
        Self(scene, path, data_is_image)
    }
}
pub type ActionListSceneEnvTexture = ActionList<OpsSceneEnvTexture>;

pub struct OpsSceneShadowMap(pub(crate) Entity, pub(crate) Option<KeyRenderTarget>);
impl OpsSceneShadowMap {
    pub fn ops(scene: Entity, key: Option<KeyRenderTarget>) -> Self {
        Self(scene, key)
    }
}
pub type ActionListSceneShadowMap = ActionList<OpsSceneShadowMap>;
