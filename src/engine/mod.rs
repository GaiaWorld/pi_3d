use std::{any::TypeId, mem::replace, sync::Arc};

use pi_ecs::{world::World, prelude::{ArchetypeId, StageBuilder}, storage::Local};

use crate::{resources::{SingleRenderBindGroupPool, SingleRenderObjectPipelinePool, SingleGeometryBufferPool}, object::{GameObject, ObjectID}, run_stage::RunStage, PluginBundleDefault, plugin::Plugin};

pub struct Engine {
    node_archetype_id: ArchetypeId,
    world: World,
    // world_call: Box<dyn Fn() -> &'static World>,
}

impl Engine {
    pub fn world(&self) -> & World {
        &self.world

        // let call = self.world_call.as_ref();
        // call()
    }

    pub fn archetype_id(&self) -> ArchetypeId {
        self.node_archetype_id
    }

    pub fn new(
        world: World,
        node_archetype_id: Local,
        // world_call: Box<dyn Fn() -> &'static World>,
    ) -> Self {

        Self {
            node_archetype_id,
            world,
            // world_call
        }
    }

    pub fn init(
        &mut self,
        x: u32, y: u32, w: u32, h: u32,
    ) {

    }

    pub fn new_object(
        & self,
    ) -> ObjectID {
        let world = self.world();
        unsafe { ObjectID::new(world.clone().archetypes_mut()[self.node_archetype_id].reserve_entity()) }
    }

    pub fn tick_run(
        &self,
    ) {
        let world = self.world();
    
        //  println!("Engine Tick Run: >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        let node_archetype_id = self.node_archetype_id;
        world.clone().archetypes_mut()[node_archetype_id].flush();

        // let commands = replace(&mut self.commands, UserCommands::default());
        // self.world.insert_resource(commands);
    }
}
