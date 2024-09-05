
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, Quaternion};



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