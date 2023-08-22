
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

use self::shader::BRDFShader;

pub mod shader;
pub mod interface;

fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut wait_list: ResMut<AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, &mut wait_list, KeyShaderMeta::from(BRDFShader::KEY), BRDFShader::meta());
}


pub struct PluginBRDFMaterial;
impl Plugin for PluginBRDFMaterial {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
    //     let key = Atom::from(BRDFShader::KEY);
    //     engine.regist_material_meta(key, BRDFShader::meta());

    //     Ok(())
    // }
}