use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use crate::normalize;

use super::{
    ishape_emitter_type::{
        compute_radians, EShapeEmitterArcMode, EShapeEmitterDirectionMode, IShapeEmitterType,
    },
    serializationObject,
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
    position: Vector3,
    rotation: Vector3,
    scaling: Vector3,
    local_matrix: Matrix,
    align_direction: bool,
    randomize_direction: f32,
    spherize_direction: f32,
    randomize_position: f32,
}

impl ConeShapeEmitter {
    const MAX_Z: f32 = 999999999.;
    const DIRECTION_MODE: EShapeEmitterDirectionMode = EShapeEmitterDirectionMode::Unity;

    /**
     * 弧形范围精度
     */
    const ARC_SPREAD_LIMIT: f32 = 0.001;
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
        self._buildShape();
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
        self._buildShape();
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
        self._buildShape();
    }

    fn _buildShape(&mut self) {
        if (self._angle != 0.) {
            self._base_height = self._start_radius / (self._angle / 2.).tan();
        } else {
            self._base_height = ConeShapeEmitter::MAX_Z;
        }
    }

    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> serializationObject {
        serializationObject {
            _type: Some(ConeShapeEmitter::get_class_name()),
            radius: Some(self._start_radius),
            angle: Some(self._angle),
            directionRandomizer: Some(self.direction_randomizer),
            radiusRange: Some(self.radius_range),
            heightRange: Some(self.height_range),
            emitFromSpawnPointOnly: Some(self.emit_from_spawn_point_only),
            size: None,
            direction1: None,
            direction2: None,
        }
    }

    /**
     * Parse properties from a JSON object
     * @param serializationObject defines the JSON object
     */
    pub fn parse(&mut self, arg: serializationObject) {
        self._start_radius = arg.radius.unwrap();
        self._angle = arg.angle.unwrap();
        self.direction_randomizer = arg.directionRandomizer.unwrap();

        self.radius_range = arg.radiusRange.unwrap();
        self.height_range = arg.heightRange.unwrap();
        self.emit_from_spawn_point_only = arg.emitFromSpawnPointOnly.unwrap();
    }

    const rotation: Vector3 = Vector3::new(0., 0., 0.);
    const position: Vector3 = Vector3::new(0., 0., 0.);
    const scaling: Vector3 = Vector3::new(1., 1., 1.);
    const localMatrix: Matrix = Matrix::new(
        1.0, 0., 0., 0., 
        0., 1., 0., 0., 
        1., 0., 1., 0., 
        0., 0., 0., 1.,
    );

    const alignDirection: bool = false;
    const randomizeDirection: f32 = 0.;
    const spherizeDirection: f32 = 0.;
    const randomizePosition: f32 = 0.;
}

impl IShapeEmitterType for ConeShapeEmitter {
    fn start_direction_function(
        &self,
        world_matrix: pi_scene_math::Matrix,
        direction_to_update: &mut pi_scene_math::Vector3,
        position: pi_scene_math::Vector3,
        local_position: pi_scene_math::Vector3,
        is_local: bool,
    ) {
        // println!(
        //     "ConeShapeEmitter::start_direction_function: {}",
        //     direction_to_update
        // );
        let mut direction = if is_local {
            normalize(&(local_position + Vector3::new(0., 0., self._base_height)))
        } else {
            let temp = world_matrix.transform_vector(&Vector3::new(
                position[0],
                position[1],
                position[2] + self._base_height,
            ));
            normalize(&(temp - Vector3::new(world_matrix[3], world_matrix[7], world_matrix[11])))
        };
        // println!("ConeShapeEmitter::start_direction_function1: {:?}, local_position: {:?}, Self::spherizeDirection: {:?}", direction, local_position, Self::spherizeDirection);

        let local_position = normalize(&local_position);
        // println!("ConeShapeEmitter local_position: {:?}", local_position);
        let x = direction[0] * (1.0 - Self::spherizeDirection)
            + local_position[0] * Self::spherizeDirection;
        // println!("=============x : {:?}", x);
        direction[0] = x;
        // println!(
        //     "ConeShapeEmitter::start_direction_function111: {}",
        //     direction
        // );
        direction[1] = direction[1] * (1.0 - Self::spherizeDirection)
            + local_position[1] * Self::spherizeDirection;
        direction[2] = direction[2] * (1.0 - Self::spherizeDirection)
            + local_position[2] * Self::spherizeDirection;
        // println!(
        //     "ConeShapeEmitter::start_direction_function11: {}",
        //     direction
        // );
        direction = normalize(&direction);
        // println!("ConeShapeEmitter::start_direction_function2: {}", direction);
        let mut rng = rand::thread_rng();
        direction[0] += rng.gen::<f32>() * Self::randomizeDirection;
        direction[1] += rng.gen::<f32>() * Self::randomizeDirection;
        direction[2] += rng.gen::<f32>() * Self::randomizeDirection;
        // println!("ConeShapeEmitter::start_direction_function3: {}", direction);
        *direction_to_update = direction;
        *direction_to_update = normalize(direction_to_update);
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

        let mut h = 0.;

        let mut rng = rand::thread_rng();
        if !self.emit_from_spawn_point_only {
            h = rng.gen::<f32>() * self.height_range;
        }
        h = h * h;
        h = h.max(0.00001);

        let t = rng.gen::<f32>() * self.radius_range;
        let mut radius = self._start_radius - self._start_radius * t * t;
        if (self._base_height > 0.) {
            radius = radius * (h * self._height + self._base_height) / self._base_height;
        }

        let mut randX = 0.;
        let mut randZ = 0.;
        let mut randY = 0.;

        if (Self::DIRECTION_MODE == EShapeEmitterDirectionMode::Unity) {
            randX = radius * (-s).sin();
            randY = radius * (-s).cos();
            randZ = h * self._height;
        } else {
            randX = radius * s.sin();
            randZ = radius * s.cos();
            randY = h * self._height;
        }

        randX += (rng.gen::<f32>() * 2.0 - 1.0) * Self::randomizePosition;
        randZ += (rng.gen::<f32>() * 2.0 - 1.0) * Self::randomizePosition;
        randY += (rng.gen::<f32>() * 2.0 - 1.0) * Self::randomizePosition;

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
        return "ConeParticleEmitter".to_string();
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
        self.local_matrix = localMatrix;
    }

    fn set_alignDirection(&mut self, alignDirection: bool) {
        self.align_direction = alignDirection;
    }

    fn set_randomizeDirection(&mut self, randomizeDirection: f32) {
        self.randomize_direction = randomizeDirection;
    }

    fn set_spherizeDirection(&mut self, spherizeDirection: f32) {
        self.spherize_direction = spherizeDirection;
    }

    fn set_randomizePosition(&mut self, randomizePosition: f32) {
        self.randomize_position = randomizePosition;
    }

    fn get_localMatrix(&mut self) -> Matrix {
        self.local_matrix.clone()
    }

    fn get_alignDirection(&mut self) -> bool {
        self.align_direction.clone()
    }

    fn get_randomizeDirection(&mut self) -> f32 {
        self.randomize_direction.clone()
    }

    fn get_spherizeDirection(&mut self) -> f32 {
        self.spherize_direction.clone()
    }

    fn get_randomizePosition(&mut self) -> f32 {
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
