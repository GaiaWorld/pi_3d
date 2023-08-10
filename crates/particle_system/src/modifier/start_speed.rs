use pi_scene_math::Vector3;
use pi_engine_shell::prelude::*;

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