use pi_engine_shell::prelude::*;

use crate::tools::Force;

#[derive(Clone)]
pub struct ForceOverLifetime {
    pub(crate) is_local_space: bool,
    pub translation_interpolate: TranslationInterpolate,
    // transformForce: Box<dyn Fn(&Vector3, Matrix, &mut Vector3)>,
}
impl Default for ForceOverLifetime {
    fn default() -> Self {
        Self {
            is_local_space: true,
            translation_interpolate: TranslationInterpolate::default(),
        }
    }
}

impl ForceOverLifetime {
    pub fn modify(&self, item: &mut Force, amount: f32, delta_seconds: f32, randoms: &BaseRandom) {
        self.translation_interpolate.compute(amount, randoms, &mut item.value);
    }
}
