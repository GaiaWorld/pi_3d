use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use super::{
    ishape_emitter_type::{IShapeEmitterType, IShapeEmitterTypeValue},
    serializationObject,
};

pub struct RectangleShapeEmitter {
    direction: Vector3,
    pub rotation: Vector3,
    pub position: Vector3,
    pub scaling: Vector3,

    localMatrix: Matrix,
    alignDirection: bool,
    randomizeDirection: f32,
    spherizeDirection: f32,
    randomizePosition: f32,
}

impl RectangleShapeEmitter {
    pub fn serialize() {
        todo!()
    }
    pub fn parse(arg: serializationObject) {
        todo!()
    }
    pub fn new()->Self{
        Self{
            direction: Vector3::new(0.0, 0.0, 1.0),
            rotation: Vector3::new(0., 0., 0.),
            position: Vector3::new(0., 0., 0.),
            scaling: Vector3::new(1., 1., 1.),

            localMatrix: Matrix::identity(),
            alignDirection: false,
            randomizeDirection: 0.,
            spherizeDirection: 0.,
            randomizePosition: 0.,
        }
    }
}

impl IShapeEmitterType for RectangleShapeEmitter {
    fn start_direction_function(
        &self,
        world_matrix: Matrix,
        direction_to_update: &mut Vector3,
        position: Vector3,
        local_position: Vector3,
        is_local: bool,
    ) {
        let mut direction = self.direction;

        let local_position = local_position.normalize();

        direction[0] = direction[0] * (1.0 - self.spherizeDirection)
            + local_position[0] * self.spherizeDirection;
        direction[1] = direction[1] * (1.0 - self.spherizeDirection)
            + local_position[1] * self.spherizeDirection;
        direction[2] = direction[2] * (1.0 - self.spherizeDirection)
            + local_position[2] * self.spherizeDirection;
        direction = direction.normalize();

        let mut rng = rand::thread_rng();
        direction[0] += rng.gen::<f32>() * self.randomizeDirection;
        direction[1] += rng.gen::<f32>() * self.randomizeDirection;
        direction[2] += rng.gen::<f32>() * self.randomizeDirection;
        
        direction = direction.normalize();

        if (is_local) {
            *direction_to_update = direction;
        } else {
            *direction_to_update = world_matrix.transform_vector(&direction).normalize();
        }
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
        let mut randZ: f32 = 0.;

        randX += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;
        randZ += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;
        randY += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;

        if is_local {
            *position_to_update = Vector3::new(randX, randY, randZ);
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(randX, randY, randZ));
        }
    }

    fn get_class_name() -> String
    where
        Self: Sized {
            todo!()
    }

    fn dispose()
    where
        Self: Sized {
        
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

    fn get_randomizeDirection(&mut self, ) -> f32 {
        self.randomizeDirection.clone()
    }

    fn get_spherizeDirection(&mut self) -> f32 {
        self.spherizeDirection.clone()
    }

    fn get_randomizePosition(&mut self) -> f32 {
        self.randomizePosition.clone()
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
