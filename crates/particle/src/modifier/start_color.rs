// import { IParticle } from "../particle_base";
// import { Color4Interpolate, IParticleModifier, ScalingInterpolate, TempVector3A, TranslationInterpolate } from "./base";

// export class StartColor extends Color4Interpolate implements IParticleModifier {
//     modify(particle: IParticle, amount: number, deltaSeconds: number) {
//         this.compute(amount, particle.startColor, Math.random());
//         particle.color.copyFrom(particle.startColor);
//     }
// }

use rand::Rng;

use crate::{particle::Particle, iparticle_system_config::FourGradientInfo};

use super::base::Color4Interpolate;

pub struct StartColor {
    color4Interpolate: Color4Interpolate,
}

impl StartColor {
    pub fn modify(&mut self, particle: &mut Particle, amount: f32, deltaSeconds: f32) {
        let mut rng = rand::thread_rng();
        self.color4Interpolate
            .compute(amount, &mut particle.start_color, rng.gen());
        particle.color = particle.start_color;
    }

    pub fn new(color4Interpolate: Color4Interpolate) -> Self {
        Self { color4Interpolate }
    }

    pub fn format(config: &FourGradientInfo, target: &mut StartColor) {
        Color4Interpolate::format(config, &mut target.color4Interpolate);
    }
}
