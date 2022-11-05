use pi_ecs::{prelude::{Setup, }};
use pi_scene_math::Number;

use crate::{plugin::Plugin, object::{ObjectID}, transforms::InterfaceTransformNode, scene::InterfaceScene};

use self::{camera::{SysCameraCommand, SingleCameraCommandList, CameraCommand}, camera_sys::{TargetCameraViewMatrixCalc, SysCameraTransformMatrix, SysCameraProjectionCalc}, free_camera::{SysFreeCameraCommand, SingleFreeCameraCommandList, FreeCameraCommand}, target_camera::{SysTargetCameraCommand, SingleTargetCameraCommandList, TargetCameraCommand}};

pub mod camera;
pub mod free_camera;
pub mod arc_rotate_camera;
pub mod target_camera;
pub mod camera_sys;

pub struct PluginCamera;
impl Plugin for PluginCamera {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysCameraCommand::setup(world, stages.command_stage());
        SysTargetCameraCommand::setup(world, stages.command_stage());
        SysFreeCameraCommand::setup(world, stages.command_stage());
        SysCameraProjectionCalc::setup(world, stages.after_world_matrix());
        TargetCameraViewMatrixCalc::setup(world, stages.after_world_matrix());
        SysCameraTransformMatrix::setup(world, stages.after_world_matrix());

        world.insert_resource(SingleCameraCommandList::default());
        world.insert_resource(SingleTargetCameraCommandList::default());
        world.insert_resource(SingleFreeCameraCommandList::default());

        Ok(())
    }
}

pub trait InterfaceCamera {
    fn create_free_camera(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_camera(
        &mut self,
        object: ObjectID,
    ) -> &mut Self;

    fn as_target_camera(
        &mut self,
        object: ObjectID,
    ) -> &mut Self;

    fn as_free_camera(
        &mut self,
        object: ObjectID,
    ) -> &mut Self;

    fn free_camera_orth_size(
        &mut self,
        object: ObjectID,
        size: Number,
    ) -> &mut Self;
}

impl InterfaceCamera for crate::engine::Engine {
    fn create_free_camera(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.new_object();
        let world = self.world_mut();

        self.add_to_scene(entity, scene);
        self.as_transform_node(entity);
        self.transform_parent(entity, scene);

        self.as_camera(entity);
        self.as_target_camera(entity);
        self.as_free_camera(entity);

        entity
    }

    fn as_camera(
        &mut self,
        object: ObjectID,
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleCameraCommandList>().unwrap();
        commands.list.push(CameraCommand::Create(object));

        self
    }

    fn as_target_camera(
        &mut self,
        object: ObjectID,
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleTargetCameraCommandList>().unwrap();
        commands.list.push(TargetCameraCommand::Create(object));

        self
    }

    fn as_free_camera(
        &mut self,
        object: ObjectID,
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleFreeCameraCommandList>().unwrap();
        commands.list.push(FreeCameraCommand::Create(object));

        self
    }
    
    fn free_camera_orth_size(
        &mut self,
        object: ObjectID,
        size: Number,
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleCameraCommandList>().unwrap();
        commands.list.push(CameraCommand::ModifyOrthSize(object, size));

        self
    }
}