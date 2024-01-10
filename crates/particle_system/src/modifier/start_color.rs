use pi_scene_math::{Vector4, Number};
use pi_scene_shell::prelude::*;

#[derive(Default)]
pub struct StartColor {
    color4_interpolate: Color4Interpolate,
}

impl StartColor {
    pub fn modify(&self, item: &mut Vector4, amount: Number, randoms: &BaseRandom) {
        self.color4_interpolate.compute(amount, item, randoms);
    }

    pub fn format(config: &FourGradientInfo, target: &mut StartColor) {
        Color4Interpolate::format(config, &mut target.color4_interpolate);
    }
}
