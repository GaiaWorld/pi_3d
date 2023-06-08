use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use super::ishape_emitter_type::{
    compute_radians, EShapeEmitterArcMode, EShapeEmitterDirectionMode, IShapeEmitterType,
    IShapeEmitterTypeValue,
};

pub struct SerializationObject {
    pub _type: String,
    /**
     * 半径
     */
    pub radius: f32,
    pub directionRandomizer: f32,
    /**
     * 半径域
     */
    pub radiusRange: f32,
}

/**
 * 圆形发射器
 */
pub struct CircleShapeEmitter {
    radius: f32,
    radiusRange: f32,
    directionRandomizer: f32,
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
    position: Vector3,
    rotation: Vector3,
    scaling: Vector3,
    localMatrix: Matrix,
    alignDirection: bool,
    randomizeDirection: f32,
    spherizeDirection: f32,
    randomizePosition: f32,
}

impl CircleShapeEmitter {
    const MAX_Z: f32 = 999999999.;
    const directionMode: EShapeEmitterDirectionMode = EShapeEmitterDirectionMode::Unity;

    /**
     * 弧形范围精度
     */
    const arcSpreadLimit: f32 = 0.001;

    /**
     *
     */
    pub fn new(radius: f32, radiusRange: f32) -> Self {
        Self {
            radius,
            radiusRange,
            directionRandomizer: 0.,
            /**
             * 弧形范围
             */
            arcValue: std::f32::consts::PI * 2.,
            /**
             * 弧形范围发射模式
             */
            arcMode: EShapeEmitterArcMode::Random,
            /**
             * 弧形周围可产生粒子的离散间隔 - 小于0.01 时, 不做间隔计算
             */
            arcSpread: 0.,
            /**
             * 弧形范围发射速度
             */
            arcSpeed: 1.,
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

    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    fn serialize(&self) -> SerializationObject {
        // let serializationObject: any = {};

        SerializationObject {
            _type: CircleShapeEmitter::get_class_name(),
            radius: self.radius,
            directionRandomizer: self.directionRandomizer,
            radiusRange: self.radiusRange,
        }
    }

    /**
     * Parse properties from a JSON object
     * @param serializationObject defines the JSON object
     */
    fn parse(&mut self, serializationObject: SerializationObject) {
        self.radius = serializationObject.radius;
        self.radiusRange = if serializationObject.radiusRange != 1. {
            serializationObject.radiusRange
        } else {
            1.
        };
        self.directionRandomizer = serializationObject.directionRandomizer;
    }
}

impl IShapeEmitterType for CircleShapeEmitter {
    fn start_direction_function(
        &self,
        world_matrix: Matrix,
        direction_to_update: &mut Vector3,
        position: Vector3,
        local_position: Vector3,
        is_local: bool,
    ) {
        let mut direction = if is_local {
            local_position.normalize()
        } else {
            (position - Vector3::new(world_matrix[3], world_matrix[7], world_matrix[11]))
                .normalize()
        };
        let mut rng = rand::thread_rng();

        direction[0] += rng.gen::<f32>() * self.randomizeDirection;
        direction[1] += rng.gen::<f32>() * self.randomizeDirection;
        direction[2] += rng.gen::<f32>() * self.randomizeDirection;

        direction = direction.normalize();

        if (is_local) {
            *direction_to_update = direction;
        } else {
            *direction_to_update = world_matrix.transform_vector(&direction);
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
        let s = compute_radians(
            emission_loop,
            emission_progress,
            emission_index,
            emission_total,
            std::f32::consts::PI * 2.,
            self.arcValue,
            self.arcSpread,
            self.arcSpeed,
            self.arcMode,
        );
        let mut rng = rand::thread_rng();
        let randRadius = self.radius - rng.gen::<f32>() * (self.radius * self.radiusRange);
        let mut randX = randRadius * s.cos();
        let mut randY = randRadius * s.sin();
        let mut randZ = 0.;

        randX += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;
        randZ += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;
        randY += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomizePosition;

        if is_local {
            *position_to_update = Vector3::new(randX, randY, randZ)
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(randX, randY, randZ));
        }
    }

    fn get_class_name() -> String
    where
        Self: Sized,
    {
        return "CircleParticleEmitter".to_string();
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

impl IShapeEmitterTypeValue for CircleShapeEmitter {
    const POSITION: Vector3 = Vector3::new(0., 0., 0.);

    const ROTATION: Vector3 = Vector3::new(0., 0., 0.);

    const SCALING: Vector3 = Vector3::new(1., 1., 1.);

    const LOCAL_MATRIX: Matrix = Matrix::new(
        1.0, 0., 0., 0., 0., 1., 0., 0., 1., 0., 1., 0., 0., 0., 0., 1.,
    );

    const ALIGN_DIRECTION: bool = false;

    const RANDOMIZE_DIRECTION: f32 = 0.;

    const SPHERIZE_DIRECTION: f32 = 0.;

    const RANDOMIZE_POSITION: f32 = 0.;
}
