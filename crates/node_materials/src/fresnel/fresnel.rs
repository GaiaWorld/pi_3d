use crate::base::TNodeMaterialBlock;

pub struct BlockFresnel;
impl BlockFresnel {
}
impl TNodeMaterialBlock for BlockFresnel {
    const KEY: &'static str = "FRESNEL";

    const FS_DEFINED: &'static str = include_str!("./fresnel.glsl");

    const VS_DEFINED: &'static str = "";
}