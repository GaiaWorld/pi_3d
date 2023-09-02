use pi_scene_math::{Matrix, Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::tools::{normalize, Random};

use super::ishape_emitter_type::{
    compute_radians, EShapeEmitterArcMode, EShapeEmitterDirectionMode, IShapeEmitterType,
    IShapeEmitterTypeValue,
};

/**
 * 球体发射器
 */
pub struct SphereShapeEmitter {
    pub max_z: f32,
    /**
     * 创建模式
     */
    pub direction_mode: EShapeEmitterDirectionMode,
    /**
     * 半径
     */
    pub radius: f32,
    /**
     * 半径域
     */
    pub radius_range: f32,
    /**
     * 弧形范围
     */
    pub arc_value: f32,
    /**
     * 弧形范围发射模式
     */
    pub arc_mode: EShapeEmitterArcMode,
    /**
     * 弧形周围可产生粒子的离散间隔 - 小于0.01 时, 不做间隔计算
     */
    pub arc_spread: f32,
    /**
     * 弧形范围发射速度
     */
    pub arc_speed: f32,
    /**
     * 弧形范围精度
     */
    pub arc_spread_limit: f32,
    pub(crate)  rotation: Vector3,
    pub(crate)  position: Vector3,
    pub(crate)  scaling: Vector3,

    pub(crate) local_matrix: Matrix,
    pub(crate) align_direction: bool,
    pub(crate) randomize_direction: f32,
    pub(crate) spherize_direction: f32,
    pub(crate) randomize_position: f32,
}

impl SphereShapeEmitter {
    pub fn new(radius: f32, radius_range: f32) -> Self {
        Self {
            max_z: 9999999999.,
            direction_mode: EShapeEmitterDirectionMode::Unity,
            radius,
            radius_range,
            arc_value: std::f32::consts::PI * 2.,
            arc_mode: EShapeEmitterArcMode::Random,
            arc_spread: 0.,
            arc_speed: 1.,
            arc_spread_limit: 0.001,
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

impl IShapeEmitterType for SphereShapeEmitter {
    fn start_direction_function(
        &self,
        direction_to_update: &mut pi_scene_math::Vector3,
        local_position: &Vector3,
        // is_local: bool,
        random: &mut Random,
    ) {
        let mut direction = normalize(&local_position);

        direction[0] += random.random() * Self::RANDOMIZE_DIRECTION;
        direction[1] += random.random() * Self::RANDOMIZE_DIRECTION;
        direction[2] += random.random() * Self::RANDOMIZE_DIRECTION;

        direction = normalize(&direction);

            *direction_to_update = direction;
    }

    fn start_position_function(
        &self,
        position_to_update: &mut pi_scene_math::Vector3,
        emission_loop: f32,
        emission_progress: f32,
        emission_index: f32,
        emission_total: f32,
        // is_local: bool,
        random: &mut Random,
    ) {
        let s = compute_radians(
            emission_loop,
            emission_progress,
            emission_index,
            emission_total,
            std::f32::consts::PI * 2.,
            self.arc_value,
            self.arc_spread,
            self.arc_speed,
            self.arc_mode,
            random
        );

        let range: f32 = random.random() * (self.radius_range);
        let rand_radius = self.radius - self.radius * range * range;
        let mut v: f32 = random.random_range(0.0, 1.0);
        v = 2. * v - 1.;
        let phi = s;
        let theta = v.acos();
        let mut rand_x = rand_radius * phi.cos() * theta.sin();
        let mut rand_z = rand_radius * v;
        let mut rand_y = rand_radius * phi.sin() * theta.sin();

        rand_x += (random.random() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        rand_z += (random.random() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        rand_y += (random.random() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;

        CoordinateSytem3::transform_coordinates(&Vector3::new(rand_x, rand_y, rand_z), &self.local_matrix, position_to_update);
    }

    fn get_class_name() -> String {
        return "SphereParticleEmitter".to_string();
    }

    fn dispose() {
        todo!()
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
