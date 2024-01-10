use pi_scene_shell::prelude::*;
use pi_scene_math::Vector4;

#[derive(Clone)]
pub struct CustomDataForUV {
    pub u_scale: FloatInterpolation,
    pub v_scale: FloatInterpolation,
    pub u_offset: FloatInterpolation,
    pub v_offset: FloatInterpolation,
}
impl Default for CustomDataForUV {
    fn default() -> Self {
        let mut u_scale = FloatInterpolation::new(1.);
        let mut v_scale = FloatInterpolation::new(1.);
        let u_offset = FloatInterpolation::new(0.);
        let v_offset = FloatInterpolation::new(0.);
        u_scale.constant0 = Some(1.);
        v_scale.constant0 = Some(1.);
        Self {
            u_scale,
            v_scale,
            u_offset,
            v_offset,
        }
    }
}

impl CustomDataForUV {
    pub fn modify(&mut self, particle: &mut Vector4, amount: &mut f32, randoms: &BaseRandom) {
        let x = 1.0 / self.u_scale.interpolate(*amount, randoms.x);
        let y = 1.0 / self.v_scale.interpolate(*amount, randoms.y);
        let z = self.u_offset.interpolate(*amount, randoms.z);
        let w = self.v_offset.interpolate(*amount, randoms.w);
        particle.copy_from_slice(&[x, y, z, w]);
    }
}

