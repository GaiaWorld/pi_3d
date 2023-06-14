use crate::iparticle_system_config::{
    TSHAPE_ARC_MODE_BURST_SPREAD, TSHAPE_ARC_MODE_LOOP, TSHAPE_ARC_MODE_PING_PONG, TSHAPE_ARC_MODE_RANDOM,
};
use pi_scene_math::{Matrix, Vector3};
use rand::Rng;
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
        world_matrix: Matrix,
        direction_to_update: &mut Vector3,
        position: Vector3,
        local_position: Vector3,
        is_local: bool,
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
        world_matrix: Matrix,
        position_to_update: &mut Vector3,
        emission_loop: f32,
        emission_progress: f32,
        emission_index: f32,
        emission_total: f32,
        is_local: bool,
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

    fn set_postion(&mut self, position: Vector3);
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
    fn get_local_matrix(&mut self) -> Matrix;
    fn get_align_direction(&mut self) -> bool;
    fn get_randomize_direction(&mut self, ) -> f32;
    fn get_spherize_direction(&mut self) -> f32;
    fn get_randomize_position(&mut self) -> f32;
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
/**
 * 形状发射器创建模式
 */
#[derive(Debug, PartialEq, Eq)]
pub enum EShapeEmitterDirectionMode {
    Unity = 0,
}

#[derive(Clone, Copy)]
pub enum EBoxShapeMode {
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
            let mut rng = rand::thread_rng();
            let random: f32 = rng.gen();

            _s = 0. + random * (arc_value - 0.);
        }
    }

    return _s;
}
