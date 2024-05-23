use std::default;

use crate::{iparticle_system_config::{
    TSHAPE_ARC_MODE_BURST_SPREAD, TSHAPE_ARC_MODE_LOOP, TSHAPE_ARC_MODE_PING_PONG,
    TSHAPE_ARC_MODE_RANDOM,
}, tools::Random};
use pi_scene_math::{Matrix, Vector3};

use super::PointShapeEmitter;


pub trait IShapeEmitterTypeValue {
    const POSITION: Vector3;
    const ROTATION: Vector3;
    const SCALING: Vector3;
    const LOCAL_MATRIX: Matrix;
    const ALIGN_DIRECTION: bool;
    const RANDOMIZE_DIRECTION: f32;
    const SPHERIZE_DIRECTION: f32;
    const RANDOMIZE_POSITION: f32;
}

#[derive(Default)]
pub struct ShapeEmitter {
    pub(crate) position: Vector3,
    pub(crate) rotation: Vector3,
    pub(crate) scaling: Vector3,
    pub(crate) local_matrix: Matrix,
    pub(crate) align_direction: bool,
    pub(crate) randomize_direction: f32,
    pub(crate) spherize_direction: f32,
    pub(crate) randomize_position: f32,
}
impl ShapeEmitter {
    pub fn new() -> Self {
        Self { 
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
}


pub struct TypeShapeEmitter {
    pub(crate) box_mode: EBoxShapeMode,
    pub(crate) spawn_point_only: bool,
    pub(crate) arc_mode: EShapeEmitterArcMode,
    pub(crate) base: ShapeEmitter,
    pub(crate) param: Vec<f32>,
    pub(crate) fn_direction: fn(&TypeShapeEmitter, &mut Vector3, &Vector3, &mut Random),
    pub(crate) fn_position: fn(&TypeShapeEmitter, &mut Vector3, f32, f32, f32, f32, &mut Random),
    pub(crate) fn_orbit_center: fn(&Vector3, &Vector3, &mut Vector3),
}

impl Default for TypeShapeEmitter {
    fn default() -> Self {
        Self::new()
    }
}
impl TypeShapeEmitter {
    pub fn new() -> Self {
        Self {
            box_mode: EBoxShapeMode::Volume,
            spawn_point_only: false,
            arc_mode: EShapeEmitterArcMode::Random,
            base: ShapeEmitter::new(),
            param: vec![],
            fn_direction: PointShapeEmitter::start_direction_function,
            fn_position: PointShapeEmitter::start_position_function,
            fn_orbit_center: PointShapeEmitter::orbit_center,
        }
    }
    pub fn orbit_center<'a>(&'a self, local_position: &'a Vector3, offset: &'a Vector3, result: &'a mut Vector3) {
        (self.fn_orbit_center)(local_position, offset, result);
    }
    pub fn start_direction_function(
        &self,
        direction_to_update: & mut Vector3,
        local_position: & Vector3,
        random: & mut Random,
    ) {
        (self.fn_direction)(&self, direction_to_update, local_position, random);
    }
    pub fn start_position_function(
        &self,
        position_to_update: & mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        random: & mut Random,
    ) {
        (self.fn_position)(&self, position_to_update, _emission_loop, _emission_progress, _emission_index, _emission_total, random);
    }
}

/**
 * Shape emitter represents a volume emitting particles.
 * This is the responsibility of the implementation to define the volume shape like cone/sphere/box.
 */
pub trait IShapeEmitterType {
    /**
     * Called by the particle System when the direction is computed for the created particle.
     */
    fn start_direction_function(
        &self,
        direction_to_update: &mut Vector3,
        local_position: &Vector3,
        random: &mut Random,
    );

    /**
     * Called by the particle System when the position is computed for the created particle.
     * @param worldMatrix
     * @param positionToUpdate
     * @param localPosition
     * @param emissionLoop 当前发射事件所属的发射循环数
     * @param emissionProgress 当前发射事件的时间进度
     * @param emissionIndex 目标在此次发射中的序号
     * @param esmissionTotal 此次发射的总量
     */
    fn start_position_function(
        &self,
        position_to_update: &mut Vector3,
        emission_loop: f32,
        emission_progress: f32,
        emission_index: f32,
        emission_total: f32,
        random: &mut Random,
    );

