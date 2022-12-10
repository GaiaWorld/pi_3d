
use command::{SysUnlitMaterialCommand, SingleUnlitMaterialCommandList};
use pi_atom::Atom;
use pi_ecs::prelude::Setup;
use pi_engine_shell::plugin::Plugin;
use pi_scene_context::materials::material_meta::InterfaceMaterialMeta;
use render_shader::shader::KeyPreShader;
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

        let key = KeyPreShader(Atom::from(UnlitShader::KEY));
        engine.regist_material_meta(key, UnlitShader::res());

        let world = engine.world_mut();
        world.insert_resource(SingleUnlitMaterialCommandList::default());

        SysUnlitMaterialCommand::setup(world, stages.command_stage());
        
        Ok(())
    }
}