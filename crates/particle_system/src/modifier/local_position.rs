use pi_scene_math::{Quaternion, Vector3};

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    iparticle_system_config::EInterpolationCurveMode,
    particle::Particle, tools::{OrbitVelocity, BaseRandom},
};

use super::base::{
    IParticleModifier, TranslationInterpolate, TEMP_VECTOR3_A, TEMP_VECTOR3_B, TEMP_VECTOR3_C,
    TEMP_VECTOR3_D,
};

pub struct OrbitVelocityModifier {
    /// 绕轨道中心的角速度 2*PI 即 1s 绕一周
    pub orbital_rotate_speed: TranslationInterpolate,
    /// 在轨道中心
    pub orbital_offset: TranslationInterpolate,
    /// 在轨道中心指向当前位置的方向上的速度量
    pub radial: FloatInterpolation,
}
impl Default for OrbitVelocityModifier {
    fn default() -> Self {
        Self {
            orbital_rotate_speed: TranslationInterpolate::default(),
            orbital_offset: TranslationInterpolate::default(),
            radial: FloatInterpolation::new(0.),
        }
    }
}

impl OrbitVelocityModifier {
    pub fn modify(&self, item: &mut OrbitVelocity, amount: f32, randoms: &BaseRandom) {
        self.orbital_rotate_speed.compute(amount, randoms, &mut item.orbit);
        self.orbital_offset.compute(amount, randoms, &mut item.offset);
        item.radial = self.radial.interpolate(amount, randoms.x);
    }
}
