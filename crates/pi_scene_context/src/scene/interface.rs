use pi_engine_shell::prelude::*;

use crate::{object::ObjectID, flags::SceneID};

use super::command::{init_scene, add_to_scene};


pub trait InterfaceScene {
    fn create_scene(
        &mut self,
    ) -> ObjectID;

    fn add_to_scene(
        &mut self,
        object: ObjectID,
        scene: ObjectID,
    ) -> &mut Self;
}

impl InterfaceScene for EnginShell {
    fn create_scene(
        &mut self,
    ) -> ObjectID {
        let entity = self.world.spawn_empty().id();
        let left = self.world.spawn_empty().id();
        let right = self.world.spawn_empty().id();

        let device = self.world.get_resource::<PiRenderDevice>().unwrap();

        let mut dynbuffer = self.world.get_resource_mut::<ResBindBufferAllocator>().unwrap();

        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &self.world);

        init_scene(entity, left, right, &mut commands, device, &mut dynbuffer);
    
        queue.apply(&mut self.world);

        entity
    }

    fn add_to_scene(
        &mut self,
        object: ObjectID,
        scene: ObjectID,
    ) -> &mut Self {
        
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &self.world);

        add_to_scene(object, scene, &mut commands);

        queue.apply(&mut self.world);

        self
    }
}