use pi_ecs::prelude::{ResMut, Query, Setup};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTreeMut;
use pi_scene_math::Vector3;

use crate::{object::{ObjectID, GameObject}, plugin::Plugin, scene::{InterfaceScene}};

use self::{transform_node_command::{SysTransformNodeCommand, SingleTransformNodeCommandList, TransformNodeCommand}, transform_node_sys::{LocalRotationMatrixCalc, LocalMatrixCalc, WorldMatrixCalc}};

pub mod transform_node;
pub mod transform_node_sys;
pub mod transform_node_command;

pub enum TreeCommand {
    Append(ObjectID, ObjectID),
    Remove(ObjectID),
}

pub struct SingleTreeCommandList {
    pub list: Vec<TreeCommand>,
}
pub struct SysTreeCommand;
#[setup]
impl SysTreeCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleTreeCommandList>,
        entitys: Query<GameObject, ObjectID>,
        mut tree: EntityTreeMut<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TreeCommand::Append(child, parent) => {
                    if entitys.get(child).is_some() {
                        tree.insert_child(child, parent, usize::MAX);
                    }
                },
                TreeCommand::Remove(child) => {
                    tree.remove(child);
                },
            }
        });
    }
}

pub trait InterfaceTransformNode {
    fn create_transform_node(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_transform_node(
        &mut self,
        object: ObjectID,
    ) -> &mut Self;

    fn transform_position(
        &mut self,
        node: ObjectID,
        position: Vector3
    ) -> &mut Self;

    fn transform_parent(
        &mut self,
        node: ObjectID,
        parent: ObjectID,
    ) -> &mut Self;
}
impl InterfaceTransformNode for crate::engine::Engine {
    fn create_transform_node(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.new_object();
        // let world = self.world_mut();

        self.add_to_scene(entity, scene);

        self.as_transform_node(entity);

        self.transform_parent(entity, scene);

        entity
    }

    fn as_transform_node(
        &mut self,
        object: ObjectID,
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleTransformNodeCommandList>().unwrap();
        commands.list.push(TransformNodeCommand::Create(object));
        
        self
    }

    fn transform_position(
        &mut self,
        node: ObjectID,
        position: Vector3
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleTransformNodeCommandList>().unwrap();
        commands.list.push(TransformNodeCommand::ModifyPosition(node, position));

        self
    }

    fn transform_parent(
        &mut self,
        node: ObjectID,
        parent: ObjectID,
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleTreeCommandList>().unwrap();
        commands.list.push(TreeCommand::Append(node, parent));

        self
    }

}


pub struct PluginTransformNode;
impl Plugin for PluginTransformNode {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysTreeCommand::setup(world, stages.command_stage());
        SysTransformNodeCommand::setup(world, stages.command_stage());
        LocalRotationMatrixCalc::setup(world, stages.local_matrix_stage());
        LocalMatrixCalc::setup(world, stages.local_matrix_stage());
        WorldMatrixCalc::setup(world, stages.world_matrix());

        world.insert_resource(SingleTreeCommandList{ list: vec![] });
        world.insert_resource(SingleTransformNodeCommandList{ list: vec![] });

        Ok(())
    }
}