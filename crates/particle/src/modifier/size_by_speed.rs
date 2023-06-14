use nalgebra::Vector3;

use crate::{interpolation::FloatInterpolation, multiply, particle::Particle};

use super::base::{IParticleModifier, ScalingInterpolate};

#[derive(Clone)]
pub struct SizeBySpeed {
    pub _range_x: f32,
    pub _range_y: f32,
    pub _range_size: f32,
    pub scaling_interpolate: ScalingInterpolate,
}

impl SizeBySpeed {
    pub fn set_range_x(&mut self, value: f32) {
        self._range_x = value;
        self._range_size = self._range_y - self._range_x;
    }
    pub fn get_range_x(&mut self) -> f32 {
        return self._range_x;
    }
    pub fn set_range_y(&mut self, value: f32) {
        self._range_y = value;
        self._range_size = self._range_y - self._range_x;
    }
    pub fn get_range_y(&mut self) -> f32 {
        return self._range_y;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            _range_x: 0.,
            _range_y: 1.,
            _range_size: 1.,
            scaling_interpolate: ScalingInterpolate::new(x, y, z),
        }
    }
}

impl IParticleModifier for SizeBySpeed {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, _delta_seconds: f32) {
        let mut local_result = Vector3::zeros();

        *amount =
            1.0f32.min(0.0f32.max((particle.direction_length - self._range_x) / self._range_size));
        self.scaling_interpolate
            .compute(*amount, particle.base_random, &mut local_result);

        particle.scaling = multiply(&particle.scaling, &local_result);
    }
}
