use crate::base::TNodeMaterialBlock;
use pi_engine_shell::prelude::*;

pub struct BlockEmissiveTextureUVOffsetSpeed;
impl BlockEmissiveTextureUVOffsetSpeed {
    pub const KEY_PARAM: &'static str = "uEmissiveUVOS";
}
impl TNodeMaterialBlock for BlockEmissiveTextureUVOffsetSpeed {
    const KEY: &'static str = "EMISSIVE_TEXTURE_UVOS";

    const FS_DEFINED: &'static str = "";

    const VS_DEFINED: &'static str = "";

    fn vec2() -> Vec<UniformPropertyVec2> {
        vec![
            UniformPropertyVec2(Atom::from(Self::KEY_PARAM), [1., 1.])
        ]
    }
}