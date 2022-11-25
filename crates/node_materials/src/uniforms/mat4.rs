
use pi_render::rhi::{dyn_uniform_buffer::Uniform, internal::bytemuck};
use pi_scene_context::{resources::RenderDynUniformBuffer, bytes_write_to_memory};
use pi_scene_math::Matrix;

use crate::{value::{Value, ValueSlotIndex, FromValueUniformStatistics, ValueMat4, SlotActiveRequire}, uniforms::value_uniform::ValueUniformStatistics};


pub trait UniformMat4: ValueMat4 {
    fn offset(&self) -> usize;
    fn write_into_buffer(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.value().as_slice()), index as usize + self.offset(), buffer);
    }
}

// ==== ==== ==== ==== 1
pub struct Mat4Slot1 {
    pub value: Matrix,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat4Slot1 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix::identity(), offset: value.mat4_begin as usize + 0 * 16 * 4 }
    }
}
impl ValueMat4 for Mat4Slot1 {
    fn value(&self) -> Matrix {
        self.value
    }
}
impl UniformMat4 for Mat4Slot1 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat4Slot1 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat4Slot1 {
    const ASK_SLOT_COUNT: u8 = 1;
}

// ==== ==== ==== ==== 2
pub struct Mat4Slot2 {
    pub value: Matrix,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat4Slot2 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix::identity(), offset: value.mat4_begin as usize + 1 * 16 * 4 }
    }
}
impl ValueMat4 for Mat4Slot2 {
    fn value(&self) -> Matrix {
        self.value
    }
}
impl UniformMat4 for Mat4Slot2 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat4Slot2 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat4Slot2 {
    const ASK_SLOT_COUNT: u8 = 2;
}

// ==== ==== ==== ==== 3
pub struct Mat4Slot3 {
    pub value: Matrix,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat4Slot3 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix::identity(), offset: value.mat4_begin as usize + 2 * 16 * 4 }
    }
}
impl ValueMat4 for Mat4Slot3 {
    fn value(&self) -> Matrix {
        self.value
    }
}
impl UniformMat4 for Mat4Slot3 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat4Slot3 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat4Slot3 {
    const ASK_SLOT_COUNT: u8 = 3;
}

// ==== ==== ==== ==== 1
pub struct Mat4Slot4 {
    pub value: Matrix,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat4Slot4 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix::identity(), offset: value.mat4_begin as usize + 3 * 16 * 4 }
    }
}
impl ValueMat4 for Mat4Slot4 {
    fn value(&self) -> Matrix {
        self.value
    }
}
impl UniformMat4 for Mat4Slot4 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat4Slot4 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat4Slot4 {
    const ASK_SLOT_COUNT: u8 = 4;
}

// ==== ==== ==== ==== 5
pub struct Mat4Slot5 {
    pub value: Matrix,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat4Slot5 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix::identity(), offset: value.mat4_begin as usize + 4 * 16 * 4 }
    }
}
impl ValueMat4 for Mat4Slot5 {
    fn value(&self) -> Matrix {
        self.value
    }
}
impl UniformMat4 for Mat4Slot5 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat4Slot5 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat4Slot5 {
    const ASK_SLOT_COUNT: u8 = 5;
}

// ==== ==== ==== ==== 6
pub struct Mat4Slot6 {
    pub value: Matrix,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat4Slot6 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix::identity(), offset: value.mat4_begin as usize + 5 * 16 * 4 }
    }
}
impl ValueMat4 for Mat4Slot6 {
    fn value(&self) -> Matrix {
        self.value
    }
}
impl UniformMat4 for Mat4Slot6 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat4Slot6 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat4Slot6 {
    const ASK_SLOT_COUNT: u8 = 6;
}

// ==== ==== ==== ==== 7
pub struct Mat4Slot7 {
    pub value: Matrix,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat4Slot7 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix::identity(), offset: value.mat4_begin as usize + 6 * 16 * 4 }
    }
}
impl ValueMat4 for Mat4Slot7 {
    fn value(&self) -> Matrix {
        self.value
    }
}
impl UniformMat4 for Mat4Slot7 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat4Slot7 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat4Slot7 {
    const ASK_SLOT_COUNT: u8 = 7;
}

// ==== ==== ==== ==== 8
pub struct Mat4Slot8 {
    pub value: Matrix,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat4Slot8 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix::identity(), offset: value.mat4_begin as usize + 7 * 16 * 4 }
    }
}
impl ValueMat4 for Mat4Slot8 {
    fn value(&self) -> Matrix {
        self.value
    }
}
impl UniformMat4 for Mat4Slot8 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat4Slot8 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat4Slot8 {
    const ASK_SLOT_COUNT: u8 = 8;
}
