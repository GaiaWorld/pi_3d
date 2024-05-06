
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

use self::shader::BRDFShader;

pub mod shader;
pub mod interface;

fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(BRDFShader::KEY), BRDFShader::meta());
}


pub struct PluginBRDFMaterial;
impl Plugin for PluginBRDFMaterial {
    fn build(&self, app: &mut App) {
        app.add_system(Update, setup);
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_scene_shell::engine_shell::EnginShell,
    //     stages: &mut pi_scene_shell::run_stage::RunStage,
    // ) -> Result<(), pi_scene_shell::plugin::ErrorPlugin> {
    //     let key = Atom::from(BRDFShader::KEY);
    //     engine.regist_material_meta(key, BRDFShader::meta());

    //     Ok(())
    // }
}