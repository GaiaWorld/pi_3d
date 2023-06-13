use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use crate::normalize;

use super::{
    ishape_emitter_type::{
        compute_radians, EShapeEmitterArcMode, EShapeEmitterDirectionMode, IShapeEmitterType,
    },
    serializationObject,
};

pub struct EdgeShapeEmitter {
    MAX_Z: f32,
    directionMode: EShapeEmitterDirectionMode,
    pub size: f32,
    /**
     * 弧形范围
     */
    pub arcValue: f32,
    /**
     * 弧形范围发射模式
     */
    pub arcMode: EShapeEmitterArcMode,
    /**
     * 弧形周围可产生粒子的离散间隔 - 小于0.01 时, 不做间隔计算
     */
    pub arcSpread: f32,
    /**
     * 弧形范围发射速度
     */
    pub arcSpeed: f32,
    /**
     * 弧形范围精度
     */
    pub arcSpreadLimit: f32,

    pub rotation: Vector3,
    pub position: Vector3,
    pub scaling: Vector3,

    pub localMatrix: Matrix,
    pub alignDirection: bool,
    pub randomizeDirection: f32,
    pub spherizeDirection: f32,
    pub randomizePosition: f32,
}

impl EdgeShapeEmitter {
    pub fn new() -> Self {
        Self {
            MAX_Z: 999999999.,
            directionMode: EShapeEmitterDirectionMode::Unity,
            size: 0.,
            arcValue: 1.0,
            arcMode:  EShapeEmitterArcMode::Random,
            arcSpread: 0.,
            arcSpeed: 1.,
            arcSpreadLimit: 0.001,
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
    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> serializationObject {
        serializationObject {
            _type: Some(EdgeShapeEmitter::get_class_name()),
            radius: None,
            angle: None,
            directionRandomizer: None,
            radiusRange: None,
            heightRange: None,
            emitFromSpawnPointOnly: None,
            size: Some(self.size),
            direction1: None,
            direction2: None,
        }
    }

    /**
     * Parse properties from a JSON object
     * @param serializationObject defines the JSON object
     */
    pub fn parse(&mut self, size: f32) {
        self.size = size;
    }
}

impl IShapeEmitterType for EdgeShapeEmitter {
    fn start_direction_function(
        &self,
        world_matrix: pi_scene_math::Matrix,
        direction_to_update: &mut pi_scene_math::Vector3,
        position: pi_scene_math::Vector3,
        local_position: pi_scene_math::Vector3,
        is_local: bool,
    ) {
        let mut direction = Vector3::new(0., 1., 0.).normalize();
        // direction.copyFromFloats();

        // localPosition.normalizeToRef(TmpVectors.Vector3[1]);
        let local_position = normalize(&local_position);
        direction[0] = direction[0] * (1.0 - self.spherizeDirection)
            + local_position[0] * self.spherizeDirection;
        direction[1] = direction[1] * (1.0 - self.spherizeDirection)
            + local_position[1] * self.spherizeDirection;
        direction[2] = direction[2] * (1.0 - self.spherizeDirection)
            + local_position[2] * self.spherizeDirection;
        direction = normalize(&direction);

        let mut rng = rand::thread_rng();
        direction[0] += rng.gen::<f32>() * self.randomizeDirection;
        direction[1] += rng.gen::<f32>() * self.randomizeDirection;
        direction[2] += rng.gen::<f32>() * self.randomizeDirection;

        direction = normalize(&direction);

        if (is_local) {
            *direction_to_update = direction;
        } else {
            *direction_to_update = world_matrix.transform_vector(&direction);
        }
    }

    fn start_position_function(
        &self,
        world_matrix: pi_scene_math::Matrix,
        position_to_update: &mut pi_scene_math::Vector3,
        emission_loop: f32,
        emission_progress: f32,
        emission_index: f32,
        emission_total: f32,
        is_local: bool,
    ) {
        let s = compute_radians(
            emission_loop,
            emission_progress,
            emission_index,
            emission_total,
            1.0,
            self.arcValue,
            self.arcSpread,
            self.arcSpeed,
            self.arcMode,
        );

        let mut randX = self.size * (s / self.arcValue * 2. - 1.);
        let mut randY = 0.;
        let mut randZ = 0.;
        let mut rng = rand::thread_rng();
        randX += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;
        randZ += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;
        randY += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;


        if (is_local) {
            *position_to_update = Vector3::new(randX, randY, randZ);
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(randX, randY, randZ));
        }
    }

    fn get_class_name() -> String
    where
        Self: Sized,
    {
        return "EdgeParticleEmitter".to_string();
    }

    fn dispose()
    where
        Self: Sized,
    {
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
