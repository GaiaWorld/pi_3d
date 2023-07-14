use pi_engine_shell::{engine_shell, object::{ObjectID, InterfaceObject}};
use pi_scene_context::materials::shader_effect::InterfaceMaterialMeta;
use render_shader::shader::KeyShaderEffect;
use crate::shader::SkinShader;
use pi_atom::Atom;



pub trait InterfaceSkinMaterial {
    fn create_skin_material(&self) -> ObjectID;
}

impl InterfaceSkinMaterial for engine_shell::EnginShell {
    fn create_skin_material(&self) -> ObjectID {
        // log::debug!("create_default_material");
        let entity = self.new_object();
        self.as_material(entity, KeyShaderEffect(Atom::from(SkinShader::KEY)));

        entity
    }
    
}
