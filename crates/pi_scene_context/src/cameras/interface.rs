
use pi_engine_shell::prelude::*;
use pi_scene_math::{Number, Vector3};

use crate::{object::{ObjectID}, transforms::{interface::InterfaceTransformNode}, scene::interface::InterfaceScene, renderers::graphic::RendererGraphicDesc};

use super::{command::{SingleCameraCommandList, SingleTargetCameraCommandList, ECameraCommand, TargetCameraCommand, SingleCameraCreateList, ECameraCreateCommand}, camera::{EFreeCameraMode, EFixedMode}};

pub trait InterfaceCamera {
    fn create_free_camera(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID;
    
    
    fn free_camera_mode(
        &self,
        object: ObjectID,
        mode: EFreeCameraMode,
    ) -> & Self;

    fn camera_fixed_mode(
        &self,
        object: ObjectID,
        mode: EFixedMode,
    ) -> & Self;

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

    fn active_camera(
        & self,
        object: ObjectID,
        flag: bool,
    ) -> &Self;
    
    fn camera_renderer(
        &self,
        id_viewer: ObjectID,
        param: RendererGraphicDesc,
    ) -> &Self;
}

impl InterfaceCamera for EnginShell {
    fn create_free_camera(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.world.spawn_empty().id();

        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &self.world);

        commands.

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
    
    fn free_camera_mode(
        &self,
        object: ObjectID,
        mode: EFreeCameraMode,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleCameraCommandList>().unwrap();
        commands.list.push(ECameraCommand::ModifyMode(object, mode));

        self
    }
    
    
    fn camera_fixed_mode(
        &self,
        object: ObjectID,
        mode: EFixedMode,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleCameraCommandList>().unwrap();
        commands.list.push(ECameraCommand::ModifyFixedMode(object, mode));

        self
    }
    
    fn free_camera_orth_size(
        & self,
        object: ObjectID,
        size: Number,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleCameraCommandList>().unwrap();
        commands.list.push(ECameraCommand::ModifyOrthSize(object, size));

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

    fn active_camera(
        & self,
        object: ObjectID,
        flag: bool,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleCameraCreateList>().unwrap();
        commands.list.push(ECameraCreateCommand::Active(object, flag));

        self
    }
    
    fn camera_renderer(
        &self,
        id_viewer: ObjectID,
        param: RendererGraphicDesc,
    ) -> &Self {
        let world = self.world();

        let id_render = self.new_object();

        let cmds = world.get_resource_mut::<SingleCameraCommandList>().unwrap();
        cmds.list.push(ECameraCommand::Renderer(id_viewer, id_render, param));

        self
    }
}
