use crate::particle::Particle;

use super::base::{Color4Interpolate, TEMP_COLOR4_A, IParticleModifier};

#[derive(Clone)]
pub struct ColorBySpeed {
    _range_x: f32,
    _range_y: f32,
    _range_size: f32,
    pub color4_interpolate: Color4Interpolate,
}

impl ColorBySpeed {
    pub fn set_range_x(&mut self, value: f32) {
        self._range_x = value;
        self._range_size = self._range_y - self._range_x;
    }

    pub fn get_range_x(&self) -> f32 {
        return self._range_x;
    }

    pub fn set_range_y(&mut self, value: f32) {
        self._range_y = value;
        self._range_size = self._range_y - self._range_x;
    }

    pub fn get_range_y(&self) -> f32 {
        return self._range_y;
    }

    pub fn new(color4_interpolate: Color4Interpolate) -> Self {
        Self {
            _range_x: 0.,
            _range_y: 1.,
            _range_size: 1.,
            color4_interpolate,
        }
    }
}

impl IParticleModifier for ColorBySpeed{
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, _delta_seconds: f32) {
        let mut local_result = TEMP_COLOR4_A;
        *amount =
            1.0f32.min(0.0f32.max((particle.direction_length - self._range_x) / self._range_size));
        self.color4_interpolate.compute(
            *amount,
            &mut local_result,
            particle.color_over_lifetime_amount,
        );

        particle.color = particle.color.component_mul(&local_result);
    }
}
