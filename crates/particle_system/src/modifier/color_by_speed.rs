use pi_scene_math::Color4;

use crate::{particle::Particle, tools::BaseRandom};

use super::base::{Color4Interpolate, TEMP_COLOR4_A, IParticleModifier};

#[derive(Clone)]
pub struct ColorBySpeed {
    pub(crate) range_x: f32,
    pub(crate) range_y: f32,
    pub(crate) range_size: f32,
    pub(crate) color4_interpolate: Color4Interpolate,
}
impl Default for ColorBySpeed {
    fn default() -> Self {
        Self {
            range_x: 0.,
            range_y: 1.,
            range_size: 1.,
            color4_interpolate: Color4Interpolate::default(),
        }
    }
}

impl ColorBySpeed {
    pub fn modify(&self, item: &mut Color4, direction_length: f32, randoms: &BaseRandom) {
        let mut local_result = TEMP_COLOR4_A;
        let amount = 1.0f32.min(0.0f32.max((direction_length - self.range_x) / self.range_size));
        self.color4_interpolate.compute(amount, &mut local_result, randoms);

        *item = item.component_mul(&local_result);
    }
}
