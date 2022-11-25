
use pi_render::rhi::{internal::bytemuck, dyn_uniform_buffer::Uniform};
use pi_scene_context::bytes_write_to_memory;
use pi_scene_math::Matrix2;

use crate::{value::{Value, FromValueUniformStatistics, ValueMat2, SlotActiveRequire}, uniforms::value_uniform::ValueUniformStatistics};

pub trait UniformMat2: ValueMat2 {
    fn offset(&self) -> usize;
    fn write_into_buffer(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.value().as_slice()), index as usize + self.offset(), buffer);
    }
}

// ==== ==== ==== ==== 1
pub struct Mat2Slot1 {
    pub value: Matrix2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat2Slot1 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix2::identity(), offset: value.mat2_begin as usize + 0 * 4 * 4 }
    }
}
impl ValueMat2 for Mat2Slot1 {
    fn value(&self) -> Matrix2 {
        self.value
    }
}
impl UniformMat2 for Mat2Slot1 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat2Slot1 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat2Slot1 {
    const ASK_SLOT_COUNT: u8 = 1;
}

// ==== ==== ==== ==== 2
pub struct Mat2Slot2 {
    pub value: Matrix2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat2Slot2 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix2::identity(), offset: value.mat2_begin as usize + 1 * 4 * 4 }
    }
}
impl ValueMat2 for Mat2Slot2 {
    fn value(&self) -> Matrix2 {
        self.value
    }
}
impl UniformMat2 for Mat2Slot2 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat2Slot2 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat2Slot2 {
    const ASK_SLOT_COUNT: u8 = 2;
}

// ==== ==== ==== ==== 3
pub struct Mat2Slot3 {
    pub value: Matrix2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat2Slot3 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix2::identity(), offset: value.mat2_begin as usize + 2 * 4 * 4 }
    }
}
impl ValueMat2 for Mat2Slot3 {
    fn value(&self) -> Matrix2 {
        self.value
    }
}
impl UniformMat2 for Mat2Slot3 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat2Slot3 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat2Slot3 {
    const ASK_SLOT_COUNT: u8 = 3;
}

// ==== ==== ==== ==== 1
pub struct Mat2Slot4 {
    pub value: Matrix2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat2Slot4 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix2::identity(), offset: value.mat2_begin as usize + 3 * 4 * 4 }
    }
}
impl ValueMat2 for Mat2Slot4 {
    fn value(&self) -> Matrix2 {
        self.value
    }
}
impl UniformMat2 for Mat2Slot4 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat2Slot4 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat2Slot4 {
    const ASK_SLOT_COUNT: u8 = 4;
}

// ==== ==== ==== ==== 5
pub struct Mat2Slot5 {
    pub value: Matrix2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat2Slot5 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix2::identity(), offset: value.mat2_begin as usize + 4 * 4 * 4 }
    }
}
impl ValueMat2 for Mat2Slot5 {
    fn value(&self) -> Matrix2 {
        self.value
    }
}
impl UniformMat2 for Mat2Slot5 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat2Slot5 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat2Slot5 {
    const ASK_SLOT_COUNT: u8 = 5;
}

// ==== ==== ==== ==== 6
pub struct Mat2Slot6 {
    pub value: Matrix2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat2Slot6 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix2::identity(), offset: value.mat2_begin as usize + 5 * 4 * 4 }
    }
}
impl ValueMat2 for Mat2Slot6 {
    fn value(&self) -> Matrix2 {
        self.value
    }
}
impl UniformMat2 for Mat2Slot6 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat2Slot6 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat2Slot6 {
    const ASK_SLOT_COUNT: u8 = 6;
}

// ==== ==== ==== ==== 7
pub struct Mat2Slot7 {
    pub value: Matrix2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat2Slot7 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix2::identity(), offset: value.mat2_begin as usize + 6 * 4 * 4 }
    }
}
impl ValueMat2 for Mat2Slot7 {
    fn value(&self) -> Matrix2 {
        self.value
    }
}
impl UniformMat2 for Mat2Slot7 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat2Slot7 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat2Slot7 {
    const ASK_SLOT_COUNT: u8 = 7;
}

// ==== ==== ==== ==== 8
pub struct Mat2Slot8 {
    pub value: Matrix2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Mat2Slot8 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Matrix2::identity(), offset: value.mat2_begin as usize + 7 * 4 * 4 }
    }
}
impl ValueMat2 for Mat2Slot8 {
    fn value(&self) -> Matrix2 {
        self.value
    }
}
impl UniformMat2 for Mat2Slot8 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Mat2Slot8 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Mat2Slot8 {
    const ASK_SLOT_COUNT: u8 = 8;
}
