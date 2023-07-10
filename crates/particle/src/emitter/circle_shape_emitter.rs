use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use crate::normalize;

use super::ishape_emitter_type::{
    compute_radians, EShapeEmitterArcMode, IShapeEmitterType,
    IShapeEmitterTypeValue,
};

pub struct SerializationObject {
    pub _type: String,
    /**
     * 半径
     */
    pub radius: f32,
    pub direction_randomizer: f32,
    /**
     * 半径域
     */
    pub radius_range: f32,
}

/**
 * 圆形发射器
 */
pub struct CircleShapeEmitter {
    radius: f32,
    radius_range: f32,
    _direction_randomizer: f32,
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
    position: Vector3,
    rotation: Vector3,
    scaling: Vector3,
    local_matrix: Matrix,
    align_direction: bool,
    randomize_direction: f32,
    spherize_direction: f32,
    randomize_position: f32,
}

impl CircleShapeEmitter {
    pub fn new(radius: f32, radius_range: f32) -> Self {
        Self {
            radius,
            radius_range,
            _direction_randomizer: 0.,
            /**
             * 弧形范围
             */
            arc_value: std::f32::consts::PI * 2.,
            /**
             * 弧形范围发射模式
             */
            arc_mode: EShapeEmitterArcMode::Random,
            /**
             * 弧形周围可产生粒子的离散间隔 - 小于0.01 时, 不做间隔计算
             */
            arc_spread: 0.,
            /**
             * 弧形范围发射速度
             */
            arc_speed: 1.,
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

    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> SerializationObject {
        // let serializationObject: any = {};

        SerializationObject {
            _type: CircleShapeEmitter::get_class_name(),
            radius: self.radius,
            direction_randomizer: self._direction_randomizer,
            radius_range: self.radius_range,
        }
    }

    /**
     * Parse properties from a JSON object
     * @param serializationObject defines the JSON object
     */
    pub fn parse(&mut self, serialization_object: SerializationObject) {
        self.radius = serialization_object.radius;
        self.radius_range = if serialization_object.radius_range != 1. {
            serialization_object.radius_range
        } else {
            1.
        };
        self._direction_randomizer = serialization_object.direction_randomizer;
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
            normalize(&local_position)
        } else {
            normalize(
                &(position - Vector3::new(world_matrix[3], world_matrix[7], world_matrix[11])),
            )
        };
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
            self.arc_value,
            self.arc_spread,
            self.arc_speed,
            self.arc_mode,
        );
        let mut rng = rand::thread_rng();
        let rand_radius = self.radius - rng.gen::<f32>() * (self.radius * self.radius_range);
        let mut rand_x = rand_radius * s.cos();
        let mut rand_y = rand_radius * s.sin();
        let mut rand_z = 0.;

        rand_x += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;
        rand_z += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;
        rand_y += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;

        if is_local {
            *position_to_update = Vector3::new(rand_x, rand_y, rand_z)
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(rand_x, rand_y, rand_z));
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
