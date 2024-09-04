use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolVector3}, Matrix, Vector3};

use crate::tools::{normalize, Random};

use super::{
    ishape_emitter_type::*,
    SerializationObject,
};

pub struct RectangleShapeEmitter {
    direction: Vector3,
    pub(crate)  base: ShapeEmitter
}

impl RectangleShapeEmitter {

    const IDX_DIRECT_X: usize = 0;
    const IDX_DIRECT_Y: usize = 1;
    const IDX_DIRECT_Z: usize = 2;

    pub fn create(shape: &crate::iparticle_system_config::IShapeRectangle) -> TypeShapeEmitter {
        
        let mut _pos = Vector3::zeros();
        let mut _rotation = Vector3::new(0., 0., 0.);
        let mut _scale = Vector3::new(1., 1., 1.);
        let mut _randomize = None;
        let mut _align_dir = 0;
        // let mut box_mode = EBoxShapeMode::Volume;

        let mut result = TypeShapeEmitter::new();
        {
            if let Some(val) = &shape.position { _pos.copy_from_slice(val); };
            if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); };
            if let Some(val) = &shape.scale { _scale.copy_from_slice(val); };
            _randomize = shape.randomize.clone();
            _align_dir = shape.align_dir;
    
            result.base.align_direction = _align_dir != 0;
            if let Some(randomize) = &_randomize {
                result.base.randomize_direction = randomize[0];
                result.base.spherize_direction = randomize[1];
                result.base.randomize_position = randomize[2];
            }
        }

        result.param.push(0.);
        result.param.push(0.);
        result.param.push(1.);

        CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut result.base.local_matrix);

        result.fn_direction = Self::start_direction_function;
        result.fn_position = Self::start_position_function;
        result.fn_orbit_center = Self::orbit_center;
        result
    }

    pub fn serialize() {
        // todo!()
    }
    pub fn parse(_arg: SerializationObject) {
        // todo!()
    }
    pub fn new() -> Self {
        Self {
            direction: Vector3::new(0.0, 0.0, 1.0),
            base: ShapeEmitter::new(),
        }
    }
    
    pub fn start_direction_function<'a>(
        shape: &'a TypeShapeEmitter,
        direction_to_update: &'a mut Vector3,
        local_position: &'a Vector3,
        random: &'a mut Random,
        temp: &'a mut Vector3,
    ) {
        direction_to_update.x = shape.param[Self::IDX_DIRECT_X];
        direction_to_update.y = shape.param[Self::IDX_DIRECT_Y];
        direction_to_update.z = shape.param[Self::IDX_DIRECT_Z];

        normalize(local_position, temp);

        let t = shape.base.spherize_direction;
        direction_to_update.axpy(t, &temp, 1.0 - t);
       normalize(&direction_to_update, temp);

        temp.x += random.random() * shape.base.randomize_direction;
        temp.y += random.random() * shape.base.randomize_direction;
        temp.z += random.random() * shape.base.randomize_direction;

        normalize(&temp, direction_to_update);
    }

    pub fn start_position_function<'a>(
        shape: &'a TypeShapeEmitter,
        position_to_update: &'a mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        random: &'a mut Random,
        temp: &'a mut Vector3,
    ) {
        let mut rand_x: f32 = random.random_range(-0.5, 0.5);
        let mut rand_y: f32 = random.random_range(-0.5, 0.5);
        let mut rand_z: f32 = 0.;

        rand_x += (random.random() * 2.0 - 1.0) * shape.base.randomize_position;
        rand_z += (random.random() * 2.0 - 1.0) * shape.base.randomize_position;
        rand_y += (random.random() * 2.0 - 1.0) * shape.base.randomize_position;

        temp.x = rand_x; temp.y = rand_y; temp.z = rand_z;
        CoordinateSytem3::transform_coordinates(&temp, &shape.base.local_matrix, position_to_update);
    }
    pub fn orbit_center<'a>(_local_position: &'a Vector3, offset: &'a Vector3, result: &'a mut Vector3) {
        result.copy_from(offset);
    }

}

