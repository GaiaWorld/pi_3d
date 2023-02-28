use pi_atom::Atom;
use pi_engine_shell::{object::{ObjectID, InterfaceObject}, engine_shell::EnginShell};
use pi_scene_context::{materials::interface::InterfaceMaterialMeta, pass::EPassTag};

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
        self.as_material(entity, Atom::from(CloudShader::KEY), EPassTag::Sky);

        entity
    }
}