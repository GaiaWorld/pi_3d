

use command::{ActionListUnlitMaterial, sys_act_unlit_material};
use pi_engine_shell::prelude::*;

use pi_scene_context::prelude::*;

use shader::UnlitShader;

pub mod shader;
pub mod command;
pub mod interface;
pub mod effects;

fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut wait_list: ResMut<AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, &mut wait_list, KeyShaderMeta::from(UnlitShader::KEY), UnlitShader::meta());
}

pub struct PluginUnlitMaterial;
impl Plugin for PluginUnlitMaterial {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListUnlitMaterial::default());
        app.add_system(sys_act_unlit_material.in_set(ERunStageChap::Command));

        let asset_mgr = app.world.get_resource::<ShareAssetMgr<ShaderEffectMeta>>().unwrap().clone();
        let mut wait_list = app.world.get_resource_mut::<AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>().unwrap();
        ActionMaterial::regist_material_meta(&asset_mgr, &mut wait_list, KeyShaderMeta::from(UnlitShader::KEY), UnlitShader::meta());
        // app.add_startup_system(setup);
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {

    //     let key = KeyShaderMeta::from(UnlitShader::KEY);
    //     engine.regist_material_meta(key, UnlitShader::meta());

    //     let world = engine.world_mut();
    //     world.insert_resource(SingleUnlitMaterialCommandList::default());

    //     SysUnlitMaterialCommand::setup(world, stages.query_stage::<SysUnlitMaterialCommand>(ERunStageChap::Command));
        
    //     Ok(())
    // }
}