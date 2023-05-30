use std::sync::Arc;

use pi_scene_math::{Matrix, Vector3};

use crate::{interpolation::FloatInterpolation, particle::Particle, iparticle_system_config::ParamInfo};

use super::base::{
    transformVectorAsLocalSpace, transformVectorAsWorldSpace, IParticleModifier,
    TranslationInterpolate, Vector3Interpolate,
};

#[derive(Clone)]
pub struct VelocityOverLifetime {
    _isLocalSpace: bool,
    pub translationInterpolate: TranslationInterpolate,
}

impl VelocityOverLifetime {
    pub fn set_isLocalSpace(&mut self, value: bool) {
        if (self._isLocalSpace != value) {
            self._isLocalSpace = value;
            if (value) {
                self.translationInterpolate.transformForce = Arc::new(transformVectorAsLocalSpace);
            } else {
                self.translationInterpolate.transformForce = Arc::new(transformVectorAsWorldSpace);
            }
        }
    }
    pub fn get_isLocalSpace(&self) -> bool {
        return self._isLocalSpace;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        let mut a = TranslationInterpolate::new(x, y, z);
        a._isAxis = true;

        Self {
            _isLocalSpace: true,
            translationInterpolate: a,
        }
    }

    pub fn modify(&self, particle: &mut Particle, amount: f32, deltaSeconds: f32) {
        let mut localResult = Vector3::zeros();

        self.translationInterpolate.compute(
            amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut localResult,
        );

        let x = localResult[0];
        let y = localResult[1];
        let z = localResult[2];
        localResult = localResult - (particle.velocity);

        particle.velocity = Vector3::new(x, y, z);

        particle.direction += localResult;
    }

    pub fn format(config: &ParamInfo, target: &mut TranslationInterpolate){
        TranslationInterpolate::format(config, target)
    }
}

impl IParticleModifier for VelocityOverLifetime {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let mut localResult = Vector3::zeros();

        self.translationInterpolate.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut localResult,
        );

        let x = localResult[0];
        let y = localResult[1];
        let z = localResult[2];
        localResult = localResult - (particle.velocity);

        particle.velocity = Vector3::new(x, y, z);

        particle.direction += localResult;
    }
}
