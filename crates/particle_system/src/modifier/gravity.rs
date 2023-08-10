
use pi_scene_math::{Vector3, Number};
use pi_engine_shell::prelude::*;

pub struct Gravity {
    pub interpolation: FloatInterpolation,
}
impl Default for Gravity {
    fn default() -> Self {
        Self {
            interpolation: FloatInterpolation::new(0.),
        }
    }
}

impl Gravity {
    pub fn modify(&self, item: &mut Number, amount: f32, delta_seconds: f32, randoms: &BaseRandom) {
        *item = self.interpolation.interpolate(amount, randoms.x);
    }
}