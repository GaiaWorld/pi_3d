use crate::particle::Particle;
use super::base::TranslationInterpolate;

pub struct StartSpeed{
    translationInterpolate: TranslationInterpolate,
}

impl StartSpeed{
    pub fn modify(&self, particle: &mut Particle, amount: f32, deltaSeconds: f32) {
        // println!("StartSpeed particle.direction: {:?}",  particle.direction);
        self.translationInterpolate.compute(amount, particle.base_random, particle.start_world_matrix_invert, &mut particle.direction);
    }
}