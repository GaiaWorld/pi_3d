use pi_render::rhi::{internal::bytemuck, dyn_uniform_buffer::Uniform};
use pi_scene_context::bytes_write_to_memory;

use crate::{value::{Value, FromValueUniformStatistics, ValueUint, SlotActiveRequire}, uniforms::value_uniform::ValueUniformStatistics};

pub trait UniformUint: ValueUint {
    fn offset(&self) -> usize;
    fn write_into_buffer(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(&[self.value()]), index as usize + self.offset(), buffer);
    }
}

// ==== ==== ==== ==== 1
pub struct UintSlot1 {
    pub value: u32,
    pub offset: usize,
}
impl FromValueUniformStatistics for UintSlot1 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 0 * 1 * 4 }
    }
}
impl ValueUint for UintSlot1 {
    fn value(&self) -> u32 {
        self.value
    }
}
impl UniformUint for UintSlot1 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for UintSlot1 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for UintSlot1 {
    const ASK_SLOT_COUNT: u8 = 1;
}

// ==== ==== ==== ==== 2
pub struct UintSlot2 {
    pub value: u32,
    pub offset: usize,
}
impl FromValueUniformStatistics for UintSlot2 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 1 * 1 * 4 }
    }
}
impl ValueUint for UintSlot2 {
    fn value(&self) -> u32 {
        self.value
    }
}
impl UniformUint for UintSlot2 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for UintSlot2 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for UintSlot2 {
    const ASK_SLOT_COUNT: u8 = 2;
}

// ==== ==== ==== ==== 3
pub struct UintSlot3 {
    pub value: u32,
    pub offset: usize,
}
impl FromValueUniformStatistics for UintSlot3 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 2 * 1 * 4 }
    }
}
impl ValueUint for UintSlot3 {
    fn value(&self) -> u32 {
        self.value
    }
}
impl UniformUint for UintSlot3 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for UintSlot3 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for UintSlot3 {
    const ASK_SLOT_COUNT: u8 = 3;
}

// ==== ==== ==== ==== 1
pub struct UintSlot4 {
    pub value: u32,
    pub offset: usize,
}
impl FromValueUniformStatistics for UintSlot4 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 3 * 1 * 4 }
    }
}
impl ValueUint for UintSlot4 {
    fn value(&self) -> u32 {
        self.value
    }
}
impl UniformUint for UintSlot4 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for UintSlot4 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for UintSlot4 {
    const ASK_SLOT_COUNT: u8 = 4;
}

// ==== ==== ==== ==== 5
pub struct UintSlot5 {
    pub value: u32,
    pub offset: usize,
}
impl FromValueUniformStatistics for UintSlot5 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 4 * 1 * 4 }
    }
}
impl ValueUint for UintSlot5 {
    fn value(&self) -> u32 {
        self.value
    }
}
impl UniformUint for UintSlot5 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for UintSlot5 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for UintSlot5 {
    const ASK_SLOT_COUNT: u8 = 5;
}

// ==== ==== ==== ==== 6
pub struct UintSlot6 {
    pub value: u32,
    pub offset: usize,
}
impl FromValueUniformStatistics for UintSlot6 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 5 * 1 * 4 }
    }
}
impl ValueUint for UintSlot6 {
    fn value(&self) -> u32 {
        self.value
    }
}
impl UniformUint for UintSlot6 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for UintSlot6 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for UintSlot6 {
    const ASK_SLOT_COUNT: u8 = 6;
}

// ==== ==== ==== ==== 7
pub struct UintSlot7 {
    pub value: u32,
    pub offset: usize,
}
impl FromValueUniformStatistics for UintSlot7 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 6 * 1 * 4 }
    }
}
impl ValueUint for UintSlot7 {
    fn value(&self) -> u32 {
        self.value
    }
}
impl UniformUint for UintSlot7 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for UintSlot7 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for UintSlot7 {
    const ASK_SLOT_COUNT: u8 = 7;
}

// ==== ==== ==== ==== 8
pub struct UintSlot8 {
    pub value: u32,
    pub offset: usize,
}
impl FromValueUniformStatistics for UintSlot8 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 7 * 1 * 4 }
    }
}
impl ValueUint for UintSlot8 {
    fn value(&self) -> u32 {
        self.value
    }
}
impl UniformUint for UintSlot8 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for UintSlot8 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for UintSlot8 {
    const ASK_SLOT_COUNT: u8 = 8;
}
