
use pi_engine_shell::{prelude::*, frame_time::SingleFrameTimeCommand};

use crate::{bytes_write_to_memory,};

use super::BindSceneEffect;

#[derive(Component)]
pub struct SceneTime {
    pub time_ms: u64,
    pub delta_ms: u64,
}
impl SceneTime {
    pub const TIME: usize = 4;
    pub const DELTA_TIME: usize = 4;

    pub const TIME_OFFSIZE: usize = 0 * 4;
    pub const DELTA_TIME_OFFSIZE: usize = Self::TIME_OFFSIZE + Self::TIME * 4;

    pub fn new() -> Self {
        Self {
            time_ms: 0,
            delta_ms: 0,
        }
    }

    pub fn reset(&mut self, delta_ms: u64) {
        self.time_ms += delta_ms;
        self.delta_ms = delta_ms;
    }
    // pub fn data(&self, data: &mut Vec<f32>) {
    //     let time_ms = self.time_ms as f32 * 0.001;
    //     let delta_ms = self.delta_ms as f32 * 0.001;
    //     let temp = [
    //         time_ms, time_ms, time_ms.sin(), time_ms.cos(),
    //         delta_ms, 1. / delta_ms, delta_ms.sin(), delta_ms.cos()
    //     ];
    //     temp.iter().for_each(|v| {
    //         data.push(*v);
    //     });
    // }
    pub fn update(&mut self, bind: &BindSceneEffect) {
        let time_ms = self.time_ms as f32 * 0.001;
        let delta_ms = self.delta_ms as f32 * 0.001;
        let values = [
            time_ms, time_ms, time_ms.sin(), time_ms.cos(),
            delta_ms, 1. / delta_ms, delta_ms.sin(), delta_ms.cos()
        ];
        bind.0.data().write_data(ShaderBindSceneAboutEffect::OFFSET_TIME as usize, bytemuck::cast_slice(&values));

        self.delta_ms = 0;
    }
}
impl WriteBuffer for SceneTime {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let time_ms = self.time_ms as f32 * 0.001;
        let delta_ms = self.delta_ms as f32 * 0.001;

        let time = vec![time_ms, time_ms, time_ms.sin(), time_ms.cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + Self::TIME_OFFSIZE, buffer);

        let time = vec![delta_ms, 1. / delta_ms, delta_ms.sin(), delta_ms.cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + Self::DELTA_TIME_OFFSIZE, buffer);
    }

    fn byte_len(&self) -> u32 {
        32
    }

    fn offset(&self) -> u32 {
        0
    }

}
