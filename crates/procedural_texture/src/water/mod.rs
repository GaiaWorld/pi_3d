
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

use crate::water::shader::WaterShader;

pub mod shader;
pub mod interface;

fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(WaterShader::KEY), WaterShader::meta());
}

pub struct PluginWaterMaterial;
impl Plugin for PluginWaterMaterial {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), ErrorPlugin> {
    //     log::debug!("PluginWaterMaterial");
    //     let key = Atom::from(WaterShader::KEY);
    //     engine.regist_material_meta(key, WaterShader::meta());

    //     Ok(())
    // }
}