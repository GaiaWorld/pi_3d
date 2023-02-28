use pi_atom::Atom;
use pi_engine_shell::plugin::Plugin;
use pi_scene_context::{materials::interface::InterfaceMaterialMeta};

use self::shader::PerlinNoiseShader;

pub mod shader;
pub mod interface;
pub mod command;

pub struct PluginPerlinNoise;
impl Plugin for PluginPerlinNoise {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let key = Atom::from(PerlinNoiseShader::KEY);
        engine.regist_material_meta(key, PerlinNoiseShader::meta());

        Ok(())
    }
}
