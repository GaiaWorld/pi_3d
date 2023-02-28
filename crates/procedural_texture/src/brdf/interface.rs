use pi_atom::Atom;
use pi_engine_shell::object::InterfaceObject;

use pi_render::renderer::shader::KeyShaderMeta;
use pi_scene_context::{object::ObjectID, materials::interface::InterfaceMaterialMeta, pass::EPassTag};

use super::shader::BRDFShader;


pub trait InterfaceBRDFMaterial {
    fn create_brdf_material(
        & self,
    ) -> ObjectID;
}

impl InterfaceBRDFMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_brdf_material(
        & self,
    ) -> ObjectID {
        let entity = self.new_object();
        self.as_material(entity, KeyShaderMeta::from(BRDFShader::KEY), EPassTag::Opaque);

        entity
    }
}