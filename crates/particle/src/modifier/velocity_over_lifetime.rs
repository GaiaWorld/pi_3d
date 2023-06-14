use std::sync::Arc;

use pi_scene_math::Vector3;

use crate::{interpolation::FloatInterpolation, particle::Particle, iparticle_system_config::ParamInfo};

use super::base::{
    transform_vector_as_local_space, transform_vector_as_world_space, IParticleModifier,
    TranslationInterpolate,
};

#[derive(Clone)]
pub struct VelocityOverLifetime {
    _is_local_space: bool,
    pub translation_interpolate: TranslationInterpolate,
}

impl VelocityOverLifetime {
    pub fn set_is_local_space(&mut self, value: bool) {
        if self._is_local_space != value {
            self._is_local_space = value;
            if value {
                self.translation_interpolate.transform_force = Arc::new(transform_vector_as_local_space);
            } else {
                self.translation_interpolate.transform_force = Arc::new(transform_vector_as_world_space);
            }
        }
    }
    pub fn get_is_local_space(&self) -> bool {
        return self._is_local_space;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        let mut a = TranslationInterpolate::new(x, y, z);
        a._is_axis = true;

        Self {
            _is_local_space: true,
            translation_interpolate: a,
        }
    }

    pub fn modify(&self, particle: &mut Particle, amount: f32, _delta_seconds: f32) {
        let mut local_result = Vector3::zeros();

        self.translation_interpolate.compute(
            amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut local_result,
        );

        let x = local_result[0];
        let y = local_result[1];
        let z = local_result[2];
        local_result = local_result - (particle.velocity);

        particle.velocity = Vector3::new(x, y, z);
       
        particle.direction += local_result;
    }

    pub fn format(config: &ParamInfo, target: &mut TranslationInterpolate){
        TranslationInterpolate::format(config, target)
    }
}

impl IParticleModifier for VelocityOverLifetime {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, _delta_seconds: f32) {
        let mut local_result = Vector3::zeros();

        self.translation_interpolate.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut local_result,
        );

        let x = local_result[0];
        let y = local_result[1];
        let z = local_result[2];
        local_result = local_result - (particle.velocity);

        particle.velocity = Vector3::new(x, y, z);

        particle.direction += local_result;
    }
}
