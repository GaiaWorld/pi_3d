// export class ColorOverLifetime extends Color4Interpolate implements IParticleModifier {
//     modify(particle: IParticle, amount: number, deltaSeconds: number) {
//         let localResult = TempColor4A;

//         this.compute(amount, localResult, particle.colorOverLifetimeAmount);

//         particle.color.multiplyToRef(localResult, particle.color);
//     }
// }

use crate::particle::Particle;

use super::base::{Color4Interpolate, IParticleModifier, TEMP_COLOR4_A};

#[derive(Clone)]
pub struct ColorOverLifetime {
    pub color4_interpolate: Color4Interpolate,
}

impl ColorOverLifetime {
    pub fn new(color4_interpolate: Color4Interpolate) -> Self {
        Self { color4_interpolate }
    }
}

impl IParticleModifier for ColorOverLifetime {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, _delta_seconds: f32) {
        let mut local_result = TEMP_COLOR4_A;
// println!("color_over_lifetime_amount: {}", particle.color_over_lifetime_amount);
        self.color4_interpolate.compute(
            *amount,
            &mut local_result,
            particle.color_over_lifetime_amount,
        );
        

        particle.color = local_result.component_mul(&particle.color);
    }
}
