use crate::{base::TNodeMaterialBlock, common::BlockTextureChannel};
use pi_engine_shell::prelude::*;

pub struct BlockPremultiplyResult;
impl BlockPremultiplyResult {
    pub const KEY_FLAG: &'static str = "uPremultiplyResult";
}
impl TNodeMaterialBlock for BlockPremultiplyResult {
    const KEY: &'static str = "PremultiplyResult";
    const FS_DEFINED: &'static str = "
vec4 PremultiplyResult(vec4 finalColor) {
    finalColor.rgb = mix(1., finalColor.a, step(0.5, PremultiplyResult));
    return finalColor;
}
    ";

    const VS_DEFINED: &'static str = "";

    fn float() -> Vec<UniformPropertyFloat> {
        vec![
            UniformPropertyFloat(Atom::from(Self::KEY_FLAG), 0.0)
        ]
    }
}