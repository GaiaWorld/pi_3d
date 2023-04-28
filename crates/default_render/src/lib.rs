
use pi_engine_shell::prelude::*;
use pi_scene_context::{materials::{material::{MaterialID}, command::{ActionMaterial, ActionListMaterialCreate, OpsMaterialCreate}, shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta}}, pass::EPassTag};
use shader::DefaultShader;

pub mod shader;
pub mod command;
pub mod interface;

#[derive(Debug, Clone, Copy, Resource)]
pub struct SingleIDBaseDefaultMaterial(pub Option<Entity>);

fn setup(
    mut commands: Commands,
    mut mat: ResMut<SingleIDBaseDefaultMaterial>,
    mut matcmds: ResMut<ActionListMaterialCreate>,
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut wait_list: ResMut<AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, &mut wait_list, KeyShaderMeta::from(DefaultShader::KEY), DefaultShader::res());

    let entity = commands.spawn_empty().id();
    matcmds.push(OpsMaterialCreate(entity, KeyShaderMeta::from(DefaultShader::KEY), EPassTag::Opaque));

    mat.0 = Some(entity);
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
        app.insert_resource(SingleIDBaseDefaultMaterial(None));
        app.add_startup_system(setup);
    }
}

