use pi_atom::Atom;
use pi_engine_shell::plugin::Plugin;
use pi_scene_context::materials::interface::InterfaceMaterialMeta;

use self::shader::CloudShader;

pub mod shader;
pub mod interface;

pub struct PluginCloudMaterial;
impl Plugin for PluginCloudMaterial {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let key = Atom::from(CloudShader::KEY);
        engine.regist_material_meta(key, CloudShader::meta());

        Ok(())
    }
}