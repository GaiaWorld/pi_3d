use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use crate::normalize;

use super::ishape_emitter_type::{EBoxShapeMode, IShapeEmitterType};

pub struct BoxShapeEmitter {
    pub emit_mode: EBoxShapeMode,
    direction: Vector3,
    position: Vector3,
    rotation: Vector3,
    scaling: Vector3,
    local_matrix: Matrix,
    align_direction: bool,
    randomize_direction: f32,
    spherize_direction: f32,
    randomize_position: f32,
}

impl BoxShapeEmitter {
    pub fn new() -> Self {
        Self {
            emit_mode: EBoxShapeMode::Volume,
            direction: Vector3::new(0.0, 0.0, 1.0),
            position: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
            scaling: Vector3::new(1., 1., 1.),
            local_matrix: Matrix::identity(),
            align_direction: false,
            randomize_direction: 0.,
            spherize_direction: 0.,
            randomize_position: 0.,
        }
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
        world_matrix: pi_scene_math::Matrix,
        direction_to_update: &mut Vector3,
        _position: pi_scene_math::Vector3,
        _local_position: pi_scene_math::Vector3,
        is_local: bool,
    ) {
        let mut direction = self.direction;

        direction[0] =
            direction[0] * (1.0 - self.spherize_direction) + direction[0] * self.spherize_direction;
        direction[1] =
            direction[1] * (1.0 - self.spherize_direction) + direction[1] * self.spherize_direction;
        direction[2] =
            direction[2] * (1.0 - self.spherize_direction) + direction[2] * self.spherize_direction;

        direction = normalize(&direction);
        let mut rng = rand::thread_rng();
        direction[0] += rng.gen::<f32>() * self.randomize_direction;
        direction[1] += rng.gen::<f32>() * self.randomize_direction;
        direction[2] += rng.gen::<f32>() * self.randomize_direction;

        direction = normalize(&direction);
        if is_local {
            *direction_to_update = direction;
        } else {
            *direction_to_update = world_matrix.transform_vector(&direction);
        }
        *direction_to_update =  normalize(direction_to_update);
    }

    fn start_position_function(
        &self,
        world_matrix: Matrix,
        position_to_update: &mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        is_local: bool,
    ) {
        let mut rng = rand::thread_rng();
        let mut rand_x: f32 = rng.gen_range(-0.5..0.5);
        let mut rand_y: f32 = rng.gen_range(-0.5..0.5);
        let mut rand_z: f32 = rng.gen_range(-0.5..0.5);

        match self.emit_mode {
            EBoxShapeMode::Volume => {
                // randX = Scalar.RandomRange(-0.5, 0.5);
                // randY = Scalar.RandomRange(-0.5, 0.5);
                // randZ = Scalar.RandomRange(-0.5, 0.5);
                // break;
            }
            EBoxShapeMode::Shell => {
                let face_range: f32 = rng.gen();
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
                let face_range: f32 = rng.gen();
                let face_range2: f32 = rng.gen();
                let face_range3: f32 = rng.gen();
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

        rand_x += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;
        rand_z += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;
        rand_y += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;

        if is_local {
            *position_to_update = Vector3::new(rand_x, rand_y, rand_z);
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(rand_x, rand_y, rand_z));
        }
    }

    fn set_postion(&mut self, position: Vector3) {
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

    fn get_local_matrix(&mut self) -> Matrix {
        self.local_matrix.clone()
    }

    fn get_align_direction(&mut self) -> bool {
        self.align_direction.clone()
    }

    fn get_randomize_direction(&mut self) -> f32 {
        self.randomize_direction.clone()
    }

    fn get_spherize_direction(&mut self) -> f32 {
        self.spherize_direction.clone()
    }

    fn get_randomize_position(&mut self) -> f32 {
        self.randomize_position.clone()
    }
}
