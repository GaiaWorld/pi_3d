use crate::{interpolation::FloatInterpolation, particle::Particle, iparticle_system_config::ParamInfo};

use super::base::RotationInterpolate;

pub struct StartRotation {
    pub rotation_interpolate: RotationInterpolate,
}

impl StartRotation {
    pub fn modify(&mut self, particle: &mut Particle, amount: f32, _delta_seconds: f32) {
        self.rotation_interpolate
            .compute(amount, particle.base_random, &mut particle.rotation);
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            rotation_interpolate: RotationInterpolate::new(x, y, z),
        }
    }

    pub fn format(config: &ParamInfo,  target: &mut StartRotation) {
        RotationInterpolate::format(config, &mut target.rotation_interpolate)
    }
}
