// export class ColorOverLifetime extends Color4Interpolate implements IParticleModifier {
//     modify(particle: IParticle, amount: number, deltaSeconds: number) {
//         let localResult = TempColor4A;

//         this.compute(amount, localResult, particle.colorOverLifetimeAmount);

//         particle.color.multiplyToRef(localResult, particle.color);
//     }
// }

use crate::particle::Particle;

use super::base::{Color4Interpolate, IParticleModifier, TempColor4A};

#[derive(Clone)]
pub struct ColorOverLifetime {
    pub color4Interpolate: Color4Interpolate,
}

impl ColorOverLifetime {
    pub fn new(color4Interpolate: Color4Interpolate) -> Self {
        Self { color4Interpolate }
    }
}

impl IParticleModifier for ColorOverLifetime {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let mut localResult = TempColor4A;
// println!("color_over_lifetime_amount: {}", particle.color_over_lifetime_amount);
        self.color4Interpolate.compute(
            *amount,
            &mut localResult,
            particle.color_over_lifetime_amount,
        );
        

        particle.color = localResult.component_mul(&particle.color);
    }
}
