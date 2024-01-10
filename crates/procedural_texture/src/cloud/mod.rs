
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

use self::shader::CloudShader;

pub mod shader;
pub mod interface;

fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(CloudShader::KEY), CloudShader::meta());
}

pub struct PluginCloudMaterial;
impl Plugin for PluginCloudMaterial {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_scene_shell::engine_shell::EnginShell,
    //     stages: &mut pi_scene_shell::run_stage::RunStage,
    // ) -> Result<(), pi_scene_shell::plugin::ErrorPlugin> {
    //     let key = Atom::from(CloudShader::KEY);
    //     engine.regist_material_meta(key, CloudShader::meta());

    //     Ok(())
    // }
}