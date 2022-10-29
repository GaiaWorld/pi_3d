use std::{any::TypeId, mem::replace};

use pi_ecs::{world::World, prelude::{ArchetypeId, StageBuilder}};

use crate::{resources::{SingleRenderBindGroupPool, SingleRenderObjectPipelinePool, SingleGeometryBufferPool}, object::{GameObject, ObjectID}, run_stage::RunStage, PluginBundleDefault, plugin::Plugin};

pub struct Engine {
    world: World,
    node_archetype_id: ArchetypeId,
}

impl Engine {
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn archetype_id(&self) -> ArchetypeId {
        self.node_archetype_id
    }

    pub fn new(world: &mut World) -> Self {
        // 注册原型
        world.new_archetype::<GameObject>().create();

        // 注册资源管理器

        // 注册单例资源
        world.insert_resource(SingleRenderBindGroupPool::default());
        world.insert_resource(SingleRenderObjectPipelinePool::default());
        world.insert_resource(SingleGeometryBufferPool::default());

        // 
        let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<GameObject>()).unwrap().clone();
        let archetype_id = world.archetypes_mut().get_or_create_archetype::<GameObject>();

        Self {
            world: world.clone(),
            node_archetype_id,
        }
    }

    pub fn init(
        &mut self,
        x: u32, y: u32, w: u32, h: u32,
    ) {

    }

    pub fn new_object(
        &mut self,
    ) -> ObjectID {
        unsafe { ObjectID::new(self.world.archetypes_mut()[self.node_archetype_id].reserve_entity()) }
    }

    pub fn tick_run(
        &mut self,
    ) {
        println!("Engine Tick Run: >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        let node_archetype_id = self.node_archetype_id;
        self.world.archetypes_mut()[node_archetype_id].flush();

        // let commands = replace(&mut self.commands, UserCommands::default());
        // self.world.insert_resource(commands);
    }
}
