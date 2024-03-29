use pi_scene_math::{Matrix, Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::tools::{normalize, Random};

use super::{
    ishape_emitter_type::{
        compute_radians, EShapeEmitterArcMode, EShapeEmitterDirectionMode, IShapeEmitterType,
    },
    SerializationObject,
};

/**
 * 锥体发射器
 */
pub struct ConeShapeEmitter {
    _base_height: f32,
    _start_radius: f32,
    /**
     * 半径域
     */
    pub radius_range: f32,
    _height: f32,
    _angle: f32,
    /**
     * Gets or sets a value indicating if all the particles should be emitted from the spawn point only (the base of the cone)
     */
    pub emit_from_spawn_point_only: bool,
    pub direction_randomizer: f32,
    /**
     * 高度范围
     */
    pub height_range: f32,
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
    pub(crate) position: Vector3,
    pub(crate)  rotation: Vector3,
    pub(crate) scaling: Vector3,
    pub(crate) local_matrix: Matrix,
    pub(crate) align_direction: bool,
    pub(crate) randomize_direction: f32,
    pub(crate) spherize_direction: f32,
    pub(crate) randomize_position: f32,
}

impl ConeShapeEmitter {
    const MAX_Z: f32 = 999999999.;
    const DIRECTION_MODE: EShapeEmitterDirectionMode = EShapeEmitterDirectionMode::Unity;
    const RANDOMIZE_DIRECTION: f32 = 0.;
    const SPHERIZE_DIRECTION: f32 = 0.;
    const RANDOMIZE_POSITION: f32 = 0.;

    pub fn new(radius: f32, angle: f32) -> Self {
        Self {
            _base_height: 0.,
            _start_radius: radius.max(0.0001),
            radius_range: 0.,
            _height: 1.,
            _angle: angle,
            emit_from_spawn_point_only: false,
            direction_randomizer: 0.,
            height_range: 0.,
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
     * 半径
     */
    pub fn get_radius(&self) -> f32 {
        return self._start_radius;
    }
    /**
     * 半径
     */
    pub fn set_radius(&mut self, value: f32) {
        self._start_radius = value.max(0.0001);
        self._build_shape();
    }

    /**
     * 锥体顶角弧度
     */
    pub fn get_angle(&self) -> f32 {
        return self._angle;
    }

    /**
     * 锥体顶角弧度
     */
    pub fn set_angle(&mut self, value: f32) {
        self._angle = value;
        self._build_shape();
    }

    /**
     * 锥体高度 - 当 heightRange > 0
     */
    pub fn get_height(&self) -> f32 {
        return self._height;
    }

    /**
     * 锥体高度 - 当 heightRange > 0
     */
    pub fn set_height(&mut self, value: f32) {
        self._height = value.max(0.0);
        self._build_shape();
    }

    fn _build_shape(&mut self) {
        if self._angle != 0. {
            self._base_height = self._start_radius / (self._angle / 2.).tan();
        } else {
            self._base_height = ConeShapeEmitter::MAX_Z;
        }
    }

    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> SerializationObject {
        SerializationObject {
            _type: Some(ConeShapeEmitter::get_class_name()),
            radius: Some(self._start_radius),
            angle: Some(self._angle),
            direction_randomizer: Some(self.direction_randomizer),
            radius_range: Some(self.radius_range),
            height_range: Some(self.height_range),
            emit_from_spawn_point_only: Some(self.emit_from_spawn_point_only),
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
        self._start_radius = arg.radius.unwrap();
        self._angle = arg.angle.unwrap();
        self.direction_randomizer = arg.direction_randomizer.unwrap();

        self.radius_range = arg.radius_range.unwrap();
        self.height_range = arg.height_range.unwrap();
        self.emit_from_spawn_point_only = arg.emit_from_spawn_point_only.unwrap();
    }

    // const ROTATION: Vector3 = Vector3::new(0., 0., 0.);
    // const position: Vector3 = Vector3::new(0., 0., 0.);
    // const scaling: Vector3 = Vector3::new(1., 1., 1.);
    // const localMatrix: Matrix = Matrix::new(
    //     1.0, 0., 0., 0., 
    //     0., 1., 0., 0., 
    //     1., 0., 1., 0., 
    //     0., 0., 0., 1.,
    // );

    // const alignDirection: bool = false;

}

impl IShapeEmitterType for ConeShapeEmitter {
    fn start_direction_function(
        &self,
        direction_to_update: &mut pi_scene_math::Vector3,
        local_position: &Vector3,
        random: &mut Random,
    ) {
        // println!(
        //     "ConeShapeEmitter::start_direction_function: {}",
        //     direction_to_update
        // );
        let mut direction = normalize(&(local_position + Vector3::new(0., 0., self._base_height)));
        // println!("ConeShapeEmitter::start_direction_function1: {:?}, local_position: {:?}, Self::spherizeDirection: {:?}", direction, local_position, Self::spherizeDirection);

        let local_position = normalize(local_position);
        // println!("ConeShapeEmitter local_position: {:?}", local_position);
        let x = direction.x * (1.0 - Self::SPHERIZE_DIRECTION) + local_position[0] * Self::SPHERIZE_DIRECTION;
        // println!("=============x : {:?}", x);
        direction.x = x;
        // println!(
        //     "ConeShapeEmitter::start_direction_function111: {}",
        //     direction
        // );
        direction.y = direction.y * (1.0 - Self::SPHERIZE_DIRECTION) + local_position.y * Self::SPHERIZE_DIRECTION;
        direction.z = direction.z * (1.0 - Self::SPHERIZE_DIRECTION) + local_position.z * Self::SPHERIZE_DIRECTION;
        // println!(
        //     "ConeShapeEmitter::start_direction_function11: {}",
        //     direction
        // );
        direction = normalize(&direction);
        // println!("ConeShapeEmitter::start_direction_function2: {}", direction);
        direction[0] += random.random() * Self::RANDOMIZE_DIRECTION;
        direction[1] += random.random() * Self::RANDOMIZE_DIRECTION;
        direction[2] += random.random() * Self::RANDOMIZE_DIRECTION;
        // println!("ConeShapeEmitter::start_direction_function3: {}", direction);
        // *direction_to_update = direction;
        *direction_to_update = normalize(&direction);
    }

    fn start_position_function(
        &self,
        position_to_update: &mut pi_scene_math::Vector3,
        emission_loop: f32,
        emission_progress: f32,
        emission_index: f32,
        emission_total: f32,
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
            random,
        );

        let mut h: f32 = 0.;

        if !self.emit_from_spawn_point_only {
            h = random.random() * self.height_range;
        }
        h = h * h;
        h = h.max(0.00001);

        let t = random.random() * self.radius_range;
        let mut radius = self._start_radius - self._start_radius * t * t;
        if self._base_height > 0.00001 {
            radius = radius * (h * self._height + self._base_height) / self._base_height;
        }

        // log::warn!("_base_height: {:?}", self._base_height);

        let mut _rand_x = 0.;
        let mut _rand_z = 0.;
        let mut _rand_y = 0.;

        // if Self::DIRECTION_MODE == EShapeEmitterDirectionMode::Unity {
        //     _rand_x = radius * (s).cos();
        //     _rand_y = radius * (s).sin();
        //     _rand_z = h * self._height;
        // } else {
        //     _rand_x = radius * s.sin();
        //     _rand_z = radius * s.cos();
        //     _rand_y = h * self._height;
        // }
        _rand_x = radius * (s).cos();
        _rand_y = radius * (s).sin();
        _rand_z = h * self._height;

        _rand_x += (random.random() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        _rand_z += (random.random() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        _rand_y += (random.random() * 2.0 - 1.0) * Self::RANDOMIZE_POSITION;
        
        // log::warn!("Position: {:?}", (_rand_x, _rand_y, _rand_z));

        CoordinateSytem3::transform_coordinates(&Vector3::new(_rand_x, _rand_y, _rand_z), &self.local_matrix, position_to_update);
    }

    fn get_class_name() -> String
    where
        Self: Sized,
    {
        return "ConeParticleEmitter".to_string();
    }

    fn dispose()
    where
        Self: Sized,
    {
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

// impl IShapeEmitterTypeValue for ConeShapeEmitter{
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
