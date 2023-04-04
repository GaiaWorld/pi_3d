
use pi_ecs::prelude::{Query, Res};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::GameObject, frame_time::SingleFrameTimeCommand, run_stage::TSystemStageInfo};
use pi_render::{rhi::{shader::WriteBuffer}, render_3d::binds::scene::{effect::ShaderBindSceneAboutEffect}};

use crate::{bytes_write_to_memory,};

use super::BindSceneEffect;

pub struct SceneTime {
    pub time_ms: u64,
    pub delta_ms: u64,
    pub dirty: bool,
}
impl SceneTime {
    pub const TIME: usize = 4;
    pub const DELTA_TIME: usize = 4;

    pub const TIME_OFFSIZE: usize = 0 * 4;
    pub const DELTA_TIME_OFFSIZE: usize = Self::TIME_OFFSIZE + Self::TIME * 4;

    pub fn new() -> Self {
        Self {
            time_ms: 0,
            delta_ms: 1,
            dirty: true,
        }
    }

    pub fn reset(&mut self, delta_ms: u64) {
        self.time_ms += delta_ms;
        self.delta_ms = delta_ms;
        self.dirty = true;
    }
    pub fn data(&self, data: &mut Vec<f32>) {
        let temp = [
            self.time_ms as f32, 1. / (self.time_ms as f32), (self.time_ms as f32).sin(), (self.time_ms as f32).cos(),
            self.delta_ms as f32, 1. / (self.delta_ms as f32), (self.delta_ms as f32).sin(), (self.delta_ms as f32).cos()
        ];
        temp.iter().for_each(|v| {
            data.push(*v);
        });
    }
    pub fn update(&self, bind: &BindSceneEffect) {
        let values = [
            self.time_ms as f32, 1. / (self.time_ms as f32), (self.time_ms as f32).sin(), (self.time_ms as f32).cos(),
            self.delta_ms as f32, 1. / (self.delta_ms as f32), (self.delta_ms as f32).sin(), (self.delta_ms as f32).cos()
        ];
        bind.0.data().write_data(ShaderBindSceneAboutEffect::OFFSET_TIME as usize, bytemuck::cast_slice(&values));
    }
}
impl WriteBuffer for SceneTime {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let time = vec![self.time_ms as f32, 1. / (self.time_ms as f32), (self.time_ms as f32).sin(), (self.time_ms as f32).cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + Self::TIME_OFFSIZE, buffer);

        let time = vec![self.delta_ms as f32, 1. / (self.delta_ms as f32), (self.delta_ms as f32).sin(), (self.delta_ms as f32).cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + Self::DELTA_TIME_OFFSIZE, buffer);
    }

    fn byte_len(&self) -> u32 {
        32
    }

    fn offset(&self) -> u32 {
        0
    }

}

pub struct SysSceneTimeUpdate;
impl TSystemStageInfo for SysSceneTimeUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysSceneCreateCommand::key()
        ]
    }
}
#[setup]
impl SysSceneTimeUpdate {
    #[system]
    fn sys(
        mut scenes: Query<GameObject, (&mut SceneTime, &mut BindSceneEffect)>,
        frame: Res<SingleFrameTimeCommand>,
    ) {
        scenes.iter_mut().for_each(|(mut scene_time, mut bind)| {
            scene_time.reset(frame.frame_ms);
            scene_time.update(&mut bind);
        });
    }
}