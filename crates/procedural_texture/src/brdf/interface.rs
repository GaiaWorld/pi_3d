use pi_engine_shell::object::InterfaceObject;

use pi_scene_context::{object::ObjectID, materials::material::{SingleMaterialIDCommandList, MaterialIDCommand}};

use super::{command::{SingleUnlitMaterialCommandList, UnlitMaterialCommand}};


pub trait InterfaceUnlitMaterial {
    fn create_unlit_material(
        & self,
    ) -> ObjectID;
}

impl InterfaceUnlitMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_unlit_material(
        & self,
    ) -> ObjectID {
        //  println!("create_unlit_material");
        let entity = self.new_object();

        let world = self.world();
        let commands = world.get_resource_mut::<SingleUnlitMaterialCommandList>().unwrap();
        commands.list.push(UnlitMaterialCommand::Create(entity));

        entity
    }
}