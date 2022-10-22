use pi_scene_math::{Quaternion, Vector3, Number};

use crate::{object::ObjectID, cameras::free_camera::{EFreeCameraMode, EFovMode}, renderers::render_object::RenderObjectID};

#[derive(Debug, Default)]
pub struct UserCommands {
    pub tree: Vec<TransformNodeTreeCommand>,
    pub transform_nodes: Vec<TransformNodeCommand>,
    pub objects: Vec<ObjectCommand>,
    pub cameras: Vec<CameraCommand>,
    pub free_cameras: Vec<FreeCameraCommand>,
    pub mesh_builder: Vec<MeshBuilderCommand>,
}

#[derive(Debug)]
pub enum TransformNodeTreeCommand {
    Append(ObjectID, ObjectID),
    Remove(ObjectID),
}

#[derive(Debug)]
pub enum TransformNodeCommand {
    ModifyPosition(ObjectID, Vector3),
    ModifyRotation(ObjectID, Vector3),
    ModifyScaling(ObjectID, Vector3),
    ModifyRotationQuaternion(ObjectID, Quaternion),
    ModifyTarget(ObjectID, Vector3),
}

#[derive(Debug)]
pub enum ObjectCommand {
    NewScene(ObjectID),
    NewTransformNode(ObjectID, ObjectID),
    NewMesh(ObjectID, ObjectID),
    NewFreeCamera(ObjectID, ObjectID),
    Destroy(ObjectID),
}

#[derive(Debug)]
pub enum CameraCommand {
    ActiveRender(ObjectID, ObjectID),
    DisableRender(ObjectID),
}

#[derive(Debug)]
pub enum FreeCameraCommand {
    ModifyMode(ObjectID, EFreeCameraMode),
    ModifyFov(ObjectID, Number),
    ModifyFovMode(ObjectID, EFovMode),
    ModifyOrthSize(ObjectID, Number),
}

#[derive(Debug)]
pub enum MeshBuilderCommand {
    Cube(ObjectID),
    Plane(ObjectID),
}