use pi_atom::Atom;
use pi_engine_shell::{object::{ObjectID, InterfaceObject}, engine_shell::EnginShell};
use pi_scene_context::materials::material_meta::InterfaceMaterialMeta;
use render_shader::shader::KeyShaderEffect;

use super::shader::CloudShader;


pub trait InterfaceCloudMaterial {
    fn create_cloud_material(
        &self,
    ) -> ObjectID;
}

impl InterfaceCloudMaterial for EnginShell {
    fn create_cloud_material(
        &self,
    ) -> ObjectID {
        let entity = self.new_object();
        self.as_material(entity, KeyShaderEffect(Atom::from(CloudShader::KEY)));

        entity
    }
}