
use pi_scene_shell::prelude::*;
use pi_scene_math::Number;

use crate::cullings::prelude::*;

use super::environment::fog::*;

pub struct OpsSceneCreation(pub(crate) Entity, pub(crate) SceneBoundingPool, pub(crate) SceneColliderPool);
impl OpsSceneCreation {
    pub fn ops(scene: Entity, cullingmode: u8, param: [i32;9]) -> Self {
        let (pool, pool2) = match cullingmode {
            2 => {
                (
                    SceneBoundingPool::create_oct(
                        (param[0] as Number, param[1] as Number, param[2] as Number),
                        (param[3] as Number, param[4] as Number, param[5] as Number),
                        param[6] as usize,
                        param[7] as usize,
                        param[8] as usize
                    ),
                    SceneColliderPool::create_oct(
                        (param[0] as Number, param[1] as Number, param[2] as Number),
                        (param[3] as Number, param[4] as Number, param[5] as Number),
                        param[6] as usize,
                        param[7] as usize,
                        param[8] as usize
                    )
                )
            },
            1 => {
                (SceneBoundingPool::create_vec(), SceneColliderPool::create_vec())
            },
            _ => {
                (SceneBoundingPool::default(), SceneColliderPool::default())
            }
        };
        Self(scene, pool, pool2)
    }
}
pub type ActionListSceneCreate = ActionList<OpsSceneCreation>;

pub enum ESceneOps {
    AmbientColor(f32, f32, f32),
    AmbientIntensity(f32),
    Time(u64),
    FogColor(f32, f32, f32),
    FogParam(FogParam),
    AnimEnable(bool),
    BRDF(Atom, bool),
    OpaqueTexture(KeyRenderTarget),
    DepthTexture(KeyRenderTarget),
    EnvTexture(Option<Atom>, bool),
    ShadowMap(Option<KeyRenderTarget>),
}
pub struct OpsSceneOption(pub(crate) Entity, pub(crate)ESceneOps);
impl OpsSceneOption {
    pub fn ops(scene: Entity, val: ESceneOps) -> Self {
        Self(scene, val)
    }
    pub fn ambientcolor(scene: Entity, r: f32, g: f32, b: f32) -> Self {
        Self(scene, ESceneOps::AmbientColor(r, g, b))
    }
    pub fn ambientinstensity(scene: Entity, val: f32) -> Self {
        Self(scene, ESceneOps::AmbientIntensity(val))
    }
    pub fn time(scene: Entity, val: u64) -> Self {
        Self(scene, ESceneOps::Time(val))
    }
    pub fn fogcolor(scene: Entity, r: f32, g: f32, b: f32) -> Self {
        Self(scene, ESceneOps::FogColor(r, g, b))
    }
    pub fn fogparam(scene: Entity, val: FogParam) -> Self {
        Self(scene, ESceneOps::FogParam(val))
    }
    pub fn anime(scene: Entity, val: bool) -> Self {
        Self(scene, ESceneOps::AnimEnable(val))
    }
    pub fn brdf(scene: Entity, url: Atom, iscompress: bool) -> Self {
        Self(scene, ESceneOps::BRDF(url, iscompress))
    }
    pub fn opaquetexture(scene: Entity, val: KeyRenderTarget) -> Self {
        Self(scene, ESceneOps::OpaqueTexture(val))
    }
    pub fn depthtexture(scene: Entity, val: KeyRenderTarget) -> Self {
        Self(scene, ESceneOps::DepthTexture(val))
    }
    pub fn envtexture(scene: Entity, val: Option<Atom>, isfile: bool) -> Self {
        Self(scene, ESceneOps::EnvTexture(val, isfile))
    }
    pub fn shadowmap(scene: Entity, val: Option<KeyRenderTarget>) -> Self {
        Self(scene, ESceneOps::ShadowMap(val))
    }
}
pub type ActionListSceneOption = ActionList<OpsSceneOption>;