    /**
     * Returns a string representing the class name
     * @returns a string containing the class name
     */
    fn get_class_name() -> String
    where
        Self: Sized;

    fn dispose()
    where
        Self: Sized;

    fn set_position(&mut self, position: Vector3);
    fn set_rotation(&mut self, rotation: Vector3);
    fn set_scaling(&mut self, scaling: Vector3);
    fn set_local_matrix(&mut self, local_matrix: Matrix);
    fn set_align_direction(&mut self, align_direction: bool);
    fn set_randomize_direction(&mut self, randomize_direction: f32);
    fn set_spherize_direction(&mut self, spherize_direction: f32);
    fn set_randomize_position(&mut self, randomize_position: f32);

    fn get_postion(&self) -> Vector3;
    fn get_rotation(&self) -> Vector3;
    fn get_scaling(&self) -> Vector3;
    fn get_local_matrix(&self) -> Matrix;
    fn get_align_direction(&self) -> bool;
    fn get_randomize_direction(&self) -> f32;
    fn get_spherize_direction(&self) -> f32;
    fn get_randomize_position(&self) -> f32;
}

#[derive(Clone, Copy)]
pub enum EShapeEmitterArcMode {
    /**
     * 弧形周围随机生成粒子
     */
    Random = TSHAPE_ARC_MODE_RANDOM,
    /**
     * 形状的弧形周围依序生成粒子，并在每个周期结束时循环回到起点
     */
    Loop = TSHAPE_ARC_MODE_LOOP,
    /**
     * 每个连续循环的发生方向与上一个循环相反
     */
    PingPong = TSHAPE_ARC_MODE_PING_PONG,
    /**
     * 在形状周围均匀分布粒子生成位置
     */
    BurstsSpread = TSHAPE_ARC_MODE_BURST_SPREAD,
}

impl Default for EShapeEmitterArcMode {
    fn default() -> Self {
        Self::Random
    }
}
/**
 * 形状发射器创建模式
 */
#[derive(PartialEq, Eq)]
pub enum EShapeEmitterDirectionMode {
    Unity = 0,
}

#[derive(Clone, Copy, Default)]
pub enum EBoxShapeMode {
    #[default]
    Volume = 0,
    Shell = 1,
    Edge = 2,
}

pub fn compute_radians(
    emission_loop: f32,
    emission_progress: f32,
    emission_index: f32,
    emission_total: f32,
    arc_total_value: f32,
    arc_value: f32,
    arc_spread: f32,
    arc_speed: f32,
    arc_mode: EShapeEmitterArcMode,
    random: &mut Random,
) -> f32 {
    let arc_spread_limit = 0.001;
    let mut _s = 0.;
    let mut spread = arc_spread;
    let mut emission_progress = emission_progress * arc_speed;

    match arc_mode {
        EShapeEmitterArcMode::Loop => {
            let radians = (emission_loop + emission_progress) * arc_total_value;
            // let loopCount = f32::floor(radians / arcValue);
            _s = radians % arc_value;

            if spread > arc_spread_limit {
                spread = spread * arc_value;

                _s = f32::round(_s / spread) * spread;
            }
        }
        EShapeEmitterArcMode::PingPong => {
            let radians = (emission_loop + emission_progress) * arc_total_value;
            let loop_count = f32::floor(radians / arc_value);
            _s = radians % arc_value;

            if spread > arc_spread_limit {
                spread = spread * arc_value;

                _s = f32::round(_s / spread) * spread;
            }

            if loop_count % 2. == 1. {
                _s = arc_value - _s;
            }
        }
        EShapeEmitterArcMode::BurstsSpread => {
            emission_progress = emission_index / emission_total;
            if spread > arc_spread_limit {
                spread = spread * arc_value;

                emission_progress = f32::round(emission_progress / spread) * spread;
            }

            _s = arc_value * emission_progress;
        }
        _ => {
            let random: f32 = random.random();
            _s = 0. + random * (arc_value - 0.);
        }
    }

    return _s;
}
