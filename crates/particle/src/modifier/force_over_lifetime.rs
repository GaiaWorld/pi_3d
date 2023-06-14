use std::sync::Arc;

use crate::particle::Particle;

use super::base::{
    transform_vector_as_local_space, transform_vector_as_world_space, IParticleModifier,
    TranslationInterpolate, TEMP_VECTOR3_A,
};

#[derive(Clone)]
pub struct ForceOverLifetime {
    _is_local_space: bool,
    pub translation_interpolate: TranslationInterpolate,
    // transformForce: Box<dyn Fn(&Vector3, Matrix, &mut Vector3)>,
}

impl ForceOverLifetime {
    pub fn set_is_local_space(&mut self, value: bool) {
        if self._is_local_space != value {
            self._is_local_space = value;
            if value {
                self.translation_interpolate.transform_force =
                    Arc::new(transform_vector_as_local_space);
            } else {
                self.translation_interpolate.transform_force =
                    Arc::new(transform_vector_as_world_space);
            }
        }
    }
    pub fn get_is_local_space(&self) -> bool {
        return self._is_local_space;
    }

    pub fn new(translation_interpolate: TranslationInterpolate) -> Self {
        Self {
            _is_local_space: true,
            translation_interpolate,
        }
    }
}

impl IParticleModifier for ForceOverLifetime {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, delta_seconds: f32) {
        let mut local_force = TEMP_VECTOR3_A;

        self.translation_interpolate.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut local_force,
        );

        local_force = local_force * delta_seconds;

        particle.direction = particle.direction + local_force;
    }
}
