use pi_engine_shell::object::InterfaceObject;

use pi_scene_context::{object::ObjectID, materials::material::{SingleMaterialIDCommandList, MaterialIDCommand}};

use super::{command::{SingeDefaultMaterialCommandList, DefaultMaterialCommand}};


pub trait InterfaceDefaultMaterial {
    fn create_default_material(
        & self,
    ) -> ObjectID;

    fn as_default_material(
        & self,
        object: ObjectID,
    ) -> & Self;

    fn use_default_material(
        & self,
        object: ObjectID,
    ) -> & Self;
}

impl InterfaceDefaultMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_default_material(
        & self,
    ) -> ObjectID {
        //  log::debug!("create_default_material");
        let entity = self.new_object();

        self.as_default_material(entity);

        entity
    }
    fn as_default_material(
        & self,
        object: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingeDefaultMaterialCommandList>().unwrap();
        commands.list.push(DefaultMaterialCommand::Create(object));

        self
    }

    fn use_default_material(
        & self,
        object: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleMaterialIDCommandList>().unwrap();

        self
    }
}