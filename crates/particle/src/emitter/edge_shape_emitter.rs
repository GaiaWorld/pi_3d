use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use crate::normalize;

use super::{
    ishape_emitter_type::{
        compute_radians, EShapeEmitterArcMode, EShapeEmitterDirectionMode, IShapeEmitterType,
    },
    SerializationObject,
};

pub struct EdgeShapeEmitter {
    _max_z: f32,
    _direction_mode: EShapeEmitterDirectionMode,
    pub size: f32,
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

    pub rotation: Vector3,
    pub position: Vector3,
    pub scaling: Vector3,

    pub local_matrix: Matrix,
    pub align_direction: bool,
    pub randomize_direction: f32,
    pub spherize_direction: f32,
    pub randomize_position: f32,
}

impl EdgeShapeEmitter {
    pub fn new() -> Self {
        Self {
            _max_z: 999999999.,
            _direction_mode: EShapeEmitterDirectionMode::Unity,
            size: 0.,
            arc_value: 1.0,
            arc_mode:  EShapeEmitterArcMode::Random,
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
    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> SerializationObject {
        SerializationObject {
            _type: Some(EdgeShapeEmitter::get_class_name()),
            radius: None,
            angle: None,
            direction_randomizer: None,
            radius_range: None,
            height_range: None,
            emit_from_spawn_point_only: None,
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
        _position: pi_scene_math::Vector3,
        local_position: pi_scene_math::Vector3,
        is_local: bool,
    ) {
        let mut direction = Vector3::new(0., 1., 0.).normalize();
        // direction.copyFromFloats();

        // localPosition.normalizeToRef(TmpVectors.Vector3[1]);
        let local_position = normalize(&local_position);
        direction[0] = direction[0] * (1.0 - self.spherize_direction)
            + local_position[0] * self.spherize_direction;
        direction[1] = direction[1] * (1.0 - self.spherize_direction)
            + local_position[1] * self.spherize_direction;
        direction[2] = direction[2] * (1.0 - self.spherize_direction)
            + local_position[2] * self.spherize_direction;
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
            self.arc_value,
            self.arc_spread,
            self.arc_speed,
            self.arc_mode,
        );

        let mut rand_x = self.size * (s / self.arc_value * 2. - 1.);
        let mut rand_y = 0.;
        let mut rand_z = 0.;
        let mut rng = rand::thread_rng();
        rand_x += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;
        rand_z += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;
        rand_y += (rng.gen::<f32>() * 2.0 - 1.0) * self.randomize_position;


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

    fn get_randomize_direction(&mut self, ) -> f32 {
        self.randomize_direction.clone()
    }

    fn get_spherize_direction(&mut self) -> f32 {
        self.spherize_direction.clone()
    }

    fn get_randomize_position(&mut self) -> f32 {
        self.randomize_position.clone()
    }
}
