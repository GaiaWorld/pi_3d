use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use crate::normalize;

use super::{
    ishape_emitter_type::{
        compute_radians, EShapeEmitterArcMode, EShapeEmitterDirectionMode, IShapeEmitterType,
        IShapeEmitterTypeValue,
    },
    SerializationObject,
};

/**
 * 半球体发射器
 */
pub struct HemisphereShapeEmitter {
    _max_z: f32,
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
    pub direction_randomizer: f32,

    pub rotation: Vector3,
    pub position: Vector3,
    pub scaling: Vector3,

    local_matrix: Matrix,
    align_direction: bool,
    randomize_direction: f32,
    spherize_direction: f32,
    randomize_position: f32,
}

impl HemisphereShapeEmitter {
    pub fn new(radius: f32, radius_range: f32) -> Self {
        Self {
            _max_z: 9999999999.,
            direction_mode: EShapeEmitterDirectionMode::Unity,
            radius,
            radius_range,
            arc_value: std::f32::consts::PI,
            arc_mode: EShapeEmitterArcMode::Random,
            arc_spread: 0.,
            arc_speed: 1.,
            arc_spread_limit: 0.001,
            direction_randomizer: 1.,
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
    /**
     *
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> SerializationObject {
        SerializationObject {
            _type: Some(HemisphereShapeEmitter::get_class_name()),
            radius: Some(self.radius),
            angle: None,
            direction_randomizer: Some(self.direction_randomizer),
            radius_range: Some(self.radius_range),
            height_range: None,
            emit_from_spawn_point_only: None,
            size: None,
            direction1: None,
            direction2: None,
        }
    }

    /**
     * Parse properties from a JSON object
     * @param serializationObject defines the JSON object
     */
    pub fn parse(&mut self, arg: SerializationObject) {
        self.radius = arg.radius.unwrap();
        self.radius_range = arg.radius_range.unwrap();
        self.direction_randomizer = arg.direction_randomizer.unwrap();
    }

    /**
     * Clones the current emitter and returns a copy of it
     * @returns the new emitter
     */
    pub fn clone() {
        // let newOne = new HemisphereShapeEmitter(self.radius, self.radiusRange, self.directionRandomizer);

        // DeepCopier.DeepCopy(self, newOne);

        // return newOne;
    }

    pub fn start_position_function_local(
        &self,
        _world_matrix: Matrix,
        position_to_update: &mut Vector3,
        emission_loop: f32,
        emission_progress: f32,
        emission_index: f32,
        emission_total: f32,
    ) {
        let mut _s = 0.;
        let spread = self.arc_spread;
        let mut emission_progress = emission_progress * self.arc_speed;

        let mut rng = rand::thread_rng();
        match self.arc_mode {
            EShapeEmitterArcMode::Loop => {
                if spread > self.arc_spread_limit {
                    emission_progress = (emission_progress / spread).round() * spread;
                }

                _s = self.arc_value * emission_progress;
            }
            EShapeEmitterArcMode::PingPong => {
                if spread > self.arc_spread_limit {
                    emission_progress = (emission_progress / spread).round() * spread;
                }

                _s = self.arc_value * emission_progress * ((emission_loop % 2. - 0.5) * -2.);
            }
            EShapeEmitterArcMode::BurstsSpread => {
                emission_progress = emission_index / emission_total;
                if spread > self.arc_spread_limit {
                    emission_progress = (emission_progress / spread).floor() * spread;
                }

                _s = self.arc_value * emission_progress;
            }
            _ => {
                _s = rng.gen_range(0.0..self.arc_value);
            }
        }
        let rand_radius = self.radius - rng.gen::<f32>() * (self.radius * self.radius_range);
        let v: f32 = rng.gen_range(0.0..1.0);
        let phi = _s;
        let theta = (2. * v - 1.).acos();
        let mut rand_x = rand_radius * phi.cos() * theta.sin();
        let mut rand_z = rand_radius * theta.cos();
        let mut rand_y = rand_radius * phi.sin() * theta.sin();

        rand_x += (rng.gen::<f32>() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        rand_z += (rng.gen::<f32>() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        rand_y += (rng.gen::<f32>() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;

        *position_to_update = Vector3::new(rand_x, rand_y.abs(), rand_z);
    }
}
impl IShapeEmitterType for HemisphereShapeEmitter {
    fn start_direction_function(
        &self,
        world_matrix: pi_scene_math::Matrix,
        direction_to_update: &mut pi_scene_math::Vector3,
        position: pi_scene_math::Vector3,
        local_position: pi_scene_math::Vector3,
        is_local: bool,
    ) {
        let mut direction = if is_local {
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

        if is_local {
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
            std::f32::consts::PI * 2.,
            self.arc_value,
            self.arc_spread,
            self.arc_speed,
            self.arc_mode,
        );

        let mut rng = rand::thread_rng();

        let range = rng.gen::<f32>() * (self.radius_range);
        let rand_radius = self.radius - self.radius * range * range;
        let v: f32 = rng.gen_range(0.0..1.0);
        let phi = s;
        let theta = (2.0 * v - 1.0).acos();
        let mut rand_x = rand_radius * phi.cos() * theta.sin();
        let mut rand_z = (rand_radius * v).abs();
        let mut rand_y = rand_radius * phi.sin() * theta.sin();

        rand_x += rng.gen::<f32>() * Self::RANDOMIZE_POSITION;
        rand_z += rng.gen::<f32>() * Self::RANDOMIZE_POSITION;
        rand_y += rng.gen::<f32>() * Self::RANDOMIZE_POSITION;

        if is_local {
            *position_to_update = Vector3::new(rand_x, rand_y, rand_z);
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(rand_x, rand_y, rand_z));
        }
    }

    fn get_class_name() -> String
    where
        Self: Sized,
    {
        return "HemisphereParticleEmitter".to_string();
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

impl IShapeEmitterTypeValue for HemisphereShapeEmitter {
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
