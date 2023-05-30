use pi_scene_math::Vector3;

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    particle::Particle,
};

use super::base::TempVector3A;

pub struct Gravity {
    defualtForce: Vector3,
    pub interpolation: FloatInterpolation,
}

impl Gravity {
    pub fn modify(&mut self, particle: &mut Particle, amount: f32, deltaSeconds: f32) {
        let factor = self.interpolation.interpolate(amount, particle.base_random);

        let mut localForce = TempVector3A;
        localForce = self.defualtForce * factor;
        localForce = particle
            .start_world_matrix_invert
            .transform_vector(&localForce);

        localForce = localForce * deltaSeconds;

        particle.direction = particle.direction + (localForce);
    }

    pub fn new(interpolation: FloatInterpolation) ->Self{
        Self {
            interpolation,
            defualtForce: Vector3::new(0., -9.8, 0.),
        }
    }
}
