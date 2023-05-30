use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    particle::Particle,
};

use super::base::IParticleModifier;

#[derive(Clone)]
pub struct CustomDataForUV {
    pub uScale: FloatInterpolation,
    pub vScale: FloatInterpolation,
    pub uOffset: FloatInterpolation,
    pub vOffset: FloatInterpolation,
}

impl CustomDataForUV {
    pub fn new() -> Self {
        let mut uScale = FloatInterpolation::new();
        let mut vScale = FloatInterpolation::new();
        let uOffset = FloatInterpolation::new();
        let vOffset = FloatInterpolation::new();
        uScale.constant0 = Some(1.);
        vScale.constant0 = Some(1.);
        Self {
            uScale,
            vScale,
            uOffset,
            vOffset,
        }
    }
}

impl IParticleModifier for CustomDataForUV {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        particle.uv[0] = 1.0 / self.uScale.interpolate(*amount, particle.base_random);
        particle.uv[1] = 1.0 / self.vScale.interpolate(*amount, particle.base_random);
        particle.uv[2] = self.uOffset.interpolate(*amount, particle.base_random);
        particle.uv[3] = self.vOffset.interpolate(*amount, particle.base_random);
    }
}
