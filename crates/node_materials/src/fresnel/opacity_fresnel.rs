use crate::base::TNodeMaterialBlock;
use pi_engine_shell::prelude::*;

pub struct BlockOpacityFresnel;
impl BlockOpacityFresnel {
    pub const KEY_LEFT: &'static str = "opacityFresnelLeft";
    pub const KEY_RIGHT: &'static str = "opacityFresnelRight";
    pub const KEY_PARAM: &'static str = "opacityFresnelParam";
}
impl TNodeMaterialBlock for BlockOpacityFresnel {
    const KEY: &'static str = "OPACITY_FRESNEL";

    const FS_DEFINED: &'static str = include_str!("./opacity_fresnel.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec2() -> Vec<UniformPropertyVec2> {
        vec![
            UniformPropertyVec2(Atom::from(Self::KEY_PARAM), [0.4, 0.]),
        ]
    }

    fn float() -> Vec<UniformPropertyFloat> {
        vec![
            UniformPropertyFloat(Atom::from(Self::KEY_LEFT), 0.),
            UniformPropertyFloat(Atom::from(Self::KEY_RIGHT), 0.),
        ]
    }
}