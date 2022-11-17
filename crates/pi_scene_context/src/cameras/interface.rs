use pi_engine_shell::object::InterfaceObject;
use pi_scene_math::Number;

use crate::{object::{ObjectID}, transforms::{interface::InterfaceTransformNode}, scene::interface::InterfaceScene};

use super::command::{SingleCameraCommandList, SingleTargetCameraCommandList, CameraCommand, TargetCameraCommand, SingleFreeCameraCommandList, FreeCameraCommand};

pub trait InterfaceCamera {
    fn create_free_camera(
        &self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_camera(
        &self,
        object: ObjectID,
    ) -> &Self;

    fn as_target_camera(
        & self,
        object: ObjectID,
    ) -> &Self;

    fn as_free_camera(
        & self,
        object: ObjectID,
    ) -> & Self;

    fn free_camera_orth_size(
        & self,
        object: ObjectID,
        size: Number,
    ) -> & Self;
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

        self.as_camera(entity);
        self.as_target_camera(entity);
        self.as_free_camera(entity);

        entity
    }

    fn as_camera(
        & self,
        object: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleCameraCommandList>().unwrap();
        commands.list.push(CameraCommand::Create(object));

        self
    }

    fn as_target_camera(
        & self,
        object: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleTargetCameraCommandList>().unwrap();
        commands.list.push(TargetCameraCommand::Create(object));

        self
    }

    fn as_free_camera(
        & self,
        object: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleFreeCameraCommandList>().unwrap();
        commands.list.push(FreeCameraCommand::Create(object));

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
}
