use crate::{base::TNodeMaterialBlock};
use pi_engine_shell::prelude::*;

pub struct BlockCutoff;
impl BlockCutoff {
    pub const KEY_VALUE: &'static str = "uCutoff";
}
impl TNodeMaterialBlock for BlockCutoff {
    const KEY: &'static str = "CUTOFF";

    const FS_DEFINED: &'static str = include_str!("./cutoff.glsl");

    const VS_DEFINED: &'static str = "";

    fn float() -> Vec<UniformPropertyFloat> {
        vec![
            UniformPropertyFloat(Atom::from(Self::KEY_VALUE), 0.)
        ]
    }
}
