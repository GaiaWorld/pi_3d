use pi_ecs::prelude::{Setup};
use pi_engine_shell::{run_stage::ERunStageChap, assets::sync_load::{PluginAssetSyncNotNeedLoad, AssetSyncLoad, PluginAssetSyncLoad}};
use pi_render::{renderer::shader::{KeyShaderMeta, KeyShader}, render_3d::shader::shader::{Shader3D, KeyShader3D}};

use self::{
    command::{SingleMaterialIDCommandList, SysMaterialIDCommand, SingleMatCreateCommands, SysMaterailCreateCommands},
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


type PluginAssetShaderEffectLoad = PluginAssetSyncLoad::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta, SysMaterailCreateCommands>;

pub struct PluginMaterial;
impl crate::Plugin for PluginMaterial {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();
        world.insert_resource(SingleMatCreateCommands::default());
        world.insert_resource(SingleMaterialIDCommandList::default());

        SysMaterailCreateCommands::setup(world, stages.query_stage::<SysMaterailCreateCommands>(ERunStageChap::Initial));

        PluginAssetSyncNotNeedLoad::<KeyShader3D, Shader3D>::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetShaderEffectLoad::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);

        let world = engine.world_mut();
        SysMaterialIDCommand::setup(world, stages.query_stage::<SysMaterialIDCommand>(ERunStageChap::Initial));

        PluginMaterialUniforms.init(engine, stages);

        Ok(())
    }
}