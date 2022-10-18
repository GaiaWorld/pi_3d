use pi_ecs_utils::system_param::tree::EntityTree;
use pi_idtree::IdTree;
use pi_render::rhi::dyn_uniform_buffer::Uniform;
use pi_scene_math::coordiante_system;

use crate::{materials::bytes_write_to_memory, shaders::{buildin_uniforms::BuildinTimeBind}};

pub struct SceneParam {
    pub coordsys: coordiante_system::CoordinateSytem3,
}


pub struct SceneTime {
    pub time: f32,
    pub delta_tims: f32,
}
impl Uniform for SceneTime {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let time = vec![self.time, 1. / self.time, self.time.sin(), self.time.cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + BuildinTimeBind::TIME_OFFSIZE, buffer);

        let time = vec![self.delta_tims, 1. / self.delta_tims, self.delta_tims.sin(), self.delta_tims.cos()];
        bytes_write_to_memory(bytemuck::cast_slice(&time), index as usize + BuildinTimeBind::DELTA_TIME_OFFSIZE, buffer);
    }
}