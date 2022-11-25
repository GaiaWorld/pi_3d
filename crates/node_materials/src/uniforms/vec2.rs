
use pi_render::rhi::{dyn_uniform_buffer::Uniform, internal::bytemuck};
use pi_scene_context::{resources::RenderDynUniformBuffer, bytes_write_to_memory};
use pi_scene_math::Vector2;

use crate::{value::{Value, ValueSlotIndex, FromValueUniformStatistics, ValueVec2, SlotActiveRequire}, uniforms::value_uniform::ValueUniformStatistics};


pub trait UniformVec2: ValueVec2 {
    fn offset(&self) -> usize;
    fn write_into_buffer(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.value().as_slice()), index as usize + self.offset(), buffer);
    }
}

// ==== ==== ==== ==== 1
pub struct Vec2Slot1 {
    pub value: Vector2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Vec2Slot1 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Vector2::identity(), offset: value.mat4_begin as usize + 0 * 16 * 4 }
    }
}
impl ValueVec2 for Vec2Slot1 {
    fn value(&self) -> Vector2 {
        self.value
    }
}
impl UniformVec2 for Vec2Slot1 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Vec2Slot1 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Vec2Slot1 {
    const ASK_SLOT_COUNT: u8 = 1;
}

// ==== ==== ==== ==== 2
pub struct Vec2Slot2 {
    pub value: Vector2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Vec2Slot2 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Vector2::identity(), offset: value.mat4_begin as usize + 1 * 16 * 4 }
    }
}
impl ValueVec2 for Vec2Slot2 {
    fn value(&self) -> Vector2 {
        self.value
    }
}
impl UniformVec2 for Vec2Slot2 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Vec2Slot2 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Vec2Slot2 {
    const ASK_SLOT_COUNT: u8 = 2;
}

// ==== ==== ==== ==== 3
pub struct Vec2Slot3 {
    pub value: Vector2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Vec2Slot3 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Vector2::identity(), offset: value.mat4_begin as usize + 2 * 16 * 4 }
    }
}
impl ValueVec2 for Vec2Slot3 {
    fn value(&self) -> Vector2 {
        self.value
    }
}
impl UniformVec2 for Vec2Slot3 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Vec2Slot3 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Vec2Slot3 {
    const ASK_SLOT_COUNT: u8 = 3;
}

// ==== ==== ==== ==== 1
pub struct Vec2Slot4 {
    pub value: Vector2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Vec2Slot4 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Vector2::identity(), offset: value.mat4_begin as usize + 3 * 16 * 4 }
    }
}
impl ValueVec2 for Vec2Slot4 {
    fn value(&self) -> Vector2 {
        self.value
    }
}
impl UniformVec2 for Vec2Slot4 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Vec2Slot4 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Vec2Slot4 {
    const ASK_SLOT_COUNT: u8 = 4;
}

// ==== ==== ==== ==== 5
pub struct Vec2Slot5 {
    pub value: Vector2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Vec2Slot5 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Vector2::identity(), offset: value.mat4_begin as usize + 4 * 16 * 4 }
    }
}
impl ValueVec2 for Vec2Slot5 {
    fn value(&self) -> Vector2 {
        self.value
    }
}
impl UniformVec2 for Vec2Slot5 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Vec2Slot5 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Vec2Slot6 {
    const ASK_SLOT_COUNT: u8 = 6;
}

// ==== ==== ==== ==== 6
pub struct Vec2Slot6 {
    pub value: Vector2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Vec2Slot6 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Vector2::identity(), offset: value.mat4_begin as usize + 5 * 16 * 4 }
    }
}
impl ValueVec2 for Vec2Slot6 {
    fn value(&self) -> Vector2 {
        self.value
    }
}
impl UniformVec2 for Vec2Slot6 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Vec2Slot6 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Vec2Slot7 {
    const ASK_SLOT_COUNT: u8 = 7;
}

// ==== ==== ==== ==== 7
pub struct Vec2Slot7 {
    pub value: Vector2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Vec2Slot7 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Vector2::identity(), offset: value.mat4_begin as usize + 6 * 16 * 4 }
    }
}
impl ValueVec2 for Vec2Slot7 {
    fn value(&self) -> Vector2 {
        self.value
    }
}
impl UniformVec2 for Vec2Slot7 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Vec2Slot7 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}

// ==== ==== ==== ==== 8
pub struct Vec2Slot8 {
    pub value: Vector2,
    pub offset: usize,
}
impl FromValueUniformStatistics for Vec2Slot8 {
    fn new(value: &ValueUniformStatistics) -> Self {
        Self { value: Vector2::identity(), offset: value.mat4_begin as usize + 7 * 16 * 4 }
    }
}
impl ValueVec2 for Vec2Slot8 {
    fn value(&self) -> Vector2 {
        self.value
    }
}
impl UniformVec2 for Vec2Slot8 {
    fn offset(&self) -> usize {
        self.offset
    }
}
impl Uniform for Vec2Slot8 {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        self.write_into_buffer(index, buffer);
    }
}
impl SlotActiveRequire for Vec2Slot8 {
    const ASK_SLOT_COUNT: u8 = 8;
}
