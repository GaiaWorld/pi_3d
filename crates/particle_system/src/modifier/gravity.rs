
use pi_scene_math::*;
use pi_scene_shell::prelude::*;

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
    pub fn modify(&self, item: &mut Number, amount: f32, _: f32, randoms: &BaseRandom) {
        *item = self.interpolation.interpolate(amount, randoms.x);
    }
}