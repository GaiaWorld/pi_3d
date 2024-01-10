use pi_scene_math::Number;
use pi_scene_shell::prelude::*;
use crate::{tools::SpeedFactor};


pub struct SpeedModifier {
    /// 同时影响 线性速度 和 轨道角速度
    pub speed_modifier: FloatInterpolation,
}
impl Default for SpeedModifier {
    fn default() -> Self {
        Self { speed_modifier: FloatInterpolation::new(0.) }
    }
}
impl SpeedModifier {
    pub fn modify(&self, item: &mut SpeedFactor, amount: Number, randoms: &BaseRandom) {
        item.value = self.speed_modifier.interpolate(amount, randoms.x);
    }
}
