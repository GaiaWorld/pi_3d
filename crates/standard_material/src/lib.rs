use block_lighting::BlockStandardLighting;
use pi_scene_shell::prelude::*;
use pi_node_materials::NodeMaterialBlocks;
use pi_scene_context::prelude::*;
use shader::StandardShader;

pub mod shader;
pub mod interface;
pub mod block_lighting;


fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut nodematblocks: ResMut<NodeMaterialBlocks>,
) {
    nodematblocks.regist::<BlockStandardLighting>();
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(StandardShader::KEY), StandardShader::meta());
}

pub struct PluginStandardMaterial;
impl Plugin for PluginStandardMaterial {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup);
        app.add_startup_system(Update, setup);
    }
}