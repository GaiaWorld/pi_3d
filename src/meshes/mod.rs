use crate::{object::ObjectID, geometry::GeometryMeta};

pub struct Mesh {
    geometrys: Vec<GeometryMeta>,
    materials: Vec<ObjectID>,
}