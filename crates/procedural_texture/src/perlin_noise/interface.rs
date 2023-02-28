use pi_atom::Atom;
use pi_engine_shell::{object::{ObjectID, InterfaceObject}, engine_shell::EnginShell};
use pi_scene_context::{materials::interface::InterfaceMaterialMeta, pass::EPassTag};

use super::shader::PerlinNoiseShader;


pub trait InterfacePerlinNoiseMaterial {
    fn create_perlinnoise_material(
        &self,
    ) -> ObjectID;
}

impl InterfacePerlinNoiseMaterial for EnginShell {
    fn create_perlinnoise_material(
        &self,
    ) -> ObjectID {
        let entity = self.new_object();
        self.as_material(entity, Atom::from(PerlinNoiseShader::KEY), EPassTag::Opaque);

        entity
    }
}