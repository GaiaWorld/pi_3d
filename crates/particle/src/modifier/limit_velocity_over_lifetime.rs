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
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, _delta_seconds: f32) {
        let local_result = self.interpolation.interpolate(*amount, particle.base_random);

        let curr_length = particle.direction_length;
        if curr_length > local_result {
            // println!("LimitVelocityOverLifetime particle.direction: {:?}",  particle.direction);
            particle.direction = particle.direction
                * (1.0 - self.dampen * (curr_length - local_result) / curr_length * (0.66));
        }
    }
}
