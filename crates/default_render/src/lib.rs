
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use shader::DefaultShader;

pub mod shader;
pub mod command;
pub mod interface;

#[derive(Debug, Clone, Copy, Resource)]
pub struct SingleIDBaseDefaultMaterial(pub Entity);

fn setup(
    mut commands: Commands,
    mut mat: ResMut<SingleIDBaseDefaultMaterial>,
    mut matcmds: ResMut<ActionListMaterialCreate>,
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut wait_list: ResMut<AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, &mut wait_list, KeyShaderMeta::from(DefaultShader::KEY), DefaultShader::res());

    let entity = mat.0;
    matcmds.push(OpsMaterialCreate(entity, KeyShaderMeta::from(DefaultShader::KEY), EPassTag::Opaque));
}

pub struct PluginDefaultMaterial;
impl Plugin for PluginDefaultMaterial {
    // fn init(
    //     &mut self,
    //     engine: &mut Engine,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), ErrorPlugin> {

    //     let world = engine.world_mut();
    //     SysDefaultMaterialCommand::setup(world, stages.query_stage::<SysDefaultMaterialCommand>(ERunStageChap::Command));

    //     let world = engine.world_mut();
    //     world.insert_resource(SingeDefaultMaterialCommandList::default());

    //     let key = KeyShaderMeta::from(DefaultShader::KEY);
    //     engine.regist_material_meta(key, DefaultShader::res());

    //     let base_default_id = engine.create_default_material(EPassTag::Opaque);
    //     let world = engine.world_mut();
    //     world.insert_resource(SingleIDBaseDefaultMaterial(MaterialID(base_default_id)));

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        let entity = app.world.spawn_empty().id();
        // log::warn!("Default Maerial {:?}", scene);
        let single = SingleIDBaseDefaultMaterial(entity);
        
        app.insert_resource(single);
        app.add_startup_system(setup);

        // let asset_mgr = app.world.get_resource::<ShareAssetMgr<ShaderEffectMeta>>().unwrap();
        // let key = KeyShaderMeta::from(DefaultShader::KEY);
        // if !asset_mgr.contains_key(&key) {
        //     if let Ok(meta) = asset_mgr.insert(key.clone(), DefaultShader::res()) {
        //         let mut wait_list = app.world.get_resource_mut::<AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>().unwrap();
        //         wait_list.1.push((key.clone(), meta));
        //     }
        // }

        // let mut matcmds = app.world.get_resource_mut::<ActionListMaterialCreate>().unwrap();
        // matcmds.push(OpsMaterialCreate(entity, KeyShaderMeta::from(DefaultShader::KEY), EPassTag::Opaque));
    }
}

