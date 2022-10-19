use std::{any::TypeId, mem::replace};

use pi_ecs::{world::World, prelude::{ArchetypeId, StageBuilder}};

use crate::{resources::command::{UserCommands, TransformNodeTreeCommand, ObjectNewCommand}, object::{GameObject, ObjectID}, systems::init_stage};

pub struct Engine {
    world: World,
    node_archetype_id: ArchetypeId,

    commands: UserCommands,
}

impl Engine {
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn new(world: &mut World) -> Self {
        // 注册原型
        world.new_archetype::<GameObject>().create();

        // 注册资源管理器
        

        // 
        let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<GameObject>()).unwrap().clone();

        let archetype_id = world.archetypes_mut().get_or_create_archetype::<GameObject>();


        Self {
            world: world.clone(),
            node_archetype_id,
            commands: UserCommands::default(),
        }
    }

    pub fn init(
        &mut self,
        x: u32, y: u32, w: u32, h: u32,
    ) -> Vec<StageBuilder> {
        // 建立System运行队列
        init_stage(&mut self.world)
    }

    pub fn new_scene(
        &mut self,
    ) -> ObjectID {
        let entity = unsafe { ObjectID::new(self.world.archetypes_mut()[self.node_archetype_id].reserve_entity()) };
        self.commands.new_objects.push(ObjectNewCommand::NewScene(entity));

        entity
    }

    pub fn destroy_scene(
        &mut self,
        scene: ObjectID,
    ) {
        self.commands.tree.push(TransformNodeTreeCommand::Destroy(scene));
    }

    pub fn new_transform_node(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID {
        let entity = unsafe { ObjectID::new(self.world.archetypes_mut()[self.node_archetype_id].reserve_entity()) };
        self.commands.new_objects.push(ObjectNewCommand::NewTransformNode(entity));
        self.commands.tree.push(TransformNodeTreeCommand::Append(entity, scene));

        entity
    }

    pub fn set_parent(
        &mut self,
        node: ObjectID,
        scene: ObjectID,
        parent: Option<ObjectID>,
    ) {
        self.commands.tree.push(TransformNodeTreeCommand::Remove(node));
        let parent = match parent {
            Some(parent) => parent,
            None => scene,
        };
        self.commands.tree.push(TransformNodeTreeCommand::Append(node, parent));
    }

    pub fn destroy_transform_node(
        &mut self,
        node: ObjectID,
    ) {
        self.commands.tree.push(TransformNodeTreeCommand::Destroy(node));
    }

    pub fn new_free_camera(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID {
        let entity = unsafe { ObjectID::new(self.world.archetypes_mut()[self.node_archetype_id].reserve_entity()) };
        self.commands.new_objects.push(ObjectNewCommand::NewTransformNode(entity));
        self.commands.new_objects.push(ObjectNewCommand::NewFreeCamera(entity));
        self.commands.tree.push(TransformNodeTreeCommand::Append(entity, scene));
        entity
    }

    pub fn tick_run(
        &mut self,
    ) {
        let node_archetype_id = self.node_archetype_id;
        self.world.archetypes_mut()[node_archetype_id].flush();
        let commands = replace(&mut self.commands, UserCommands::default());
        self.world.insert_resource(commands);
    }
}
