use crate::iparticle_system_config::{
    TShapeArcModeBurstSpread, TShapeArcModeLoop, TShapeArcModePingPong, TShapeArcModeRandom,
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
    fn set_localMatrix(&mut self, localMatrix: Matrix);
    fn set_alignDirection(&mut self, alignDirection: bool);
    fn set_randomizeDirection(&mut self, randomizeDirection: f32);
    fn set_spherizeDirection(&mut self, spherizeDirection: f32);
    fn set_randomizePosition(&mut self, randomizePosition: f32);

    fn get_postion(&self) -> Vector3;
    fn get_rotation(&self) -> Vector3;
    fn get_scaling(&self) -> Vector3;
    fn get_localMatrix(&mut self) -> Matrix;
    fn get_alignDirection(&mut self) -> bool;
    fn get_randomizeDirection(&mut self, ) -> f32;
    fn get_spherizeDirection(&mut self) -> f32;
    fn get_randomizePosition(&mut self) -> f32;
}

#[derive(Clone, Copy)]
pub enum EShapeEmitterArcMode {
    /**
     * 弧形周围随机生成粒子
     */
    Random = TShapeArcModeRandom,
    /**
     * 形状的弧形周围依序生成粒子，并在每个周期结束时循环回到起点
     */
    Loop = TShapeArcModeLoop,
    /**
     * 每个连续循环的发生方向与上一个循环相反
     */
    PingPong = TShapeArcModePingPong,
    /**
     * 在形状周围均匀分布粒子生成位置
     */
    BurstsSpread = TShapeArcModeBurstSpread,
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
    let mut s = 0.;
    let mut spread = arc_spread;
    let mut emission_progress = emission_progress * arc_speed;

    match arc_mode {
        EShapeEmitterArcMode::Loop => {
            let radians = (emission_loop + emission_progress) * arc_total_value;
            // let loopCount = f32::floor(radians / arcValue);
            s = radians % arc_value;

            if spread > arc_spread_limit {
                spread = spread * arc_value;

                s = f32::round(s / spread) * spread;
            }
        }
        EShapeEmitterArcMode::PingPong => {
            let radians = (emission_loop + emission_progress) * arc_total_value;
            let loop_count = f32::floor(radians / arc_value);
            s = radians % arc_value;

            if spread > arc_spread_limit {
                spread = spread * arc_value;

                s = f32::round(s / spread) * spread;
            }

            if loop_count % 2. == 1. {
                s = arc_value - s;
            }
        }
        EShapeEmitterArcMode::BurstsSpread => {
            emission_progress = emission_index / emission_total;
            if spread > arc_spread_limit {
                spread = spread * arc_value;

                emission_progress = f32::round(emission_progress / spread) * spread;
            }

            s = arc_value * emission_progress;
        }
        _ => {
            let mut rng = rand::thread_rng();
            let random: f32 = rng.gen();

            s = 0. + random * (arc_value - 0.);
        }
    }

    return s;
}
