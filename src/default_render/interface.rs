use crate::{object::ObjectID, materials::material::{SingleMaterialIDCommandList, MaterialIDCommand}};

use super::{command::{SingeDefaultMaterialCommandList, DefaultMaterialCommand}, SingleIDBaseDefaultMaterial};


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

impl InterfaceDefaultMaterial for crate::engine::Engine {
    fn create_default_material(
        & self,
    ) -> ObjectID {
        //  println!("create_default_material");
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

        let base_material = world.get_resource::<SingleIDBaseDefaultMaterial>().unwrap();
        let commands = world.get_resource_mut::<SingleMaterialIDCommandList>().unwrap();
        commands.list.push(MaterialIDCommand::Use(object, base_material.0));

        self
    }
}