pub mod shader;
pub mod skin;
pub mod interface;



use pi_engine_shell::{plugin::{Plugin, ErrorPlugin}, engine_shell::EnginShell, run_stage::RunStage};
use pi_scene_context::materials::shader_effect::InterfaceMaterialMeta;
use render_shader::shader::KeyShaderEffect;
use pi_atom::Atom;
use crate::shader::SkinShader;

pub struct PluginBones;
impl Plugin for PluginBones {
    fn init(
        &mut self,
        engine: &mut EnginShell,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin> {
        println!("PluginBones");
        let key = KeyShaderEffect(Atom::from(SkinShader::KEY));
        engine.regist_material_meta(key, SkinShader::meta());

        Ok(())
    }
}