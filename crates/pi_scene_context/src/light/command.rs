
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, Number};

use super::base::*;

#[derive(Debug, Clone, Copy)]
pub enum ELightType {
    Direct,
    Spot,
    Point,
    Hemispheric,
}

pub struct OpsLightCreate(pub(crate) Entity, pub(crate) Entity, pub ELightType);
impl OpsLightCreate {
    pub fn ops(scene: Entity, light: Entity, ltype: ELightType) -> Self {
        OpsLightCreate(scene, light, ltype)
    }
}
pub type ActionListLightCreate = ActionList<OpsLightCreate>;

// pub struct OpsLightColor(pub(crate) Entity, pub(crate) Vector3);
// impl OpsLightColor {
//     pub fn ops(light: Entity, r: Number, g: Number, b: Number) -> Self {
//         Self(light, Vector3::new(r, g, b))
//     }
// }
// pub type ActionListLightColor = ActionList<OpsLightColor>;

pub enum ELightModify {
    Color(Number, Number, Number),
    LightingType(LightingMode),
    Directional(Vector3),
    SpotAngle(f32, f32),
    Strength(f32),
    Radius(f32),
}
pub struct OpsLightParam(pub(crate) Entity, pub(crate) ELightModify);
impl OpsLightParam {
    pub fn ops(light: Entity, param: ELightModify) -> Self {
        Self(light, param)
    }
}
pub type ActionListLightParam = ActionList<OpsLightParam>;


// pub struct OpsSpotLightAngle(pub(crate) Entity, pub(crate) f32, pub(crate) f32);
// impl OpsSpotLightAngle {
//     pub fn ops(light: Entity, outangle: f32, inangle: f32) -> Self {
//         Self(light, outangle, inangle)
//     }
// }
// pub type ActionListSpotLightAngle = ActionList<OpsSpotLightAngle>;

// pub struct OpsLightStrength(pub(crate) Entity, pub(crate) f32);
// impl OpsLightStrength {
//     pub fn ops(light: Entity, strength: f32) -> Self {
//         Self(light, strength)
//     }
// }
// pub type ActionListLightStrength = ActionList<OpsLightStrength>;

// pub struct OpsLightRadius(pub(crate) Entity, pub(crate) f32);
// impl OpsLightRadius {
//     pub fn ops(light: Entity, radius: f32) -> Self {
//         Self(light, radius)
//     }
// }
// pub type ActionListLightRadius = ActionList<OpsLightRadius>;