
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use shader::DefaultShader;

pub mod shader;
pub mod command;
pub mod interface;


fn setup(
    mat: Res<SingleIDBaseDefaultMaterial>,
    mut matcmds: ResMut<ActionListMaterialCreate>,
) {
    // ActionMaterial::regist_material_meta(&asset_mgr, &mut wait_list, KeyShaderMeta::from(DefaultShader::KEY), DefaultShader::res());

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
        
        let asset_mgr = app.world.get_resource::<ShareAssetMgr<ShaderEffectMeta>>().unwrap().clone();
        ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(DefaultShader::KEY), DefaultShader::res());

        app.add_systems(Update, setup);
    }
}

