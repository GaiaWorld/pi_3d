use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    particle::Particle,
};

use super::base::IParticleModifier;

#[derive(Clone)]
pub struct CustomDataForUV {
    pub u_scale: FloatInterpolation,
    pub v_scale: FloatInterpolation,
    pub u_offset: FloatInterpolation,
    pub v_offset: FloatInterpolation,
}

impl CustomDataForUV {
    pub fn new() -> Self {
        let mut u_scale = FloatInterpolation::new();
        let mut v_scale = FloatInterpolation::new();
        let u_offset = FloatInterpolation::new();
        let v_offset = FloatInterpolation::new();
        u_scale.constant0 = Some(1.);
        v_scale.constant0 = Some(1.);
        Self {
            u_scale,
            v_scale,
            u_offset,
            v_offset,
        }
    }
}

impl IParticleModifier for CustomDataForUV {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, _delta_seconds: f32) {
        particle.uv[0] = 1.0 / self.u_scale.interpolate(*amount, particle.base_random);
        particle.uv[1] = 1.0 / self.v_scale.interpolate(*amount, particle.base_random);
        particle.uv[2] = self.u_offset.interpolate(*amount, particle.base_random);
        particle.uv[3] = self.v_offset.interpolate(*amount, particle.base_random);
    }
}
