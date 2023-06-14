use pi_scene_math::Vector3;

use crate::{interpolation::FloatInterpolation, particle::Particle};

use super::base::{RotationInterpolate, IParticleModifier};

#[derive(Clone)]
pub struct RotationBySpeed {
    _range_x: f32,
    _range_y: f32,
    _range_size: f32,
    pub rotation_interpolate: RotationInterpolate,
}

impl RotationBySpeed {
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
            rotation_interpolate: RotationInterpolate::new(x, y, z),
        }
    }
}

impl IParticleModifier for RotationBySpeed{
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, delta_seconds: f32) {
        let mut local_result = Vector3::zeros();

        *amount =
            1.0f32.min(0.0f32.max((particle.direction_length - self._range_x) / self._range_size));
        self.rotation_interpolate
            .compute(*amount, particle.base_random, &mut local_result);

        local_result = local_result * delta_seconds;

        particle.rotation = particle.rotation + local_result;
    }
}
