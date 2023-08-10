// export class ColorOverLifetime extends Color4Interpolate implements IParticleModifier {
//     modify(particle: IParticle, amount: number, deltaSeconds: number) {
//         let localResult = TempColor4A;

//         this.compute(amount, localResult, particle.colorOverLifetimeAmount);

//         particle.color.multiplyToRef(localResult, particle.color);
//     }
// }

use pi_scene_math::{Number, Color4};
use pi_engine_shell::prelude::*;

#[derive(Clone, Default)]
pub struct ColorOverLifetime {
    pub color4_interpolate: Color4Interpolate,
}

impl ColorOverLifetime {
    pub fn modify(&self, item: &mut Color4, amount:  f32, randoms: &BaseRandom) {
        self.color4_interpolate.compute(amount, item, randoms);
    }
}

