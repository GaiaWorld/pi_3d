use pi_scene_math::{Matrix, Vector3, Vector4};
use rand::Rng;

use super::ishape_emitter_type::{EBoxShapeMode, IShapeEmitterType, IShapeEmitterTypeValue};

pub struct BoxShapeEmitter {
    pub emitMode: EBoxShapeMode,
    direction: Vector3,
    position: Vector3,
    rotation: Vector3,
    scaling: Vector3,
    localMatrix: Matrix,
    alignDirection: bool,
    randomizeDirection: f32,
    spherizeDirection: f32,
    randomizePosition: f32,
}

impl BoxShapeEmitter {
    pub fn new() -> Self {
        Self {
            emitMode: EBoxShapeMode::Volume,
            direction: Vector3::new(0.0, 0.0, 1.0),
            position: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
            scaling: Vector3::new(1., 1., 1.),
            localMatrix: Matrix::identity(),
            alignDirection: false,
            randomizeDirection: 0.,
            spherizeDirection: 0.,
            randomizePosition: 0.,
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
        position: pi_scene_math::Vector3,
        local_position: pi_scene_math::Vector3,
        is_local: bool,
    ) {
        let mut direction = self.direction;

        direction[0] =
            direction[0] * (1.0 - self.spherizeDirection) + direction[0] * self.spherizeDirection;
        direction[1] =
            direction[1] * (1.0 - self.spherizeDirection) + direction[1] * self.spherizeDirection;
        direction[2] =
            direction[2] * (1.0 - self.spherizeDirection) + direction[2] * self.spherizeDirection;

        direction = direction.normalize();
        let mut rng = rand::thread_rng();
        direction[0] += rng.gen::<f32>() * self.randomizeDirection;
        direction[1] += rng.gen::<f32>() * self.randomizeDirection;
        direction[2] += rng.gen::<f32>() * self.randomizeDirection;

        direction = direction.normalize();
        if is_local {
            *direction_to_update = direction;
        } else {
            *direction_to_update = world_matrix.transform_vector(&direction);
        }
        *direction_to_update = direction_to_update.normalize();
    }

    fn start_position_function(
        &self,
        world_matrix: Matrix,
        position_to_update: &mut Vector3,
        emission_loop: f32,
        emission_progress: f32,
        emission_index: f32,
        emission_total: f32,
        is_local: bool,
    ) {
        let mut rng = rand::thread_rng();
        let mut randX: f32 = rng.gen_range(-0.5..0.5);
        let mut randY: f32 = rng.gen_range(-0.5..0.5);
        let mut randZ: f32 = rng.gen_range(-0.5..0.5);

        match self.emitMode {
            EBoxShapeMode::Volume => {
                // randX = Scalar.RandomRange(-0.5, 0.5);
                // randY = Scalar.RandomRange(-0.5, 0.5);
                // randZ = Scalar.RandomRange(-0.5, 0.5);
                // break;
            }
            EBoxShapeMode::Shell => {
                let face_range: f32 = rng.gen();
                // YZ
                if (face_range < 0.3333f32) {
                    randX = if face_range < 0.1666 { 0.5 } else { -0.5 };
                // XZ
                } else if (face_range < 0.6666) {
                    randY = if face_range < 0.5 { 0.5 } else { -0.5 };
                // XY
                } else {
                    randZ = if face_range < 0.8333 { 0.5 } else { -0.5 };
                }
            }
            EBoxShapeMode::Edge => {
                let faceRange: f32 = rng.gen();
                let faceRange2: f32 = rng.gen();
                let faceRange3: f32 = rng.gen();
                // X
                if (faceRange < 0.3333) {
                    randY = if faceRange2 < 0.5 { 0.5 } else { -0.5 };
                    randZ = if faceRange3 < 0.5 { 0.5 } else { -0.5 };
                // Y
                } else if (faceRange < 0.6666) {
                    randX = if faceRange2 < 0.5 { 0.5 } else { -0.5 };
                    randZ = if faceRange3 < 0.5 { 0.5 } else { -0.5 };
                // Z
                } else {
                    randX = if faceRange2 < 0.5 { 0.5 } else { -0.5 };
                    randY = if faceRange3 < 0.5 { 0.5 } else { -0.5 };
                }
            }
        }

        randX += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;
        randZ += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;
        randY += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;

        if (is_local) {
            *position_to_update = Vector3::new(randX, randY, randZ);
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(randX, randY, randZ));
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

    fn set_localMatrix(&mut self, localMatrix: Matrix) {
        self.localMatrix = localMatrix;
    }

    fn set_alignDirection(&mut self, alignDirection: bool) {
        self.alignDirection = alignDirection;
    }

    fn set_randomizeDirection(&mut self, randomizeDirection: f32) {
        self.randomizeDirection = randomizeDirection;
    }

    fn set_spherizeDirection(&mut self, spherizeDirection: f32) {
        self.spherizeDirection = spherizeDirection;
    }

    fn set_randomizePosition(&mut self, randomizePosition: f32) {
        self.randomizePosition = randomizePosition;
    }

    fn get_localMatrix(&mut self) -> Matrix {
        self.localMatrix.clone()
    }

    fn get_alignDirection(&mut self) -> bool {
        self.alignDirection.clone()
    }

    fn get_randomizeDirection(&mut self) -> f32 {
        self.randomizeDirection.clone()
    }

    fn get_spherizeDirection(&mut self) -> f32 {
        self.spherizeDirection.clone()
    }

    fn get_randomizePosition(&mut self) -> f32 {
        self.randomizePosition.clone()
    }
}
