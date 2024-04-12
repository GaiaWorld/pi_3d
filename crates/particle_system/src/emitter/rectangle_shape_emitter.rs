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
    ) {
        let mut direction = Vector3::new(shape.param[Self::IDX_DIRECT_X], shape.param[Self::IDX_DIRECT_Y], shape.param[Self::IDX_DIRECT_Z]);

        let local_position = normalize(local_position);

        direction[0] = direction[0] * (1.0 - shape.base.spherize_direction)
            + local_position[0] * shape.base.spherize_direction;
        direction[1] = direction[1] * (1.0 - shape.base.spherize_direction)
            + local_position[1] * shape.base.spherize_direction;
        direction[2] = direction[2] * (1.0 - shape.base.spherize_direction)
            + local_position[2] * shape.base.spherize_direction;
        direction = normalize(&direction);

        direction[0] += random.random() * shape.base.randomize_direction;
        direction[1] += random.random() * shape.base.randomize_direction;
        direction[2] += random.random() * shape.base.randomize_direction;

        direction = normalize(&direction);

            *direction_to_update = direction;
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
        let mut rand_z: f32 = 0.;

        rand_x += (random.random() * 2.0 - 1.0) * shape.base.randomize_position;
        rand_z += (random.random() * 2.0 - 1.0) * shape.base.randomize_position;
        rand_y += (random.random() * 2.0 - 1.0) * shape.base.randomize_position;

        CoordinateSytem3::transform_coordinates(&Vector3::new(rand_x, rand_y, rand_z), &shape.base.local_matrix, position_to_update);
    }
    pub fn orbit_center<'a>(_local_position: &'a Vector3, offset: &'a Vector3, result: &'a mut Vector3) {
        result.copy_from(offset);
    }

}

impl IShapeEmitterType for RectangleShapeEmitter {
    fn start_direction_function(
        &self,
        direction_to_update: &mut Vector3,
        local_position: &Vector3,
        // is_local: bool,
        random: &mut Random,
    ) {
        let mut direction = self.direction;

        let local_position = normalize(local_position);

        direction[0] = direction[0] * (1.0 - self.base.spherize_direction)
            + local_position[0] * self.base.spherize_direction;
        direction[1] = direction[1] * (1.0 - self.base.spherize_direction)
            + local_position[1] * self.base.spherize_direction;
        direction[2] = direction[2] * (1.0 - self.base.spherize_direction)
            + local_position[2] * self.base.spherize_direction;
        direction = normalize(&direction);

        direction[0] += random.random() * self.base.randomize_direction;
        direction[1] += random.random() * self.base.randomize_direction;
        direction[2] += random.random() * self.base.randomize_direction;

        direction = normalize(&direction);

            *direction_to_update = direction;
    }

    fn start_position_function(
        &self,
        position_to_update: &mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        // is_local: bool,
        random: &mut Random,
    ) {
        let mut rand_x: f32 = random.random_range(-0.5, 0.5);
        let mut rand_y: f32 = random.random_range(-0.5, 0.5);
        let mut rand_z: f32 = 0.;

        rand_x += (random.random() * 2.0 - 1.0) * self.base.randomize_position;
        rand_z += (random.random() * 2.0 - 1.0) * self.base.randomize_position;
        rand_y += (random.random() * 2.0 - 1.0) * self.base.randomize_position;

        CoordinateSytem3::transform_coordinates(&Vector3::new(rand_x, rand_y, rand_z), &self.base.local_matrix, position_to_update);
    }

    fn get_class_name() -> String
    where
        Self: Sized,
    {
        todo!()
    }

    fn dispose()
    where
        Self: Sized,
    {
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

    fn get_local_matrix(& self) -> Matrix {
        self.base.local_matrix.clone()
    }

    fn get_align_direction(& self) -> bool {
        self.base.align_direction.clone()
    }

    fn get_randomize_direction(& self) -> f32 {
        self.base.randomize_direction.clone()
    }

    fn get_spherize_direction(& self) -> f32 {
        self.base.spherize_direction.clone()
    }

    fn get_randomize_position(& self) -> f32 {
        self.base.randomize_position.clone()
    }
}

// impl IShapeEmitterTypeValue for RectangleShapeEmitter {
//     const POSITION: Vector3 = Vector3::new(0., 0., 0.);

//     const ROTATION: Vector3 = Vector3::new(0., 0., 0.);

//     const SCALING: Vector3 = Vector3::new(1., 1., 1.);

//     const LOCAL_MATRIX: Matrix = Matrix::new(
//         1.0, 0., 0., 0., 0., 1., 0., 0., 1., 0., 1., 0., 0., 0., 0., 1.,
//     );

//     const ALIGN_DIRECTION: bool = false;

//     const RANDOMIZE_DIRECTION: f32 = 0.;

//     const SPHERIZE_DIRECTION: f32 = 0.;

//     const RANDOMIZE_POSITION: f32 = 0.;
// }
