use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use crate::normalize;

use super::ishape_emitter_type::{
    compute_radians, EShapeEmitterArcMode, EShapeEmitterDirectionMode, IShapeEmitterType,
    IShapeEmitterTypeValue,
};

/**
 * 球体发射器
 */
pub struct SphereShapeEmitter {
    pub MAX_Z: f32,
    /**
     * 创建模式
     */
    pub directionMode: EShapeEmitterDirectionMode,
    /**
     * 半径
     */
    pub radius: f32,
    /**
     * 半径域
     */
    pub radiusRange: f32,
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

    localMatrix: Matrix,
    alignDirection: bool,
    randomizeDirection: f32,
    spherizeDirection: f32,
    randomizePosition: f32,
}

impl SphereShapeEmitter {
    pub fn new(radius: f32, radiusRange: f32) -> Self {
        Self {
            MAX_Z: 9999999999.,
            directionMode: EShapeEmitterDirectionMode::Unity,
            radius,
            radiusRange,
            arcValue: std::f32::consts::PI * 2.,
            arcMode: EShapeEmitterArcMode::Random,
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
}

impl IShapeEmitterType for SphereShapeEmitter {
    fn start_direction_function(
        &self,
        world_matrix: pi_scene_math::Matrix,
        direction_to_update: &mut pi_scene_math::Vector3,
        position: pi_scene_math::Vector3,
        local_position: pi_scene_math::Vector3,
        is_local: bool,
    ) {
        let mut direction = if (is_local) {
            normalize(&local_position)
        } else {
            normalize(
                &(position - Vector3::new(world_matrix[3], world_matrix[7], world_matrix[11])),
            )
        };

        let mut rng = rand::thread_rng();
        direction[0] += rng.gen::<f32>() * Self::RANDOMIZE_DIRECTION;
        direction[1] += rng.gen::<f32>() * Self::RANDOMIZE_DIRECTION;
        direction[2] += rng.gen::<f32>() * Self::RANDOMIZE_DIRECTION;

        direction = normalize(&direction);
        if (is_local) {
            *direction_to_update = direction;
        } else {
            *direction_to_update = normalize(&world_matrix.transform_vector(&direction));
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
            std::f32::consts::PI * 2.,
            self.arcValue,
            self.arcSpread,
            self.arcSpeed,
            self.arcMode,
        );

        let mut rng = rand::thread_rng();
        let range: f32 = rng.gen::<f32>() * (self.radiusRange);
        let randRadius = self.radius - self.radius * range * range;
        let mut v: f32 = rng.gen_range(0.0..1.0);
        v = 2. * v - 1.;
        let phi = s;
        let theta = v.acos();
        let mut randX = randRadius * phi.cos() * theta.sin();
        let mut randZ = randRadius * v;
        let mut randY = randRadius * phi.sin() * theta.sin();

        randX += (rng.gen::<f32>() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        randZ += (rng.gen::<f32>() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        randY += (rng.gen::<f32>() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;

        if (is_local) {
            *position_to_update = Vector3::new(randX, randY, randZ);
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(randX, randY, randZ));
        }
    }

    fn get_class_name() -> String {
        return "SphereParticleEmitter".to_string();
    }

    fn dispose() {
        todo!()
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

impl IShapeEmitterTypeValue for SphereShapeEmitter {
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
