use pi_scene_math::Vector3;

use crate::{interpolation::FloatInterpolation, particle::Particle};

use super::base::{RotationInterpolate, IParticleModifier};

#[derive(Clone)]
pub struct RotationOverLifetime {
    pub rotationInterpolate: RotationInterpolate,
}

impl RotationOverLifetime {


    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            rotationInterpolate: RotationInterpolate::new(x, y, z),
        }
    }
}

impl IParticleModifier for RotationOverLifetime{
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let mut localResult = Vector3::zeros();

        self.rotationInterpolate
            .compute(*amount, particle.base_random, &mut localResult);

        localResult = localResult * deltaSeconds;

        particle.rotation = particle.rotation + localResult;
    }
}
