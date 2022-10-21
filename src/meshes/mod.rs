use crate::{object::ObjectID};

pub struct Mesh {
    materials: Vec<ObjectID>,
}
impl Default for Mesh {
    fn default() -> Self {
        Self {
            materials: vec![],
        }
    }
}