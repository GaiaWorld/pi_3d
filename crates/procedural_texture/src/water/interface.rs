use pi_atom::Atom;
use pi_scene_context::{
    materials::{
        interface::InterfaceMaterialMeta,
    },
    object::ObjectID,
    pass::EPassTag,
};
use pi_engine_shell::object::InterfaceObject;

use crate::water::shader::WaterShader;




pub trait InterfaceWaterMaterial {
    fn create_water_material(&self) -> ObjectID;
}

impl InterfaceWaterMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_water_material(&self) -> ObjectID {
        log::debug!("create_default_material");
        let entity = self.new_object();
        self.as_material(entity, Atom::from(WaterShader::KEY), EPassTag::Opaque);

        entity
    }
    
}
