use std::default;

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

impl ShapeEmitter {
    pub fn start_position_function(
        &self,
        position_to_update: &mut Vector3,
        emission_loop: Number,
        emission_progress: Number,
        emission_index: Number,
        emission_total: Number,
        random: &mut Random,
    ) {
        match self {
            ShapeEmitter::Box(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, random);
            },
            ShapeEmitter::Circle(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, random);
            },
            ShapeEmitter::Cone(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, random);
            },
            ShapeEmitter::Edge(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, random);
            },
            ShapeEmitter::Hemisphere(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, random);
            },
            ShapeEmitter::Point(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, random);
            },
            ShapeEmitter::Rectangle(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, random);
            },
            ShapeEmitter::Sphere(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, random);
            },
        }
    }
    
    pub fn start_direction_function(
        &self,
        direction_to_update: &mut Vector3,
        local_position: &Vector3,
        random: &mut Random,
    ) {
        match self {
            ShapeEmitter::Box(item) => {
                item.start_direction_function(direction_to_update, local_position, random);
            },
            ShapeEmitter::Circle(item) => {
                item.start_direction_function(direction_to_update, local_position, random);
            },
            ShapeEmitter::Cone(item) => {
                item.start_direction_function(direction_to_update, local_position, random);
            },
            ShapeEmitter::Edge(item) => {
                item.start_direction_function(direction_to_update, local_position, random);
            },
            ShapeEmitter::Hemisphere(item) => {
                item.start_direction_function(direction_to_update, local_position, random);
            },
            ShapeEmitter::Point(item) => {
                item.start_direction_function(direction_to_update, local_position, random);
            },
            ShapeEmitter::Rectangle(item) => {
                item.start_direction_function(direction_to_update, local_position, random);
            },
            ShapeEmitter::Sphere(item) => {
                item.start_direction_function(direction_to_update, local_position, random);
            },
        }
    }

    pub fn orbit_center(&self, local_position: &Vector3, offset: &Vector3, result: &mut Vector3) {
        match self {
            ShapeEmitter::Box(_) => {
                result.copy_from(offset);
            },
            ShapeEmitter::Circle(_) => {
                result.copy_from(offset);
            },
            ShapeEmitter::Cone(_) => {
                result.copy_from(offset);
                result.z += local_position.z;
            },
            ShapeEmitter::Edge(_) => {
                result.copy_from(offset);
            },
            ShapeEmitter::Hemisphere(_) => {
                result.copy_from(offset);
            },
            ShapeEmitter::Point(_) => {
                result.copy_from(offset);
            },
            ShapeEmitter::Rectangle(_) => {
                result.copy_from(offset);
            },
            ShapeEmitter::Sphere(_) => {
                result.copy_from(offset);
            },
        }
    }

    pub fn position(&mut self, val: Vector3) {
        match self {
            ShapeEmitter::Box(item) => { item.set_position(val); },
            ShapeEmitter::Circle(item) => { item.set_position(val); },
            ShapeEmitter::Cone(item) => { item.set_position(val); },
            ShapeEmitter::Edge(item) => { item.set_position(val); },
            ShapeEmitter::Hemisphere(item) => { item.set_position(val); },
            ShapeEmitter::Point(item) => { item.set_position(val); },
            ShapeEmitter::Rectangle(item) => { item.set_position(val); },
            ShapeEmitter::Sphere(item) => { item.set_position(val); },
        }
    }
    pub fn rotation(&mut self, val: Vector3) {
        match self {
            ShapeEmitter::Box(item) => { item.set_rotation(val); },
            ShapeEmitter::Circle(item) => { item.set_rotation(val); },
            ShapeEmitter::Cone(item) => { item.set_rotation(val); },
            ShapeEmitter::Edge(item) => { item.set_rotation(val); },
            ShapeEmitter::Hemisphere(item) => { item.set_rotation(val); },
            ShapeEmitter::Point(item) => { item.set_rotation(val); },
            ShapeEmitter::Rectangle(item) => { item.set_rotation(val); },
            ShapeEmitter::Sphere(item) => { item.set_rotation(val); },
        }
    }
    pub fn scaling(&mut self, val: Vector3) {
        match self {
            ShapeEmitter::Box(item) => { item.set_scaling(val); },
            ShapeEmitter::Circle(item) => { item.set_scaling(val); },
            ShapeEmitter::Cone(item) => { item.set_scaling(val); },
            ShapeEmitter::Edge(item) => { item.set_scaling(val); },
            ShapeEmitter::Hemisphere(item) => { item.set_scaling(val); },
            ShapeEmitter::Point(item) => { item.set_scaling(val); },
            ShapeEmitter::Rectangle(item) => { item.set_scaling(val); },
            ShapeEmitter::Sphere(item) => { item.set_scaling(val); },
        }
    }
    pub fn align_direction(&mut self, val: bool) {
        match self {
            ShapeEmitter::Box(item) => { item.base.align_direction = val; },
            ShapeEmitter::Circle(item) => { item.base.align_direction = val; },
            ShapeEmitter::Cone(item) => { item.base.align_direction = val; },
            ShapeEmitter::Edge(item) => { item.base.align_direction = val; },
            ShapeEmitter::Hemisphere(item) => { item.base.align_direction = val; },
            ShapeEmitter::Point(item) => { item.base.align_direction = val; },
            ShapeEmitter::Rectangle(item) => { item.base.align_direction = val; },
            ShapeEmitter::Sphere(item) => { item.base.align_direction = val; },
        }
    }
    pub fn randomize_direction(&mut self, val: f32) {
        match self {
            ShapeEmitter::Box(item) => { item.base.randomize_direction = val; },
            ShapeEmitter::Circle(item) => { item.base.randomize_direction = val; },
            ShapeEmitter::Cone(item) => { item.base.randomize_direction = val; },
            ShapeEmitter::Edge(item) => { item.base.randomize_direction = val; },
            ShapeEmitter::Hemisphere(item) => { item.base.randomize_direction = val; },
            ShapeEmitter::Point(item) => { item.base.randomize_direction = val; },
            ShapeEmitter::Rectangle(item) => { item.base.randomize_direction = val; },
            ShapeEmitter::Sphere(item) => { item.base.randomize_direction = val; },
        }
    }
    pub fn spherize_direction(&mut self, val: f32) {
        match self {
            ShapeEmitter::Box(item) => { item.base.spherize_direction = val; },
            ShapeEmitter::Circle(item) => { item.base.spherize_direction = val; },
            ShapeEmitter::Cone(item) => { item.base.spherize_direction = val; },
            ShapeEmitter::Edge(item) => { item.base.spherize_direction = val; },
            ShapeEmitter::Hemisphere(item) => { item.base.spherize_direction = val; },
            ShapeEmitter::Point(item) => { item.base.spherize_direction = val; },
            ShapeEmitter::Rectangle(item) => { item.base.spherize_direction = val; },
            ShapeEmitter::Sphere(item) => { item.base.spherize_direction = val; },
        }
    }
    pub fn randomize_position(&mut self, val: f32) {
        match self {
            ShapeEmitter::Box(item) => { item.base.randomize_position = val; },
            ShapeEmitter::Circle(item) => { item.base.randomize_position = val; },
            ShapeEmitter::Cone(item) => { item.base.randomize_position = val; },
            ShapeEmitter::Edge(item) => { item.base.randomize_position = val; },
            ShapeEmitter::Hemisphere(item) => { item.base.randomize_position = val; },
            ShapeEmitter::Point(item) => { item.base.randomize_position = val; },
            ShapeEmitter::Rectangle(item) => { item.base.randomize_position = val; },
            ShapeEmitter::Sphere(item) => { item.base.randomize_position = val; },
        }
    }
}
