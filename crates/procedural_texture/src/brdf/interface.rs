use pi_atom::Atom;
use pi_engine_shell::object::InterfaceObject;

use pi_scene_context::{object::ObjectID, materials::{material::{SingleMaterialIDCommandList, MaterialIDCommand}, material_meta::InterfaceMaterialMeta}};
use render_shader::shader::KeyShaderEffect;

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
        self.as_material(entity, KeyShaderEffect(Atom::from(BRDFShader::KEY)));

        entity
    }
}