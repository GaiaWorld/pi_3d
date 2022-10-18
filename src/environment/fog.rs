use pi_render::rhi::dyn_uniform_buffer::Uniform;

use crate::{shaders::buildin_uniforms::{BuildinTimeBind, BuildinFogBind}, materials::bytes_write_to_memory};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EFogMode {
    None,
    Linear,
    Exp,
    Exp2,
}

pub struct SceneFog {
    mode: EFogMode,
    color: (f32, f32, f32),
    start: f32,
    end: f32,
    intensity: f32,
    pub dirty: bool,
}
impl Default for SceneFog {
    fn default() -> Self {
        Self {
            mode: EFogMode::None,
            color: (0.1, 0.5, 0.1),
            start: 10.,
            end: 100.,
            intensity: 1.0,
            dirty: true,
        }
    }
}
impl SceneFog {
    pub fn mode(&mut self, mode: EFogMode) {
        if self.mode == mode {

        } else {
            self.dirty = true;
            self.mode = mode;
        }
    }
    pub fn color(&mut self, value: (f32, f32, f32)) {
        if self.color.0 != value.0 || self.color.1 != value.1 || self.color.2 != value.2 {
            self.dirty = true;
            self.color = value;
        }
    }
    pub fn start(&mut self, value: f32) {
        if self.start != value {
            self.dirty = true;
            self.start = value;
        }
    }
    pub fn end(&mut self, value: f32) {
        if self.end != value {
            self.dirty = true;
            self.end = value;
        }
    }
    pub fn intensity(&mut self, value: f32) {
        if self.intensity != value {
            self.dirty = true;
            self.intensity = value;
        }
    }
}
impl Uniform for SceneFog {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let values = vec![self.color.0, self.color.1, self.color.2, 1.];
        bytes_write_to_memory(bytemuck::cast_slice(&values), index as usize + BuildinFogBind::FOG_COLOR_OFFSIZE, buffer);

        let mode: f32 = match self.mode {
            EFogMode::None => 0.,
            EFogMode::Linear => 1.,
            EFogMode::Exp => 2.,
            EFogMode::Exp2 => 3.,
        };
        let values = vec![mode, self.start, self.end, self.intensity];
        bytes_write_to_memory(bytemuck::cast_slice(&values), index as usize + BuildinFogBind::FOG_PARAM_OFFSIZE, buffer);
    }
}

