// import { IParticle } from "../particle_base";
// import { IParticleModifier, ScalingInterpolate, TempVector3A, TranslationInterpolate } from "./base";

// export class StartSize extends ScalingInterpolate implements IParticleModifier {
//     modify(particle: IParticle, amount: number, deltaSeconds: number) {
//         this.compute(amount, particle.baseRandom, particle.startScaling);
//         particle.scaling.copyFrom(particle.startScaling);
//     }
// }

use super::base::ScalingInterpolate;
use crate::{particle::Particle, iparticle_system_config::ParamInfo};

pub struct StartSize {
    pub scaling_interpolate: ScalingInterpolate,
}

impl Default for StartSize {
    fn default() -> Self {
        Self {
            scaling_interpolate: ScalingInterpolate::default(),
        }
    }
}

impl StartSize {
    pub fn modify(&self, particle: &mut Particle, amount: f32, _delta_seconds: f32) {
        self.scaling_interpolate
            .compute(amount, particle.base_random, &mut particle.start_scaling);
    }

    pub fn format(config: &ParamInfo, target: &mut StartSize){
        ScalingInterpolate::format(config, &mut target.scaling_interpolate)
    }
}
