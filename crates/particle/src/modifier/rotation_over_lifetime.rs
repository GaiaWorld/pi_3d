use pi_scene_math::Vector3;

use crate::{interpolation::FloatInterpolation, particle::Particle};

use super::base::{RotationInterpolate, IParticleModifier};

#[derive(Clone)]
pub struct RotationOverLifetime {
    pub rotation_interpolate: RotationInterpolate,
}

impl RotationOverLifetime {


    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            rotation_interpolate: RotationInterpolate::new(x, y, z),
        }
    }
}

impl IParticleModifier for RotationOverLifetime{
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, delta_seconds: f32) {
        let mut local_result = Vector3::zeros();

        self.rotation_interpolate
            .compute(*amount, particle.base_random, &mut local_result);

        local_result = local_result * delta_seconds;

        particle.rotation = particle.rotation + local_result;
    }
}
