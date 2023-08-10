use pi_scene_math::{Vector3, Number};
use pi_engine_shell::prelude::*;

#[derive(Default)]
pub struct RotationOverLifetime {
    pub rotation_interpolate: RotationInterpolate,
}

impl RotationOverLifetime {
    pub fn modify(&self, item: &mut Vector3, amount: f32, delta_seconds: f32, randoms: &BaseRandom) {
        let mut local_result = Vector3::zeros();

        self.rotation_interpolate.compute(amount, randoms, &mut local_result);

        local_result = local_result * delta_seconds;

        *item += local_result;
    }
}
