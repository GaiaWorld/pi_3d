use pi_scene_context::pass::CoordinateSytem3;
use pi_scene_math::{vector::TToolVector3, Matrix, Vector3};


use crate::tools::Random;

use super::{
    ishape_emitter_type::*,
    SerializationObject,
};

/**
 * 点发射器
 */
#[derive(Default)]
pub struct PointShapeEmitter {
    _max_z: f32,
    /**
     * 创建模式
     */
    _direction_mode: EShapeEmitterDirectionMode,
    /**
     * 第一发射方向
     */
    direction1: Vector3,
    /**
     * 第二发射方向
     */
    direction2: Vector3,
    pub(crate)  base: ShapeEmitter,
}

impl PointShapeEmitter {
    pub fn create() -> TypeShapeEmitter {

        let mut result = TypeShapeEmitter::new();

        result.fn_direction = Self::start_direction_function;
        result.fn_position = Self::start_position_function;
        result.fn_orbit_center = Self::orbit_center;
        result
    }

    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> SerializationObject {
        SerializationObject {
            _type: Some(String::from("PointShapeEmitter")),
            radius: None,
            angle: None,
            direction_randomizer: None,
            radius_range: None,
            height_range: None,
            emit_from_spawn_point_only: None,
            size: None,
            direction1: Some(self.direction1),
            direction2: Some(self.direction2),
            
        }
    }

    /**
     * Parse properties from a JSON object
     * @param serializationObject defines the JSON object
     */
    pub fn parse(&mut self, arg: SerializationObject) {
        self.direction1 = arg.direction1.as_ref().unwrap().clone();
        self.direction2 = arg.direction2.as_ref().unwrap().clone();
    }

    pub fn new() -> Self{
        Self{
            _max_z: 999999999.,
            _direction_mode: EShapeEmitterDirectionMode::Unity,
            direction1: Vector3::new(0.,0.,1.),
            direction2: Vector3::new(0.,0.,1.),
            base: ShapeEmitter::new()
        }
    }
    pub fn start_direction_function<'a>(
        _shape: &'a TypeShapeEmitter,
        direction_to_update: &'a mut Vector3,
        _local_position: &'a Vector3,
        _random: &'a mut Random,
        _temp: &'a mut Vector3,
    ) {
        direction_to_update.x = 0.;
        direction_to_update.y = 0.;
        direction_to_update.z = 1.;
    }

    pub fn start_position_function<'a>(
        _shape: &'a TypeShapeEmitter,
        position_to_update: &'a mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        _random: &'a mut Random,
        temp: &'a mut Vector3,
    ) {
        temp.x = 0.; temp.y = 0.; temp.z = 0.;
        CoordinateSytem3::transform_coordinates_floats(0., 0., 0., &_shape.base.local_matrix, position_to_update);
    }
    
    pub fn orbit_center<'a>(_local_position: &'a Vector3, offset: &'a Vector3, result: &'a mut Vector3) {
        result.copy_from(offset);
    }

}
