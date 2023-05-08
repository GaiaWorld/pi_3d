use crate::base::TNodeMaterialBlock;
use pi_engine_shell::prelude::*;

pub struct BlockEmissiveBase;
impl BlockEmissiveBase {
    pub const KEY_INFO: &'static str = "uEmissiveInfo";
}
impl TNodeMaterialBlock for BlockEmissiveBase {
    const KEY: &'static str = "EMISSIVE_BASE";

    const FS_DEFINED: &'static str = include_str!("./emissive_base.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec4() -> Vec<UniformPropertyVec4> {
        vec![
            UniformPropertyVec4(Atom::from(Self::KEY_INFO), [0., 0., 0., 1.]),
        ]
    }
}