use crate::{object::ObjectID, geometry::GeometryMeta};

pub struct Mesh {
    geometrys: Vec<GeometryMeta>,
    materials: Vec<ObjectID>,
}
impl Default for Mesh {
    fn default() -> Self {
        Self {
            geometrys: vec![],
            materials: vec![],
        }
    }
}