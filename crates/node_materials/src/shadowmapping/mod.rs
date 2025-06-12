use pi_scene_shell::prelude::*;
use crate::{prelude::TNodeMaterialBlock, NodeMaterialBlocks};

pub struct BlockShadowMapping;
impl TNodeMaterialBlock for BlockShadowMapping {
    const KEY: &'static str = "ShadowMapping";

    const FS_DEFINED: &'static str = include_str!("./shadowmapping.glsl");

    const VS_DEFINED: &'static str = "
    ";

    const BIND_DEFINES: pi_scene_shell::prelude::BindDefine = pi_scene_shell::prelude::BindDefines::SHADOWMAP;
}


fn _setup(
    mut nodematblocks: ResMut<NodeMaterialBlocks>,
) {
    nodematblocks.regist::<BlockShadowMapping>();
}

pub struct PluginShadowMapping;
impl Plugin for PluginShadowMapping {
    fn build(&self, app: &mut App) {
        
        // #[cfg(not(target_arch="wasm32"))]
        // {
            let nodematblocks = app.world.get_resource_mut::<NodeMaterialBlocks>().unwrap();
            nodematblocks.regist::<BlockShadowMapping>();
        // }

        // app.add_systems(Startup, setup);
    }
}