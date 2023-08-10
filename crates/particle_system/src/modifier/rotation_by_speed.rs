use pi_scene_math::Vector3;
use pi_engine_shell::prelude::*;

#[derive(Clone)]
pub struct RotationBySpeed {
    pub(crate) range_x: f32,
    pub(crate) range_y: f32,
    pub(crate) range_size: f32,
    pub rotation_interpolate: RotationInterpolate,
}
impl Default for RotationBySpeed {
    fn default() -> Self {
        Self {
            range_x: 0.,
            range_y: 1.,
            range_size: 1.,
            rotation_interpolate: RotationInterpolate::default(),
        }
    }
}

impl RotationBySpeed {
    pub fn modify(&self, particle: &mut Vector3, speed: f32, delta_seconds: f32, randoms: &BaseRandom) {
        let mut local_result = Vector3::zeros();

        let amount = 1.0f32.min(0.0f32.max((speed - self.range_x) / self.range_size));
        self.rotation_interpolate.compute(amount, randoms, &mut local_result);

        local_result = local_result * delta_seconds;

        *particle = particle.clone() + local_result;
    }
}