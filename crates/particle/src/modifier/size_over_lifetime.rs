use super::base::{IParticleModifier, ScalingInterpolate};
use crate::{interpolation::FloatInterpolation, particle::Particle};
use pi_scene_math::Vector3;

#[derive(Clone)]
pub struct SizeOverLifetime {
    pub scalingInterpolate: ScalingInterpolate,
}

impl SizeOverLifetime {
    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            scalingInterpolate: ScalingInterpolate::new(x, y, z),
        }
    }
}

impl IParticleModifier for SizeOverLifetime {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let mut localResult = Vector3::zeros();

        self.scalingInterpolate
            .compute(*amount, particle.base_random, &mut localResult);

        particle.scaling = localResult.cross(&particle.scaling);
    }
}
