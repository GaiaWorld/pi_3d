use std::sync::Arc;

use pi_scene_math::{Matrix, Vector3};

use crate::particle::Particle;

use super::base::{
    transformVectorAsLocalSpace, transformVectorAsWorldSpace, Color4Interpolate, TempVector3A,
    TranslationInterpolate, IParticleModifier,
};

#[derive(Clone)]
pub struct ForceOverLifetime {
    _isLocalSpace: bool,
    pub translationInterpolate: TranslationInterpolate,
    // transformForce: Box<dyn Fn(&Vector3, Matrix, &mut Vector3)>,
}

impl ForceOverLifetime {
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

    pub fn new(translationInterpolate: TranslationInterpolate) -> Self {
        Self {
            _isLocalSpace: true,
            translationInterpolate,
        }
    }
}

impl IParticleModifier for ForceOverLifetime{
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let mut localForce = TempVector3A;

        self.translationInterpolate.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut localForce,
        );

        localForce = localForce * deltaSeconds;

        particle.direction = particle.direction + localForce;
    }
}