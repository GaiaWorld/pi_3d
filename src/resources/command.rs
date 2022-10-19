use crate::object::ObjectID;

#[derive(Debug, Default)]
pub struct UserCommands {
    pub tree: Vec<TransformNodeTreeCommand>,
    pub new_objects: Vec<ObjectNewCommand>,
}

#[derive(Debug)]
pub enum TransformNodeTreeCommand {
    Append(ObjectID, ObjectID),
    Remove(ObjectID),
    Destroy(ObjectID),
}

#[derive(Debug)]
pub enum ObjectNewCommand {
    NewScene(ObjectID),
    NewTransformNode(ObjectID),
    NewMesh(ObjectID),
    NewFreeCamera(ObjectID),
}