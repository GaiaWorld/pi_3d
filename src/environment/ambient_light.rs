use pi_render::rhi::dyn_uniform_buffer::{Uniform, Bind, DynUniformBuffer, BindOffset};

use crate::{materials::bytes_write_to_memory, shaders::{FragmentUniformBind}};


pub struct AmbientLight {
    color: (f32, f32, f32),
    intensity: f32,
    pub dirty: bool,
    pub bind_offset: BindOffset,
}
impl AmbientLight {
    pub const AMBIENT_LIGHT: usize = 4;
    pub const AMBIENT_LIGHT_OFFSIZE: usize = 0 * 4;

    pub fn new(
        dynbuffer: &mut DynUniformBuffer,
    ) -> Self {
        Self {
            color: (1., 1., 1.),
            intensity: 1.0,
            dirty: true,
            bind_offset: dynbuffer.alloc_binding::<Self>(),
        }
    }
    pub fn color(&mut self, value: (f32, f32, f32)) {
        if self.color.0 != value.0 || self.color.1 != value.1 || self.color.2 != value.2 {
            self.dirty = true;
            self.color = value;
        }
    }
    pub fn intensity(&mut self, value: f32) {
        if self.intensity != value {
            self.dirty = true;
            self.intensity = value;
        }
    }
}
impl Uniform for AmbientLight {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let value = vec![self.color.0, self.color.1, self.color.2, self.intensity];
        bytes_write_to_memory(bytemuck::cast_slice(&value), index as usize + Self::AMBIENT_LIGHT_OFFSIZE, buffer);
    }
}
impl FragmentUniformBind for AmbientLight {
    const ID: u32 = 3;
    const SIZE: usize = Self::AMBIENT_LIGHT_OFFSIZE + Self::AMBIENT_LIGHT * 4;
}
impl Bind for AmbientLight {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
