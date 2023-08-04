use pi_scene_math::Vector3;

use crate::{particle::Particle, tools::BaseRandom};
use super::base::TranslationInterpolate;

#[derive(Default)]
pub struct StartSpeed{
    translation_interpolate: TranslationInterpolate,
}

impl StartSpeed{
    pub fn modify(&self, particle: &mut Vector3, amount: f32, randoms: &BaseRandom) {
        // println!("StartSpeed particle.direction: {:?}",  particle.direction);
        self.translation_interpolate.compute(amount, randoms, particle);
    }
}