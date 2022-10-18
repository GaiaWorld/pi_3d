use pi_render::rhi::dyn_uniform_buffer::Uniform;

use crate::{materials::bytes_write_to_memory, shaders::buildin_uniforms::BuildinAmbientLightBind};


pub struct AmbientLight {
    color: (f32, f32, f32),
    intensity: f32,
    pub dirty: bool,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: (0., 0., 0.),
            intensity: 1.,
            dirty: true,
        }
    }
}
impl AmbientLight {
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
        bytes_write_to_memory(bytemuck::cast_slice(&value), index as usize + BuildinAmbientLightBind::AMBIENT_LIGHT_OFFSIZE, buffer);
    }
}
