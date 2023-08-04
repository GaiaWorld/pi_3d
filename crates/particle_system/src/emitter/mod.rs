use pi_engine_shell::prelude::*;
use pi_scene_math::*;

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

impl ShapeEmitter {
    pub fn start_position_function(
        &self,
        position_to_update: &mut Vector3,
        emission_loop: Number,
        emission_progress: Number,
        emission_index: Number,
        emission_total: Number,
        is_local: bool,
        random: &mut Random,
    ) {
        match self {
            ShapeEmitter::Box(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, is_local, random);
            },
            ShapeEmitter::Circle(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, is_local, random);
            },
            ShapeEmitter::Cone(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, is_local, random);
            },
            ShapeEmitter::Edge(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, is_local, random);
            },
            ShapeEmitter::Hemisphere(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, is_local, random);
            },
            ShapeEmitter::Point(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, is_local, random);
            },
            ShapeEmitter::Rectangle(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, is_local, random);
            },
            ShapeEmitter::Sphere(item) => {
                item.start_position_function(position_to_update, emission_loop, emission_progress, emission_index, emission_total, is_local, random);
            },
        }
    }
    
    pub fn start_direction_function(
        &self,
        direction_to_update: &mut Vector3,
        local_position: &Vector3,
        is_local: bool,
        random: &mut Random,
    ) {
        match self {
            ShapeEmitter::Box(item) => {
                item.start_direction_function(direction_to_update, local_position, is_local, random);
            },
            ShapeEmitter::Circle(item) => {
                item.start_direction_function(direction_to_update, local_position, is_local, random);
            },
            ShapeEmitter::Cone(item) => {
                item.start_direction_function(direction_to_update, local_position, is_local, random);
            },
            ShapeEmitter::Edge(item) => {
                item.start_direction_function(direction_to_update, local_position, is_local, random);
            },
            ShapeEmitter::Hemisphere(item) => {
                item.start_direction_function(direction_to_update, local_position, is_local, random);
            },
            ShapeEmitter::Point(item) => {
                item.start_direction_function(direction_to_update, local_position, is_local, random);
            },
            ShapeEmitter::Rectangle(item) => {
                item.start_direction_function(direction_to_update, local_position, is_local, random);
            },
            ShapeEmitter::Sphere(item) => {
                item.start_direction_function(direction_to_update, local_position, is_local, random);
            },
        }
    }

    pub fn orbit_center(&self, local_position: &Vector3, offset: &Vector3, result: &mut Vector3) {
        match self {
            ShapeEmitter::Box(_) => {
                *result = offset.clone();
            },
            ShapeEmitter::Circle(_) => {
                *result = offset.clone();
            },
            ShapeEmitter::Cone(_) => {
                *result = offset.clone();
                result.z += local_position.z;
            },
            ShapeEmitter::Edge(_) => {
                *result = offset.clone();
            },
            ShapeEmitter::Hemisphere(_) => {
                *result = offset.clone();
            },
            ShapeEmitter::Point(_) => {
                *result = offset.clone();
            },
            ShapeEmitter::Rectangle(_) => {
                *result = offset.clone();
            },
            ShapeEmitter::Sphere(_) => {
                *result = offset.clone();
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
            ShapeEmitter::Box(item) => { item.align_direction = val; },
            ShapeEmitter::Circle(item) => { item.align_direction = val; },
            ShapeEmitter::Cone(item) => { item.align_direction = val; },
            ShapeEmitter::Edge(item) => { item.align_direction = val; },
            ShapeEmitter::Hemisphere(item) => { item.align_direction = val; },
            ShapeEmitter::Point(item) => { item.align_direction = val; },
            ShapeEmitter::Rectangle(item) => { item.align_direction = val; },
            ShapeEmitter::Sphere(item) => { item.align_direction = val; },
        }
    }
    pub fn randomize_direction(&mut self, val: f32) {
        match self {
            ShapeEmitter::Box(item) => { item.randomize_direction = val; },
            ShapeEmitter::Circle(item) => { item.randomize_direction = val; },
            ShapeEmitter::Cone(item) => { item.randomize_direction = val; },
            ShapeEmitter::Edge(item) => { item.randomize_direction = val; },
            ShapeEmitter::Hemisphere(item) => { item.randomize_direction = val; },
            ShapeEmitter::Point(item) => { item.randomize_direction = val; },
            ShapeEmitter::Rectangle(item) => { item.randomize_direction = val; },
            ShapeEmitter::Sphere(item) => { item.randomize_direction = val; },
        }
    }
    pub fn spherize_direction(&mut self, val: f32) {
        match self {
            ShapeEmitter::Box(item) => { item.spherize_direction = val; },
            ShapeEmitter::Circle(item) => { item.spherize_direction = val; },
            ShapeEmitter::Cone(item) => { item.spherize_direction = val; },
            ShapeEmitter::Edge(item) => { item.spherize_direction = val; },
            ShapeEmitter::Hemisphere(item) => { item.spherize_direction = val; },
            ShapeEmitter::Point(item) => { item.spherize_direction = val; },
            ShapeEmitter::Rectangle(item) => { item.spherize_direction = val; },
            ShapeEmitter::Sphere(item) => { item.spherize_direction = val; },
        }
    }
    pub fn randomize_position(&mut self, val: f32) {
        match self {
            ShapeEmitter::Box(item) => { item.randomize_position = val; },
            ShapeEmitter::Circle(item) => { item.randomize_position = val; },
            ShapeEmitter::Cone(item) => { item.randomize_position = val; },
            ShapeEmitter::Edge(item) => { item.randomize_position = val; },
            ShapeEmitter::Hemisphere(item) => { item.randomize_position = val; },
            ShapeEmitter::Point(item) => { item.randomize_position = val; },
            ShapeEmitter::Rectangle(item) => { item.randomize_position = val; },
            ShapeEmitter::Sphere(item) => { item.randomize_position = val; },
        }
    }
}
