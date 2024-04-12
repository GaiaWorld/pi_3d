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
    ) {
        let mut direction = Vector3::new(shape.param[Self::IDX_DIRECT_X], shape.param[Self::IDX_DIRECT_Y], shape.param[Self::IDX_DIRECT_Z]);

        direction[0] =
            direction[0] * (1.0 - shape.base.spherize_direction) + direction[0] * shape.base.spherize_direction;
        direction[1] =
            direction[1] * (1.0 - shape.base.spherize_direction) + direction[1] * shape.base.spherize_direction;
        direction[2] =
            direction[2] * (1.0 - shape.base.spherize_direction) + direction[2] * shape.base.spherize_direction;

        direction = normalize(&direction);
        direction[0] += random.random() * shape.base.randomize_direction;
        direction[1] += random.random() * shape.base.randomize_direction;
        direction[2] += random.random() * shape.base.randomize_direction;

        direction = normalize(&direction);
        
            *direction_to_update = direction;
            
        *direction_to_update = normalize(direction_to_update);
    }
    
    pub fn start_position_function<'a>(
        shape: &'a TypeShapeEmitter,
        position_to_update: &'a mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        random: &'a mut Random,
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

        CoordinateSytem3::transform_coordinates(&Vector3::new(rand_x, rand_y, rand_z), &shape.base.local_matrix, position_to_update);
    }
    pub fn orbit_center<'a>(_local_position: &'a Vector3, offset: &'a Vector3, result: &'a mut Vector3) {
        result.copy_from(offset);
    }
}

impl IShapeEmitterType for BoxShapeEmitter {
    fn get_class_name() -> String {
        todo!()
    }

    fn dispose() {
        todo!()
    }

    fn start_direction_function(
        &self,
        direction_to_update: &mut Vector3,
        _local_position: &Vector3,
        random: &mut Random,
    ) {
        let mut direction = self.direction;

        direction[0] =
            direction[0] * (1.0 - self.base.spherize_direction) + direction[0] * self.base.spherize_direction;
        direction[1] =
            direction[1] * (1.0 - self.base.spherize_direction) + direction[1] * self.base.spherize_direction;
        direction[2] =
            direction[2] * (1.0 - self.base.spherize_direction) + direction[2] * self.base.spherize_direction;

        direction = normalize(&direction);
        direction[0] += random.random() * self.base.randomize_direction;
        direction[1] += random.random() * self.base.randomize_direction;
        direction[2] += random.random() * self.base.randomize_direction;

        direction = normalize(&direction);
        
            *direction_to_update = direction;
            
        *direction_to_update = normalize(direction_to_update);
    }

    fn start_position_function(
        &self,
        position_to_update: &mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        random: &mut Random,
    ) {
        let mut rand_x: f32 = random.random_range(-0.5, 0.5);
        let mut rand_y: f32 = random.random_range(-0.5, 0.5);
        let mut rand_z: f32 = random.random_range(-0.5, 0.5);

        match self.emit_mode {
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

        rand_x += (random.random() * 2.0 - 1.0) * self.base.randomize_position;
        rand_z += (random.random() * 2.0 - 1.0) * self.base.randomize_position;
        rand_y += (random.random() * 2.0 - 1.0) * self.base.randomize_position;

        CoordinateSytem3::transform_coordinates(&Vector3::new(rand_x, rand_y, rand_z), &self.base.local_matrix, position_to_update);
    }

    fn set_position(&mut self, position: Vector3) {
        self.base.position = position;
    }

    fn set_rotation(&mut self, rotation: Vector3) {
        self.base.rotation = rotation;
    }

    fn set_scaling(&mut self, scaling: Vector3) {
        self.base.scaling = scaling;
    }

    fn get_postion(&self) -> Vector3 {
        self.base.position.clone()
    }

    fn get_rotation(&self) -> Vector3 {
        self.base.rotation.clone()
    }

    fn get_scaling(&self) -> Vector3 {
        self.base.scaling.clone()
    }

    fn set_local_matrix(&mut self, local_matrix: Matrix) {
        self.base.local_matrix = local_matrix;
    }

    fn set_align_direction(&mut self, align_direction: bool) {
        self.base.align_direction = align_direction;
    }

    fn set_randomize_direction(&mut self, randomize_direction: f32) {
        self.base.randomize_direction = randomize_direction;
    }

    fn set_spherize_direction(&mut self, spherize_direction: f32) {
        self.base.spherize_direction = spherize_direction;
    }

    fn set_randomize_position(&mut self, randomize_position: f32) {
        self.base.randomize_position = randomize_position;
    }

    fn get_local_matrix(&self) -> Matrix {
        self.base.local_matrix.clone()
    }

    fn get_align_direction(&self) -> bool {
        self.base.align_direction.clone()
    }

    fn get_randomize_direction(&self) -> f32 {
        self.base.randomize_direction.clone()
    }

    fn get_spherize_direction(&self) -> f32 {
        self.base.spherize_direction.clone()
    }

    fn get_randomize_position(&self) -> f32 {
        self.base.randomize_position.clone()
    }
}
