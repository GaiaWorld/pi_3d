use crate::object::ObjectID;

#[derive(Debug, Default)]
pub struct UserCommands {
    pub tree: Vec<TransformNodeCommand>,
}

#[derive(Debug)]
pub enum TransformNodeCommand {
    Append(ObjectID, ObjectID),
    Remove(ObjectID),
    Destroy(ObjectID),
}

