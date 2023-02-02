
use pi_render::rhi::dyn_uniform_buffer::{Uniform};

use crate::{bytes_write_to_memory, };

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

    pub fn update(&mut self, delta_ms: u64) {
        self.time_ms += delta_ms;
        self.delta_ms = delta_ms;
        self.dirty = true;
    }
}
impl Uniform for SceneTime {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let time = vec![self.time_ms as f32, 1. / (self.time_ms as f32), (self.time_ms as f32).sin(), (self.time_ms as f32).cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + Self::TIME_OFFSIZE, buffer);

        let time = vec![self.delta_ms as f32, 1. / (self.delta_ms as f32), (self.delta_ms as f32).sin(), (self.delta_ms as f32).cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + Self::DELTA_TIME_OFFSIZE, buffer);
    }

}