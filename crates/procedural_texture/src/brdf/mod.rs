use pi_atom::Atom;
use pi_ecs::prelude::Setup;
use pi_engine_shell::plugin::Plugin;
use pi_scene_context::materials::shader_effect::InterfaceMaterialMeta;
use render_shader::shader::KeyShaderEffect;

use self::shader::BRDFShader;

pub mod shader;
pub mod interface;
pub struct PluginBRDFMaterial;
impl Plugin for PluginBRDFMaterial {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let key = KeyShaderEffect(Atom::from(BRDFShader::KEY));
        engine.regist_material_meta(key, BRDFShader::meta());

        Ok(())
    }
}