
use pi_render::rhi::dyn_uniform_buffer::{Uniform};

use crate::{bytes_write_to_memory, };

pub struct SceneTime {
    pub time: f32,
    pub delta_tims: f32,
    pub dirty: bool,
}
impl SceneTime {
    pub const TIME: usize = 4;
    pub const DELTA_TIME: usize = 4;

    pub const TIME_OFFSIZE: usize = 0 * 4;
    pub const DELTA_TIME_OFFSIZE: usize = Self::TIME_OFFSIZE + Self::TIME * 4;

    pub fn new() -> Self {
        Self {
            time: 0.,
            delta_tims: 0.,
            dirty: true,
        }
    }
}
impl Uniform for SceneTime {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let time = vec![self.time, 1. / self.time, self.time.sin(), self.time.cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + Self::TIME_OFFSIZE, buffer);

        let time = vec![self.delta_tims, 1. / self.delta_tims, self.delta_tims.sin(), self.delta_tims.cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + Self::DELTA_TIME_OFFSIZE, buffer);
    }

}