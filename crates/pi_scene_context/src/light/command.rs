
use pi_engine_shell::prelude::*;
use pi_scene_math::Vector3;

use super::base::{Light, LightingMode};

pub struct OpsLightCreate(pub(crate) Entity, pub(crate) Entity);
impl OpsLightCreate {
    pub fn ops(scene: Entity, light: Entity) -> Self {
        OpsLightCreate(scene, light)
    }
}

pub type ActionListLightCreate = ActionList<OpsLightCreate>;

pub type ActionListLightParam = ActionList<ELightModifyCommand>;

pub enum ELightModifyCommand {
    LightType(Entity, Light),
    LightingType(Entity, LightingMode),
    ShadowMinz(Entity, f32),
    ShadowMaxz(Entity, f32),
    ShadowFrustumSize(Entity, f32),
    Directional(Entity, Vector3),
    Bias(Entity, f32),
    NormalBias(Entity, f32),
    DepthScale(Entity, f32),
    AtlasSize(Entity, u32),
    ShadowEnable(Entity, bool),
}
