use pi_atom::Atom;
use pi_engine_shell::{engine_shell::EnginShell, plugin::{Plugin, ErrorPlugin}};
use render_shader::shader::KeyShaderEffect;
use pi_scene_context::materials::shader_effect::InterfaceMaterialMeta;
use crate::water::shader::WaterShader;

pub mod shader;
pub mod interface;


pub struct PluginWaterMaterial;
impl Plugin for PluginWaterMaterial {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        log::debug!("PluginWaterMaterial");
        let key = KeyShaderEffect(Atom::from(WaterShader::KEY));
        engine.regist_material_meta(key, WaterShader::meta());

        Ok(())
    }
}