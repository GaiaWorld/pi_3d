use pi_engine_shell::{engine_shell, object::{ObjectID, InterfaceObject}};
use pi_scene_context::materials::material_meta::InterfaceMaterialMeta;
use render_shader::shader::KeyShaderEffect;
use crate::shader::AxisShader;
use pi_atom::Atom;



pub trait InterfaceAxisMaterial {
    fn create_axis_material(&self) -> ObjectID;
}

impl InterfaceAxisMaterial for engine_shell::EnginShell {
    fn create_axis_material(&self) -> ObjectID {
        println!("create_default_material");
        let entity = self.new_object();
        self.as_material(entity, KeyShaderEffect(Atom::from(AxisShader::KEY)));

        entity
    }
    
}
