
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

use self::shader::PerlinNoiseShader;

pub mod shader;
pub mod interface;
pub mod command;

fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(PerlinNoiseShader::KEY), PerlinNoiseShader::meta());
}

pub struct PluginPerlinNoise;
impl Plugin for PluginPerlinNoise {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_scene_shell::engine_shell::EnginShell,
    //     stages: &mut pi_scene_shell::run_stage::RunStage,
    // ) -> Result<(), pi_scene_shell::plugin::ErrorPlugin> {
    //     let key = Atom::from(PerlinNoiseShader::KEY);
    //     engine.regist_material_meta(key, PerlinNoiseShader::meta());

    //     Ok(())
    // }
}
