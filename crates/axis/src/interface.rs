use pi_engine_shell::{engine_shell, object::{ObjectID, InterfaceObject}};
use pi_render::renderer::shader::KeyShaderMeta;
use pi_scene_context::{materials::{interface::InterfaceMaterialMeta}, pass::EPassTag};
use crate::shader::AxisShader;

pub trait InterfaceAxisMaterial {
    fn create_axis_material(&self) -> ObjectID;
}

impl InterfaceAxisMaterial for engine_shell::EnginShell {
    fn create_axis_material(&self) -> ObjectID {
        log::debug!("create_default_material");
        let entity = self.new_object();
        self.as_material(entity, KeyShaderMeta::from(AxisShader::KEY), EPassTag::Opaque);

        entity
    }
    
}
