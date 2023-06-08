use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    particle::Particle,
};

use super::base::IParticleModifier;

#[derive(Clone)]
pub struct LimitVelocityOverLifetime {
    pub interpolation: FloatInterpolation,
    pub dampen: f32,
}

impl LimitVelocityOverLifetime {
    pub fn new(interpolation: FloatInterpolation) -> Self {
        Self {
            interpolation,
            dampen: 0.0,
        }
    }
}

impl IParticleModifier for LimitVelocityOverLifetime {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let localResult = self.interpolation.interpolate(*amount, particle.base_random);

        let currLength = particle.direction_length;
        if (currLength > localResult) {
            particle.direction = particle.direction
                * (1.0 - self.dampen * (currLength - localResult) / currLength * (0.66));
        }
    }
}
