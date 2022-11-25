use pi_render::rhi::{internal::bytemuck, dyn_uniform_buffer::Uniform};
use pi_scene_context::bytes_write_to_memory;

use crate::{value::{FromValueUniformStatistics, ValueFloat, SlotActiveRequire}, uniforms::value_uniform::ValueUniformStatistics};

pub trait UniformFloat: ValueFloat {
    fn offset(&self) -> usize;
    fn write_into_buffer(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(&[self.value()]), index as usize + self.offset(), buffer);
    }
}

// ==== ==== ==== ==== 1
pub struct FloatSlot1 {
    pub value: f32,
    pub offset: usize,
}
impl FromValueUniformStatistics for FloatSlot1 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0., offset: value.float_begin as usize + 0 * 1 * 4 }
    }
}
impl ValueFloat for FloatSlot1 {
    fn value(&self) -> f32 {
        self.value
    }
}
impl UniformFloat for FloatSlot1 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for FloatSlot1 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for FloatSlot1 {
    const ASK_SLOT_COUNT: u8 = 1;
}

// ==== ==== ==== ==== 2
pub struct FloatSlot2 {
    pub value: f32,
    pub offset: usize,
}
impl FromValueUniformStatistics for FloatSlot2 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0., offset: value.float_begin as usize + 1 * 1 * 4 }
    }
}
impl ValueFloat for FloatSlot2 {
    fn value(&self) -> f32 {
        self.value
    }
}
impl UniformFloat for FloatSlot2 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for FloatSlot2 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for FloatSlot2 {
    const ASK_SLOT_COUNT: u8 = 2;
}

// ==== ==== ==== ==== 3
pub struct FloatSlot3 {
    pub value: f32,
    pub offset: usize,
}
impl FromValueUniformStatistics for FloatSlot3 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0., offset: value.float_begin as usize + 2 * 1 * 4 }
    }
}
impl ValueFloat for FloatSlot3 {
    fn value(&self) -> f32 {
        self.value
    }
}
impl UniformFloat for FloatSlot3 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for FloatSlot3 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for FloatSlot3 {
    const ASK_SLOT_COUNT: u8 = 3;
}

// ==== ==== ==== ==== 1
pub struct FloatSlot4 {
    pub value: f32,
    pub offset: usize,
}
impl FromValueUniformStatistics for FloatSlot4 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0., offset: value.float_begin as usize + 3 * 1 * 4 }
    }
}
impl ValueFloat for FloatSlot4 {
    fn value(&self) -> f32 {
        self.value
    }
}
impl UniformFloat for FloatSlot4 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for FloatSlot4 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for FloatSlot4 {
    const ASK_SLOT_COUNT: u8 = 4;
}

// ==== ==== ==== ==== 5
pub struct FloatSlot5 {
    pub value: f32,
    pub offset: usize,
}
impl FromValueUniformStatistics for FloatSlot5 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0., offset: value.float_begin as usize + 4 * 1 * 4 }
    }
}
impl ValueFloat for FloatSlot5 {
    fn value(&self) -> f32 {
        self.value
    }
}
impl UniformFloat for FloatSlot5 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for FloatSlot5 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for FloatSlot5 {
    const ASK_SLOT_COUNT: u8 = 5;
}

// ==== ==== ==== ==== 6
pub struct FloatSlot6 {
    pub value: f32,
    pub offset: usize,
}
impl FromValueUniformStatistics for FloatSlot6 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0., offset: value.float_begin as usize + 5 * 1 * 4 }
    }
}
impl ValueFloat for FloatSlot6 {
    fn value(&self) -> f32 {
        self.value
    }
}
impl UniformFloat for FloatSlot6 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for FloatSlot6 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for FloatSlot6 {
    const ASK_SLOT_COUNT: u8 = 6;
}

// ==== ==== ==== ==== 7
pub struct FloatSlot7 {
    pub value: f32,
    pub offset: usize,
}
impl FromValueUniformStatistics for FloatSlot7 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0., offset: value.float_begin as usize + 6 * 1 * 4 }
    }
}
impl ValueFloat for FloatSlot7 {
    fn value(&self) -> f32 {
        self.value
    }
}
impl UniformFloat for FloatSlot7 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for FloatSlot7 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for FloatSlot7 {
    const ASK_SLOT_COUNT: u8 = 7;
}

// ==== ==== ==== ==== 8
pub struct FloatSlot8 {
    pub value: f32,
    pub offset: usize,
}
impl FromValueUniformStatistics for FloatSlot8 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0., offset: value.float_begin as usize + 7 * 1 * 4 }
    }
}
impl ValueFloat for FloatSlot8 {
    fn value(&self) -> f32 {
        self.value
    }
}
impl UniformFloat for FloatSlot8 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for FloatSlot8 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for FloatSlot8 {
    const ASK_SLOT_COUNT: u8 = 8;
}
