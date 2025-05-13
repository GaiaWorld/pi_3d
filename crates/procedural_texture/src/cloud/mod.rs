
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

use self::shader::CloudShader;

pub mod shader;
pub mod interface;

fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    engineopt: Res<EngineCustomPlugins>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(CloudShader::KEY), CloudShader::meta(&engineopt));
}

pub struct PluginCloudMaterial;
impl Plugin for PluginCloudMaterial {
    fn build(&self, app: &mut App) {
        app.add_systems(StageD3, setup);
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