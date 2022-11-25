use pi_render::rhi::{internal::bytemuck, dyn_uniform_buffer::Uniform};
use pi_scene_context::bytes_write_to_memory;

use crate::{value::{Value, FromValueUniformStatistics, ValueInt, SlotActiveRequire}, uniforms::value_uniform::ValueUniformStatistics};

pub trait UniformInt: ValueInt {
    fn offset(&self) -> usize;
    fn write_into_buffer(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(&[self.value()]), index as usize + self.offset(), buffer);
    }
}

// ==== ==== ==== ==== 1
pub struct IntSlot1 {
    pub value: i32,
    pub offset: usize,
}
impl FromValueUniformStatistics for IntSlot1 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 0 * 1 * 4 }
    }
}
impl ValueInt for IntSlot1 {
    fn value(&self) -> i32 {
        self.value
    }
}
impl UniformInt for IntSlot1 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for IntSlot1 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for IntSlot1 {
    const ASK_SLOT_COUNT: u8 = 1;
}

// ==== ==== ==== ==== 2
pub struct IntSlot2 {
    pub value: i32,
    pub offset: usize,
}
impl FromValueUniformStatistics for IntSlot2 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 1 * 1 * 4 }
    }
}
impl ValueInt for IntSlot2 {
    fn value(&self) -> i32 {
        self.value
    }
}
impl UniformInt for IntSlot2 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for IntSlot2 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for IntSlot2 {
    const ASK_SLOT_COUNT: u8 = 2;
}

// ==== ==== ==== ==== 3
pub struct IntSlot3 {
    pub value: i32,
    pub offset: usize,
}
impl FromValueUniformStatistics for IntSlot3 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 2 * 1 * 4 }
    }
}
impl ValueInt for IntSlot3 {
    fn value(&self) -> i32 {
        self.value
    }
}
impl UniformInt for IntSlot3 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for IntSlot3 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for IntSlot3 {
    const ASK_SLOT_COUNT: u8 = 3;
}

// ==== ==== ==== ==== 1
pub struct IntSlot4 {
    pub value: i32,
    pub offset: usize,
}
impl FromValueUniformStatistics for IntSlot4 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 3 * 1 * 4 }
    }
}
impl ValueInt for IntSlot4 {
    fn value(&self) -> i32 {
        self.value
    }
}
impl UniformInt for IntSlot4 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for IntSlot4 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for IntSlot4 {
    const ASK_SLOT_COUNT: u8 = 4;
}

// ==== ==== ==== ==== 5
pub struct IntSlot5 {
    pub value: i32,
    pub offset: usize,
}
impl FromValueUniformStatistics for IntSlot5 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 4 * 1 * 4 }
    }
}
impl ValueInt for IntSlot5 {
    fn value(&self) -> i32 {
        self.value
    }
}
impl UniformInt for IntSlot5 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for IntSlot5 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for IntSlot5 {
    const ASK_SLOT_COUNT: u8 = 5;
}

// ==== ==== ==== ==== 6
pub struct IntSlot6 {
    pub value: i32,
    pub offset: usize,
}
impl FromValueUniformStatistics for IntSlot6 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 5 * 1 * 4 }
    }
}
impl ValueInt for IntSlot6 {
    fn value(&self) -> i32 {
        self.value
    }
}
impl UniformInt for IntSlot6 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for IntSlot6 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for IntSlot6 {
    const ASK_SLOT_COUNT: u8 = 6;
}

// ==== ==== ==== ==== 7
pub struct IntSlot7 {
    pub value: i32,
    pub offset: usize,
}
impl FromValueUniformStatistics for IntSlot7 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 6 * 1 * 4 }
    }
}
impl ValueInt for IntSlot7 {
    fn value(&self) -> i32 {
        self.value
    }
}
impl UniformInt for IntSlot7 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for IntSlot7 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for IntSlot7 {
    const ASK_SLOT_COUNT: u8 = 7;
}

// ==== ==== ==== ==== 8
pub struct IntSlot8 {
    pub value: i32,
    pub offset: usize,
}
impl FromValueUniformStatistics for IntSlot8 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: 0, offset: value.float_begin as usize + 7 * 1 * 4 }
    }
}
impl ValueInt for IntSlot8 {
    fn value(&self) -> i32 {
        self.value
    }
}
impl UniformInt for IntSlot8 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for IntSlot8 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for IntSlot8 {
    const ASK_SLOT_COUNT: u8 = 8;
}
