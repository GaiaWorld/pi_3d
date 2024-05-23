

use pi_scene_shell::prelude::*;


use super::BindSceneEffect;

#[derive(Clone, Copy)]
pub struct FogLinearParam {
    pub start: f32,
    pub end: f32,
}

#[derive(Clone, Copy)]
pub struct FogExpParam {
    pub density_fallof: f32,
}
impl Default for FogExpParam {
    fn default() -> Self {
        Self { density_fallof: 0.1 }
    }
}

#[derive(Clone, Copy)]
pub struct FogExp2Param {
    pub density_fallof: f32,
}
impl Default for FogExp2Param {
    fn default() -> Self {
        Self { density_fallof: 0.1 }
    }
}

#[derive(Clone, Copy)]
pub struct FogAltitudeBaseParam {
    pub h_while_max_density: f32,
    pub density_fallof: f32,
    pub density: f32,
}
impl Default for FogAltitudeBaseParam {
    fn default() -> Self {
        Self { h_while_max_density: 0., density_fallof: 0.2, density: 1. }
    }
}

#[derive(Clone, Copy)]
pub enum FogParam {
    None,
    Linear(FogLinearParam),
    Exp(FogExpParam),
    Exp2(FogExp2Param),
    AltitudeBase(FogAltitudeBaseParam),
}
impl FogParam {
    pub const NONE: u8 = 0;
    pub const LINEAR: u8 = 3;
    pub const EXP: u8 = 1;
    pub const EXP2: u8 = 2;
    pub const ALTITUDE_BASE: u8 = 4;
    pub fn as_array(&self) -> [f32;4] {
        match self {
            FogParam::None => [FogParam::NONE as f32, 0., 0., 0.],
            FogParam::Exp(val) => [FogParam::EXP as f32, 0., 0., val.density_fallof],
            FogParam::Exp2(val) => [FogParam::EXP2 as f32, 0., 0., val.density_fallof],
            FogParam::Linear(val) => [FogParam::LINEAR as f32, val.start, val.end, 0.],
            FogParam::AltitudeBase(val) => [FogParam::ALTITUDE_BASE as f32, val.h_while_max_density, val.density, val.density_fallof],
        }
    }
}

// 
#[derive(Component)]
pub struct SceneFog {
    pub param: FogParam,
    pub r: Number,
    pub g: Number,
    pub b: Number,
}

pub fn update_scenefog_uniform(fog: &SceneFog, bind: &mut BindSceneEffect) {
    bind.0.data().write_data(ShaderBindSceneAboutEffect::OFFSET_FOG_INFO as usize, bytemuck::cast_slice(&[fog.r, fog.g, fog.b]));
    bind.0.data().write_data(ShaderBindSceneAboutEffect::OFFSET_FOG_PARAM as usize, bytemuck::cast_slice(&fog.param.as_array()));
}
