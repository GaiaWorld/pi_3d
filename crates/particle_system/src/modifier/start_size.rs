// import { IParticle } from "../particle_base";
// import { IParticleModifier, ScalingInterpolate, TempVector3A, TranslationInterpolate } from "./base";

// export class StartSize extends ScalingInterpolate implements IParticleModifier {
//     modify(particle: IParticle, amount: number, deltaSeconds: number) {
//         this.compute(amount, particle.baseRandom, particle.startScaling);
//         particle.scaling.copyFrom(particle.startScaling);
//     }
// }

use pi_scene_math::Vector3;

use super::base::ScalingInterpolate;
use crate::{particle::Particle, iparticle_system_config::ParamInfo, tools::BaseRandom};

#[derive(Default)]
pub struct StartSize {
    pub scaling_interpolate: ScalingInterpolate,
}

impl StartSize {
    pub fn modify(&self, item: &mut Vector3, amount: f32, randoms: &BaseRandom) {
        self.scaling_interpolate.compute(amount, randoms, item);
    }

    pub fn format(config: &ParamInfo, target: &mut StartSize){
        ScalingInterpolate::format(config, &mut target.scaling_interpolate)
    }
}
