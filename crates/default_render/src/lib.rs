
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use shader::DefaultShader;

pub mod shader;
pub mod command;
pub mod interface;

pub struct PluginDefaultMaterial;
impl Plugin for PluginDefaultMaterial {
    fn build(&self, app: &mut App) {
        
        let asset_mgr = app.world.get_resource::<ShareAssetMgr<ShaderEffectMeta>>().unwrap().clone();
        ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(DefaultShader::KEY), DefaultShader::res());

    }
}

