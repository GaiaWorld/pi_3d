use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolVector3}, Matrix, Vector3};

use crate::tools::{normalize, Random};

use super::{ishape_emitter_type::{EBoxShapeMode, IShapeEmitterType, ShapeEmitter}, TypeShapeEmitter};

pub struct BoxShapeEmitter {
    pub emit_mode: EBoxShapeMode,
    pub(crate) direction: Vector3,
    pub(crate) base: ShapeEmitter,
}

impl BoxShapeEmitter {

    const IDX_DIRECT_X: usize = 0;
    const IDX_DIRECT_Y: usize = 1;
    const IDX_DIRECT_Z: usize = 2;

    pub fn new(shape: &crate::iparticle_system_config::IShapeBox) -> TypeShapeEmitter {
        
        let mut _pos = Vector3::zeros();
        let mut _rotation = Vector3::new(0., 0., 0.);
        let mut _scale = Vector3::new(1., 1., 1.);
        let mut _randomize = None;
        let mut _align_dir = 0;
        let mut box_mode = EBoxShapeMode::Volume;

        let mut result = TypeShapeEmitter::new();
        {
            if let Some(val) = &shape.position { _pos.copy_from_slice(val); } else { _pos.copy_from_slice(&[0., 0., 0.]) };
            if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); } else { _rotation.copy_from_slice(&[0., 0., 0.]) };
            if let Some(val) = &shape.scale { _scale.copy_from_slice(val); } else { _scale.copy_from_slice(&[1., 1., 1.]) };
            _randomize = shape.randomize.clone();
            _align_dir = shape.align_dir;
    
            result.base.align_direction = _align_dir != 0;
            if let Some(randomize) = &_randomize {
                result.base.randomize_direction = randomize[0];
                result.base.spherize_direction = randomize[1];
                result.base.randomize_position = randomize[2];
            }
        }

        if let Some(mode) = &shape.box_emit_mode { box_mode = *mode; };
        result.box_mode = box_mode;
        {
            result.param.push(0.); result.param.push(0.); result.param.push(1.);
        }

        CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut result.base.local_matrix);

        result.fn_direction = Self::start_direction_function;
        result.fn_position = Self::start_position_function;
        result.fn_orbit_center = Self::orbit_center;
        result
    }
    pub fn start_direction_function<'a>(
        shape: &'a TypeShapeEmitter,
        direction_to_update: &'a mut Vector3,
        _local_position: &'a Vector3,
        random: &'a mut Random,
        temp: &'a mut Vector3,
    ) {
        direction_to_update.x = shape.param[Self::IDX_DIRECT_X];
        direction_to_update.y = shape.param[Self::IDX_DIRECT_Y];
        direction_to_update.z = shape.param[Self::IDX_DIRECT_Z];

        normalize(_local_position, temp);
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
        let mut rand_z: f32 = random.random_range(-0.5, 0.5);

        match shape.box_mode {
            EBoxShapeMode::Volume => {
                // randX = Scalar.RandomRange(-0.5, 0.5);
                // randY = Scalar.RandomRange(-0.5, 0.5);
                // randZ = Scalar.RandomRange(-0.5, 0.5);
                // break;
            }
            EBoxShapeMode::Shell => {
                let face_range: f32 = random.random();
                // YZ
                if face_range < 0.3333f32 {
                    rand_x = if face_range < 0.1666 { 0.5 } else { -0.5 };
                // XZ
                } else if face_range < 0.6666 {
                    rand_y = if face_range < 0.5 { 0.5 } else { -0.5 };
                // XY
                } else {
                    rand_z = if face_range < 0.8333 { 0.5 } else { -0.5 };
                }
            }
            EBoxShapeMode::Edge => {
                let face_range: f32 = random.random();
                let face_range2: f32 = random.random();
                let face_range3: f32 = random.random();
                // X
                if face_range < 0.3333 {
                    rand_y = if face_range2 < 0.5 { 0.5 } else { -0.5 };
                    rand_z = if face_range3 < 0.5 { 0.5 } else { -0.5 };
                // Y
                } else if face_range < 0.6666 {
                    rand_x = if face_range2 < 0.5 { 0.5 } else { -0.5 };
                    rand_z = if face_range3 < 0.5 { 0.5 } else { -0.5 };
                // Z
                } else {
                    rand_x = if face_range2 < 0.5 { 0.5 } else { -0.5 };
                    rand_y = if face_range3 < 0.5 { 0.5 } else { -0.5 };
                }
            }
        }

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
