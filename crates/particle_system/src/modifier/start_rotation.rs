use pi_scene_math::Vector3;
use pi_scene_shell::prelude::*;

#[derive(Default)]
pub struct StartRotation {
    pub rotation_interpolate: RotationInterpolate,
}

impl StartRotation {
    pub fn modify(&self, item: &mut Vector3, amount: f32, randoms: &BaseRandom) {
        self.rotation_interpolate.compute(amount, randoms, item);
    }

    pub fn format(config: &ParamInfo,  target: &mut StartRotation) {
        RotationInterpolate::format(config, &mut target.rotation_interpolate)
    }
}
