

use pi_scene_math::Vector3;
use pi_engine_shell::prelude::*;

use crate::tools::Velocity;

#[derive(Clone)]
pub struct VelocityOverLifetime {
    pub(crate) is_local_space: bool,
    pub translation_interpolate: TranslationInterpolate,
}
impl Default for VelocityOverLifetime {
    fn default() -> Self {
        Self {
            is_local_space: true,
            translation_interpolate: TranslationInterpolate::default()
        }
    }
}

impl VelocityOverLifetime {
    pub fn format(config: &ParamInfo, target: &mut TranslationInterpolate){
        TranslationInterpolate::format(config, target)
    }
    pub fn modify(&self, item: &mut Velocity, amount: f32, randoms: &BaseRandom) {
        let mut local_result = Vector3::zeros();

        self.translation_interpolate.compute(amount, randoms, &mut local_result);

        let x = local_result[0];
        let y = local_result[1];
        let z = local_result[2];
        let delta = local_result - (item.value);
        item.delta = delta;
        item.value = Vector3::new(x, y, z);
    }
}

