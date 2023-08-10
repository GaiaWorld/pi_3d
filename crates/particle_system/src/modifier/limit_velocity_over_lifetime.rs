use pi_engine_shell::prelude::*;

use crate::tools::LimitVelocityScalar;

#[derive(Clone)]
pub struct LimitVelocityOverLifetime {
    pub interpolation: FloatInterpolation,
    pub dampen: f32,
}
impl Default for LimitVelocityOverLifetime {
    fn default() -> Self {
        Self {
            interpolation: FloatInterpolation::default(),
            dampen: 0.0,
        }
    }
}

impl LimitVelocityOverLifetime {
    pub fn modify(&self, item: &mut LimitVelocityScalar, amount: f32, randoms: &BaseRandom) {
        item.value = self.interpolation.interpolate(amount, randoms.x);
        item.dampen = self.dampen;

        // let curr_length = particle.direction_length;
        // if curr_length > local_result {
        //     // println!("LimitVelocityOverLifetime particle.direction: {:?}",  particle.direction);
        //     particle.direction = particle.direction
        //         * (1.0 - self.dampen * (curr_length - local_result) / curr_length * (0.66));
        // }
    }
}
