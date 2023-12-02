use crate::base::TNodeMaterialBlock;

pub struct BlockFog;
impl BlockFog {
}
impl TNodeMaterialBlock for BlockFog {
    const KEY: &'static str = "FOG";

    const FS_DEFINED: &'static str = include_str!("./fog.glsl");

    const VS_DEFINED: &'static str = "";

    const BIND_DEFINES: pi_engine_shell::prelude::BindDefine = pi_engine_shell::prelude::BindDefines::SCENE_EFFECT;
}