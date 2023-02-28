pub mod shader;
pub mod axis;
pub mod interface;



use pi_engine_shell::{plugin::{Plugin, ErrorPlugin}, engine_shell::EnginShell, run_stage::RunStage};
use pi_render::renderer::shader::KeyShaderMeta;
use pi_scene_context::materials::interface::InterfaceMaterialMeta;
use crate::shader::AxisShader;

pub struct PluginAxis;
impl Plugin for PluginAxis {
    fn init(
        &mut self,
        engine: &mut EnginShell,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin> {
        log::debug!("PluginAxis");
        let key = KeyShaderMeta::from(AxisShader::KEY);
        engine.regist_material_meta(key, AxisShader::meta());

        Ok(())
    }
}