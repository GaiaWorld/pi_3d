use rand::Rng;
use crate::{particle::Particle, iparticle_system_config::FourGradientInfo};
use super::base::Color4Interpolate;

pub struct StartColor {
    color4_interpolate: Color4Interpolate,
}

impl StartColor {
    pub fn modify(&mut self, particle: &mut Particle, amount: f32, _delta_seconds: f32) {
        let mut rng = rand::thread_rng();
        self.color4_interpolate
            .compute(amount, &mut particle.start_color, rng.gen());
        particle.color = particle.start_color;
    }

    pub fn new(color4_interpolate: Color4Interpolate) -> Self {
        Self { color4_interpolate }
    }

    pub fn format(config: &FourGradientInfo, target: &mut StartColor) {
        Color4Interpolate::format(config, &mut target.color4_interpolate);
    }
}
