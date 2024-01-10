
use pi_scene_math::Vector3;
use pi_scene_shell::prelude::*;

#[derive(Clone)]
pub struct SizeBySpeed {
    pub(crate) range_x: f32,
    pub(crate) range_y: f32,
    pub(crate) range_size: f32,
    pub(crate) scaling_interpolate: ScalingInterpolate,
}

impl Default for SizeBySpeed {
    fn default() -> Self {
        Self {
            range_x: 0.,
            range_y: 1.,
            range_size: 1.,
            scaling_interpolate: ScalingInterpolate::default(),
        }
    }
}

impl SizeBySpeed {
    pub fn modify(&self, particle: &mut Vector3, speed: f32, randoms: &BaseRandom) {
        let mut local_result = Vector3::zeros();

        let amount = 1.0f32.min(0.0f32.max((speed - self.range_x) / self.range_size));
        self.scaling_interpolate.compute(amount, randoms, &mut local_result);

        particle.copy_from_slice(particle.component_mul(&local_result).as_slice());
    }
}

