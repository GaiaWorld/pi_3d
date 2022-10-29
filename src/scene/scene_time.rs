use pi_ecs_utils::system_param::tree::EntityTree;
use pi_idtree::IdTree;
use pi_render::rhi::dyn_uniform_buffer::{Uniform, DynUniformBuffer, Bind, BindOffset};
use pi_scene_math::coordiante_system;

use crate::{shaders::{FragmentUniformBind}, bytes_write_to_memory};



pub struct SceneTime {
    pub time: f32,
    pub delta_tims: f32,
    pub bind_offset: BindOffset,
}
impl SceneTime {
    pub const TIME: usize = 4;
    pub const DELTA_TIME: usize = 4;

    pub const TIME_OFFSIZE: usize = 0 * 4;
    pub const DELTA_TIME_OFFSIZE: usize = Self::TIME_OFFSIZE + Self::TIME * 4;

    pub fn new(
        dynbuffer: &mut DynUniformBuffer,
    ) -> Self {
        Self {
            time: 0.,
            delta_tims: 0.,
            bind_offset: dynbuffer.alloc_binding::<Self>(),
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
impl FragmentUniformBind for SceneTime {
    const ID: u32 = 1;
    const SIZE: usize = Self::DELTA_TIME_OFFSIZE + Self::DELTA_TIME * 4;
}
impl Bind for SceneTime {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}