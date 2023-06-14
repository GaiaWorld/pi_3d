use crate::particle::Particle;
use super::base::TranslationInterpolate;

pub struct StartSpeed{
    translation_interpolate: TranslationInterpolate,
}

impl StartSpeed{
    pub fn modify(&self, particle: &mut Particle, amount: f32, _delta_secondss: f32) {
        // println!("StartSpeed particle.direction: {:?}",  particle.direction);
        self.translation_interpolate.compute(amount, particle.base_random, particle.start_world_matrix_invert, &mut particle.direction);
    }
}