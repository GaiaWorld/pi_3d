use pi_render::rhi::{dyn_uniform_buffer::{Uniform, BindOffset, Bind, DynUniformBuffer}, device::RenderDevice};

use crate::{shaders::{FragmentUniformBind}, materials::bytes_write_to_memory};


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
    pub bind_offset: BindOffset,
}
impl SceneFog {
    pub const FOG_PARAM: usize = 4;
    pub const FOG_COLOR: usize = 4;

    pub const FOG_PARAM_OFFSIZE: usize = 0 * 4;
    pub const FOG_COLOR_OFFSIZE: usize = Self::FOG_PARAM_OFFSIZE + Self::FOG_PARAM_OFFSIZE * 4;

    pub fn new(
        dynbuffer: &mut DynUniformBuffer,
    ) -> Self {
        Self {
            mode: EFogMode::None,
            color: (0.1, 0.5, 0.1),
            start: 10.,
            end: 100.,
            intensity: 1.0,
            dirty: true,
            bind_offset: dynbuffer.alloc_binding::<Self>(),
        }
    }

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
        bytes_write_to_memory(bytemuck::cast_slice(&values), index as usize + SceneFog::FOG_COLOR_OFFSIZE, buffer);

        let mode: f32 = match self.mode {
            EFogMode::None => 0.,
            EFogMode::Linear => 1.,
            EFogMode::Exp => 2.,
            EFogMode::Exp2 => 3.,
        };
        let values = vec![mode, self.start, self.end, self.intensity];
        bytes_write_to_memory(bytemuck::cast_slice(&values), index as usize + SceneFog::FOG_PARAM_OFFSIZE, buffer);
    }
}
impl FragmentUniformBind for SceneFog {
    const ID: u32 = 2;
    const SIZE: usize = Self::FOG_COLOR_OFFSIZE + Self::FOG_COLOR * 4;
}
impl Bind for SceneFog {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}


