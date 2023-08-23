use pi_scene_math::{Matrix, Vector3};

use crate::tools::{normalize, Random};

use super::{
    ishape_emitter_type::IShapeEmitterType,
    SerializationObject,
};

pub struct RectangleShapeEmitter {
    direction: Vector3,
    pub(crate)  rotation: Vector3,
    pub(crate)  position: Vector3,
    pub(crate)  scaling: Vector3,

    local_matrix: Matrix,
    pub(crate) align_direction: bool,
    pub(crate) randomize_direction: f32,
    pub(crate) spherize_direction: f32,
    pub(crate) randomize_position: f32,
}

impl RectangleShapeEmitter {
    pub fn serialize() {
        // todo!()
    }
    pub fn parse(_arg: SerializationObject) {
        // todo!()
    }
    pub fn new() -> Self {
        Self {
            direction: Vector3::new(0.0, 0.0, 1.0),
            rotation: Vector3::new(0., 0., 0.),
            position: Vector3::new(0., 0., 0.),
            scaling: Vector3::new(1., 1., 1.),

            local_matrix: Matrix::identity(),
            align_direction: false,
            randomize_direction: 0.,
            spherize_direction: 0.,
            randomize_position: 0.,
        }
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

        direction[0] = direction[0] * (1.0 - self.spherize_direction)
            + local_position[0] * self.spherize_direction;
        direction[1] = direction[1] * (1.0 - self.spherize_direction)
            + local_position[1] * self.spherize_direction;
        direction[2] = direction[2] * (1.0 - self.spherize_direction)
            + local_position[2] * self.spherize_direction;
        direction = normalize(&direction);

        direction[0] += random.random() * self.randomize_direction;
        direction[1] += random.random() * self.randomize_direction;
        direction[2] += random.random() * self.randomize_direction;

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

        rand_x += (random.random() * 2.0 - 1.0) * self.randomize_position;
        rand_z += (random.random() * 2.0 - 1.0) * self.randomize_position;
        rand_y += (random.random() * 2.0 - 1.0) * self.randomize_position;

            *position_to_update = Vector3::new(rand_x, rand_y, rand_z);
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
        self.position = position;
    }

    fn set_rotation(&mut self, rotation: Vector3) {
        self.rotation = rotation;
    }

    fn set_scaling(&mut self, scaling: Vector3) {
        self.scaling = scaling;
    }

    fn get_postion(&self) -> Vector3 {
        self.position.clone()
    }

    fn get_rotation(&self) -> Vector3 {
        self.rotation.clone()
    }

    fn get_scaling(&self) -> Vector3 {
        self.scaling.clone()
    }

    fn set_local_matrix(&mut self, local_matrix: Matrix) {
        self.local_matrix = local_matrix;
    }

    fn set_align_direction(&mut self, align_direction: bool) {
        self.align_direction = align_direction;
    }

    fn set_randomize_direction(&mut self, randomize_direction: f32) {
        self.randomize_direction = randomize_direction;
    }

    fn set_spherize_direction(&mut self, spherize_direction: f32) {
        self.spherize_direction = spherize_direction;
    }

    fn set_randomize_position(&mut self, randomize_position: f32) {
        self.randomize_position = randomize_position;
    }

    fn get_local_matrix(& self) -> Matrix {
        self.local_matrix.clone()
    }

    fn get_align_direction(& self) -> bool {
        self.align_direction.clone()
    }

    fn get_randomize_direction(& self) -> f32 {
        self.randomize_direction.clone()
    }

    fn get_spherize_direction(& self) -> f32 {
        self.spherize_direction.clone()
    }

    fn get_randomize_position(& self) -> f32 {
        self.randomize_position.clone()
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
