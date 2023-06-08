use std::ops::Mul;

use crate::particle::Particle;

use super::base::{Color4Interpolate, TempColor4A, IParticleModifier};

#[derive(Clone)]
pub struct ColorBySpeed {
    _rangeX: f32,
    _rangeY: f32,
    _rangeSize: f32,
    pub color4Interpolate: Color4Interpolate,
}

impl ColorBySpeed {
    pub fn set_rangeX(&mut self, value: f32) {
        self._rangeX = value;
        self._rangeSize = self._rangeY - self._rangeX;
    }

    pub fn get_rangeX(&self) -> f32 {
        return self._rangeX;
    }

    pub fn set_rangeY(&mut self, value: f32) {
        self._rangeY = value;
        self._rangeSize = self._rangeY - self._rangeX;
    }

    pub fn get_rangeY(&self) -> f32 {
        return self._rangeY;
    }

    pub fn new(color4Interpolate: Color4Interpolate) -> Self {
        Self {
            _rangeX: 0.,
            _rangeY: 1.,
            _rangeSize: 1.,
            color4Interpolate,
        }
    }
}

impl IParticleModifier for ColorBySpeed{
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let mut localResult = TempColor4A;
        *amount =
            1.0f32.min(0.0f32.max((particle.direction_length - self._rangeX) / self._rangeSize));
        self.color4Interpolate.compute(
            *amount,
            &mut localResult,
            particle.color_over_lifetime_amount,
        );

        particle.color = particle.color.component_mul(&localResult);
    }
}
