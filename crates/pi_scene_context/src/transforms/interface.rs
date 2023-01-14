
use pi_engine_shell::object::InterfaceObject;
use pi_scene_math::{Vector3, Quaternion};

use crate::{object::{ObjectID}, scene::{interface::InterfaceScene}, layer_mask::{command::{SingleLayerMaskCommandList, LayerMaskCommand}, LayerMask}};

use super::{command::{SingleTreeCommandList, TreeCommand, TransformNodeCommand, SingleTransformNodeCommandList}};

pub trait InterfaceTransformNode {
    fn create_transform_node(
        & self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_transform_node(
        & self,
        object: ObjectID,
    ) -> & Self;

    fn transform_position(
        & self,
        node: ObjectID,
        position: Vector3
    ) -> & Self;

    fn transform_parent(
        & self,
        node: ObjectID,
        parent: ObjectID,
    ) -> & Self;

    fn transform_scaling(
        & self,
        node: ObjectID,
        scaling: Vector3
    ) -> & Self;

    fn transform_rotation_euler(
        & self,
        node: ObjectID,
        euler_angle: Vector3
    ) -> & Self;

    fn transform_rotation_quaternion(
        & self,
        node: ObjectID,
        quat: Quaternion
    ) -> & Self;
}
impl InterfaceTransformNode for crate::engine::Engine {
    fn create_transform_node(
        & self,
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
        & self,
        object: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleTransformNodeCommandList>().unwrap();
        commands.list.push(TransformNodeCommand::Create(object));

        let commands = world.get_resource_mut::<SingleLayerMaskCommandList>().unwrap();
        commands.list.push(LayerMaskCommand::Set(object, LayerMask::default()));
        
        self
    }

    fn transform_position(
        & self,
        node: ObjectID,
        position: Vector3
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleTransformNodeCommandList>().unwrap();
        commands.list.push(TransformNodeCommand::ModifyPosition(node, position));

        self
    }

    fn transform_rotation_euler(
        & self,
        node: ObjectID,
        euler_angle: Vector3
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleTransformNodeCommandList>().unwrap();
        commands.list.push(TransformNodeCommand::ModifyRotation(node, euler_angle));

        self
    }

    fn transform_parent(
        & self,
        node: ObjectID,
        parent: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleTreeCommandList>().unwrap();
        commands.list.push(TreeCommand::Remove(node));
        commands.list.push(TreeCommand::Append(node, parent));

        self
    }

    fn transform_rotation_quaternion(
        & self,
        node: ObjectID,
        quat: Quaternion
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleTransformNodeCommandList>().unwrap();
        commands.list.push(TransformNodeCommand::ModifyRotationQuaternion(node, quat));

        self
    }

    fn transform_scaling(
        & self,
        node: ObjectID,
        scaling: Vector3
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleTransformNodeCommandList>().unwrap();
        commands.list.push(TransformNodeCommand::ModifyScaling(node, scaling));

        self
    }
}

