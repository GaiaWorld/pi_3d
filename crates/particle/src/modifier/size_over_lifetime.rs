use super::base::{IParticleModifier, ScalingInterpolate};
use crate::{interpolation::FloatInterpolation, multiply, particle::Particle};
use pi_scene_math::Vector3;

#[derive(Clone)]
pub struct SizeOverLifetime {
    pub scaling_interpolate: ScalingInterpolate,
}

impl SizeOverLifetime {
    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            scaling_interpolate: ScalingInterpolate::new(x, y, z),
        }
    }
}

impl IParticleModifier for SizeOverLifetime {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, _delta_seconds: f32) {
        let mut local_result = Vector3::zeros();

        self.scaling_interpolate
            .compute(*amount, particle.base_random, &mut local_result);

        particle.scaling = multiply(&local_result, &particle.scaling);
    }
}
