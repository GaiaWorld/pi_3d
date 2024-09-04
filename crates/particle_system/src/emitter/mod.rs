
use pi_scene_shell::prelude::*;

use crate::tools::Random;

pub use self::{box_shape_emitter::*, circle_shape_emitter::*, cone_shape_emitter::*, edge_shape_emitter::*, hemisphere_shape_emitter::*, point_shape_emitter::*, rectangle_shape_emitter::*, sphere_shape_emitter::*};
pub use ishape_emitter_type::*;

mod box_shape_emitter;
mod ishape_emitter_type;
mod circle_shape_emitter;
mod cone_shape_emitter;
mod edge_shape_emitter;
mod hemisphere_shape_emitter;
mod point_shape_emitter;
mod rectangle_shape_emitter;
mod sphere_shape_emitter;

pub struct SerializationObject {
    pub _type: Option<String> ,
    pub radius: Option<Number>,
    pub angle: Option<Number>,
    pub direction_randomizer: Option<Number>,
    pub radius_range: Option<Number>,
    pub height_range: Option<Number>,
    pub emit_from_spawn_point_only: Option<bool>,
    pub size: Option<Number>,
    pub direction1: Option<Vector3>,
    pub direction2: Option<Vector3>
}

#[derive(Component)]
pub enum ShapeEmitter {
    Box(BoxShapeEmitter),
    Circle(CircleShapeEmitter),
    Cone(ConeShapeEmitter),
    Edge(EdgeShapeEmitter),
    Hemisphere(HemisphereShapeEmitter),
    Point(PointShapeEmitter),
    Rectangle(RectangleShapeEmitter),
    Sphere(SphereShapeEmitter),
}
impl Default for ShapeEmitter {
    fn default() -> Self {
        Self::Point(PointShapeEmitter::default())
    }
}