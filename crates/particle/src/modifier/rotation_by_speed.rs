use pi_scene_math::Vector3;

use crate::{interpolation::FloatInterpolation, particle::Particle};

use super::base::{RotationInterpolate, IParticleModifier};

#[derive(Clone)]
pub struct RotationBySpeed {
    _rangeX: f32,
    _rangeY: f32,
    _rangeSize: f32,
    pub rotationInterpolate: RotationInterpolate,
}

impl RotationBySpeed {
    pub fn set_rangeX(&mut self, value: f32) {
        self._rangeX = value;
        self._rangeSize = self._rangeY - self._rangeX;
    }
    pub fn get_rangeX(&mut self) -> f32 {
        return self._rangeX;
    }
    pub fn set_rangeY(&mut self, value: f32) {
        self._rangeY = value;
        self._rangeSize = self._rangeY - self._rangeX;
    }
    pub fn get_rangeY(&mut self) -> f32 {
        return self._rangeY;
    }
    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            _rangeX: 0.,
            _rangeY: 1.,
            _rangeSize: 1.,
            rotationInterpolate: RotationInterpolate::new(x, y, z),
        }
    }
}

impl IParticleModifier for RotationBySpeed{
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let mut localResult = Vector3::zeros();

        *amount =
            1.0f32.min(0.0f32.max((particle.direction_length - self._rangeX) / self._rangeSize));
        self.rotationInterpolate
            .compute(*amount, particle.base_random, &mut localResult);

        localResult = localResult * deltaSeconds;

        particle.rotation = particle.rotation + localResult;
    }
}
