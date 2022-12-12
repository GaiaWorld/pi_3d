use pi_atom::Atom;
use pi_engine_shell::plugin::Plugin;
use pi_scene_context::materials::material_meta::InterfaceMaterialMeta;
use render_shader::shader::KeyShaderEffect;

use self::shader::CloudShader;

pub mod shader;
pub mod material;
pub mod interface;
pub mod command;
// pub mod material_sys;

pub struct PluginCloudMaterial;
impl Plugin for PluginCloudMaterial {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let key = KeyShaderEffect(Atom::from(CloudShader::KEY));
        engine.regist_material_meta(key, CloudShader::meta());

        Ok(())
    }
}