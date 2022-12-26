pub mod shader;
pub mod axis;
pub mod interface;



use pi_engine_shell::{plugin::{Plugin, ErrorPlugin}, engine_shell::EnginShell, run_stage::RunStage};
use pi_scene_context::materials::material_meta::InterfaceMaterialMeta;
use render_shader::shader::KeyShaderEffect;
use pi_atom::Atom;
use crate::shader::AxisShader;

pub struct PluginAxis;
impl Plugin for PluginAxis {
    fn init(
        &mut self,
        engine: &mut EnginShell,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin> {
        println!("PluginAxis");
        let key = KeyShaderEffect(Atom::from(AxisShader::KEY));
        engine.regist_material_meta(key, AxisShader::meta());

        Ok(())
    }
}