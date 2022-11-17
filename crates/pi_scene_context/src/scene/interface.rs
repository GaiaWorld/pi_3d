use pi_engine_shell::object::InterfaceObject;

use crate::{object::ObjectID, flags::SceneID};

use super::command::{SingleSceneCommandList, SceneCommand};


pub trait InterfaceScene {
    fn create_scene(
        & self,
    ) -> ObjectID;

    fn add_to_scene(
        & self,
        object: ObjectID,
        scene: ObjectID,
    ) -> & Self;
}

impl InterfaceScene for crate::engine::Engine {
    fn create_scene(
        & self,
    ) -> ObjectID {
        let entity = self.new_object();
        let world = self.world();

        let commands = world.get_resource_mut::<SingleSceneCommandList>().unwrap();
        commands.list.push(SceneCommand::Create(entity));

        entity
    }

    fn add_to_scene(
        & self,
        object: ObjectID,
        scene: ObjectID,
    ) -> & Self {
        let world = self.world();
        
        let commands = world.get_resource_mut::<SingleSceneCommandList>().unwrap();
        commands.list.push(SceneCommand::AddObject(object, SceneID(scene)));

        self
    }
}