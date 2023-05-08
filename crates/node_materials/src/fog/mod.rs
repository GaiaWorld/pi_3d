use crate::base::TNodeMaterialBlock;
use pi_engine_shell::prelude::*;

pub struct BlockFog;
impl BlockFog {
}
impl TNodeMaterialBlock for BlockFog {
    const KEY: &'static str = "FOG";

    const FS_DEFINED: &'static str = include_str!("./fog.glsl");

    const VS_DEFINED: &'static str = "";

}