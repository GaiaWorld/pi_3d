use pi_engine_shell::object::InterfaceObject;
use pi_scene_math::{Number, Vector3};

use crate::{object::{ObjectID}, transforms::{interface::InterfaceTransformNode}, scene::interface::InterfaceScene, layer_mask::{command::{LayerMaskCommand, SingleLayerMaskCommandList}, LayerMask}};

use super::command::{SingleCameraCommandList, SingleTargetCameraCommandList, CameraCommand, TargetCameraCommand, SingleFreeCameraCommandList, FreeCameraCommand, SingleCameraCreateList, ECameraCreateCommand};

pub trait InterfaceCamera {
    fn create_free_camera(
        &self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_free_camera(
        & self,
        object: ObjectID,
    ) -> & Self;

    fn free_camera_orth_size(
        & self,
        object: ObjectID,
        size: Number,
    ) -> & Self;

    fn camera_target(
        & self,
        object: ObjectID,
        value: Vector3,
    ) -> &Self;
}

impl InterfaceCamera for crate::engine::Engine {
    fn create_free_camera(
        & self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.new_object();
        let world = self.world();

        self.add_to_scene(entity, scene);
        self.as_transform_node(entity);
        self.transform_parent(entity, scene);

        self.as_free_camera(entity);

        entity
    }

    fn as_free_camera(
        & self,
        object: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleCameraCreateList>().unwrap();
        commands.list.push(ECameraCreateCommand::FreeCamera(object));

        self
    }
    
    fn free_camera_orth_size(
        & self,
        object: ObjectID,
        size: Number,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleCameraCommandList>().unwrap();
        commands.list.push(CameraCommand::ModifyOrthSize(object, size));

        self
    }

    fn camera_target(
        & self,
        object: ObjectID,
        value: Vector3,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleTargetCameraCommandList>().unwrap();
        commands.list.push(TargetCameraCommand::Target(object, value));

        self
    }
}
