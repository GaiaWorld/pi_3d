use super::base::{IParticleModifier, ScalingInterpolate};
use crate::{interpolation::FloatInterpolation, tools::*, particle::Particle};
use pi_scene_math::{Vector3, Number};

#[derive(Default)]
pub struct SizeOverLifetime {
    pub scaling_interpolate: ScalingInterpolate,
}

impl SizeOverLifetime {
    pub fn modify(&self, item: &mut Vector3, amount: f32, randoms: &BaseRandom) {
        let mut local_result = Vector3::zeros();

        self.scaling_interpolate.compute(amount, randoms, &mut local_result);

        item.x *= local_result.x;
        item.y *= local_result.y;
        item.z *= local_result.z;
    }
}
