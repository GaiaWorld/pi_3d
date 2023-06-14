use pi_scene_math::Vector3;

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    particle::Particle,
};

use super::base::TEMP_VECTOR3_A;

pub struct Gravity {
    defualt_force: Vector3,
    pub interpolation: FloatInterpolation,
}

impl Gravity {
    pub fn modify(&mut self, particle: &mut Particle, amount: f32, delta_seconds: f32) {
        let factor = self.interpolation.interpolate(amount, particle.base_random);

        let mut _local_force = TEMP_VECTOR3_A;
        _local_force = self.defualt_force * factor;
        _local_force = particle
            .start_world_matrix_invert
            .transform_vector(&_local_force);

        _local_force = _local_force * delta_seconds;
        
        particle.direction = particle.direction + (_local_force);
    }

    pub fn new(interpolation: FloatInterpolation) ->Self{
        Self {
            interpolation,
            defualt_force: Vector3::new(0., -9.8, 0.),
        }
    }
}
