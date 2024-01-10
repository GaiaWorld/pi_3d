use crate::base::TNodeMaterialBlock;
use pi_scene_shell::prelude::*;

pub struct BlockEmissiveFresnel;
impl BlockEmissiveFresnel {
    pub const KEY_LEFT: &'static str = "fresnelLeft";
    pub const KEY_RIGHT: &'static str = "fresnelRight";
    pub const KEY_PARAM: &'static str = "fresnelParam";
}
impl TNodeMaterialBlock for BlockEmissiveFresnel {
    const KEY: &'static str = "EMISSIVE_FRESNEL";

    const FS_DEFINED: &'static str = include_str!("./emissive_fresnel.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec4() -> Vec<UniformPropertyVec4> {
        vec![
            UniformPropertyVec4(Atom::from(Self::KEY_LEFT), [1., 1., 1., 1.], true),
            UniformPropertyVec4(Atom::from(Self::KEY_RIGHT), [1., 1., 1., 1.], true),
        ]
    }

    fn vec2() -> Vec<UniformPropertyVec2> {
        vec![
            UniformPropertyVec2(Atom::from(Self::KEY_PARAM), [0.4, 0.], true),
        ]
    }
}