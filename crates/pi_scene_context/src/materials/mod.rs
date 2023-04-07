
use pi_engine_shell::{prelude::*, assets::sync_load::{sys_sync_load_create, sys_sync_load_check_await}};
use pi_render::{renderer::shader::{KeyShaderMeta, KeyShader}, render_3d::shader::shader::{Shader3D, KeyShader3D}};

use self::{
    command::*,
    uniforms::PluginMaterialUniforms, shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta, ShaderEffectMeta}
};

pub mod material;
pub mod material_meta;
pub mod uniforms;
pub mod value;
pub mod shader_effect;
pub mod command;
pub mod interface;
pub mod system;

pub type MBKK = usize;


// type PluginAssetShaderEffectLoad = PluginAssetSyncLoad::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta, SysMaterailCreateCommands>;

pub struct PluginMaterial;
impl crate::Plugin for PluginMaterial {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            (
                sys_sync_load_create::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>,
                sys_sync_load_check_await::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>
            ).chain()
        );
        
        app.add_systems(
            (
                sys_sync_load_create::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>,
                sys_sync_load_check_await::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>
            ).chain()
        );
    }
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();
    //     world.insert_resource(SingleMatCreateCommands::default());
    //     world.insert_resource(SingleMaterialIDCommandList::default());

    //     SysMaterailCreateCommands::setup(world, stages.query_stage::<SysMaterailCreateCommands>(ERunStageChap::Initial));

    //     PluginAssetSyncNotNeedLoad::<KeyShader3D, Shader3D>::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);
    //     PluginAssetShaderEffectLoad::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);

    //     let world = engine.world_mut();
    //     SysMaterialIDCommand::setup(world, stages.query_stage::<SysMaterialIDCommand>(ERunStageChap::Initial));

    //     PluginMaterialUniforms.init(engine, stages);

    //     Ok(())
    // }
}