
use command::{SysUnlitMaterialCommand, SingleUnlitMaterialCommandList};
use pi_ecs::prelude::Setup;
use pi_engine_shell::{plugin::Plugin, run_stage::ERunStageChap};
use pi_render::renderer::shader::KeyShaderMeta;
use pi_scene_context::materials::interface::InterfaceMaterialMeta;
use shader::UnlitShader;

pub mod shader;
pub mod command;
pub mod interface;

pub struct PluginUnlitMaterial;
impl Plugin for PluginUnlitMaterial {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {

        let key = KeyShaderMeta::from(UnlitShader::KEY);
        engine.regist_material_meta(key, UnlitShader::meta());

        let world = engine.world_mut();
        world.insert_resource(SingleUnlitMaterialCommandList::default());

        SysUnlitMaterialCommand::setup(world, stages.query_stage::<SysUnlitMaterialCommand>(ERunStageChap::Command));
        
        Ok(())
    }
}