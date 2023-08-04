
use pi_scene_math::{Vector3, Number};

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    particle::Particle, tools::{GravityFactor, BaseRandom},
};

use super::base::{TEMP_VECTOR3_A, IParticleModifier};

pub struct Gravity {
    pub interpolation: FloatInterpolation,
}
impl Default for Gravity {
    fn default() -> Self {
        Self {
            interpolation: FloatInterpolation::new(0.),
        }
    }
}

impl Gravity {
    pub fn modify(&self, item: &mut Number, amount: f32, delta_seconds: f32, randoms: &BaseRandom) {
        *item = self.interpolation.interpolate(amount, randoms.x);
    }
}