use std::ops::Range;

use pi_render::rhi::{internal::bytemuck, dyn_uniform_buffer::Uniform};
use pi_scene_math::Number;
use crate::{bytes_write_to_memory, materials::value::{FromValueUniformStatistics}};

use super::{value_uniform::MaterialValueBind, update_data};

pub enum Vec2Uniform {
    Slot0(u32),
    Slot1(u32, [Number; 1 * 2]),
    Slot2(u32, [Number; 2 * 2]),
    Slot3(u32, [Number; 3 * 2]),
    Slot4(u32, [Number; 4 * 2]),
    Slot5(u32, [Number; 5 * 2]),
    Slot6(u32, [Number; 6 * 2]),
    Slot7(u32, [Number; 7 * 2]),
    Slot8(u32, [Number; 8 * 2]),
}
impl FromValueUniformStatistics for Vec2Uniform {
    fn new(value: &MaterialValueBind) -> Self {
        if value.vec2_count == 1 {
            Self::Slot1(value.vec2_begin, [0., 0.])
        }
        else if value.vec2_count == 2 {
            Self::Slot2(value.vec2_begin, [0., 0., 0., 0.])
        }
        else if value.vec2_count == 3 {
            Self::Slot3(value.vec2_begin, [0., 0., 0., 0., 0., 0.])
        }
        else if value.vec2_count == 4 {
            Self::Slot4(value.vec2_begin, [0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec2_count == 5 {
            Self::Slot5(value.vec2_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec2_count == 6 {
            Self::Slot6(value.vec2_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec2_count == 7 {
            Self::Slot7(value.vec2_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec2_count == 8 {
            Self::Slot8(value.vec2_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else {
            Self::Slot0(value.vec2_begin)
        }
    }
}
impl Vec2Uniform {
    const N: usize = 2;
    const M: [Number; Self::N] = [0., 0.];
    pub fn value(&self, slot: usize) -> &[Number] {
        let range = Range { start: slot * Self::N, end: (slot + 1) * Self::N };
        match self {
            Self::Slot0(_) => &Self::M[0..Self::N],
            Self::Slot1(_, data) => &data[range],
            Self::Slot2(_, data) => &data[range],
            Self::Slot3(_, data) => &data[range],
            Self::Slot4(_, data) => &data[range],
            Self::Slot5(_, data) => &data[range],
            Self::Slot6(_, data) => &data[range],
            Self::Slot7(_, data) => &data[range],
            Self::Slot8(_, data) => &data[range],
        }
    }
    pub fn set(
        &mut self,
        slot: usize,
        value: &[Number],
    ) {
        match self {
            Vec2Uniform::Slot0(_) => {},
            Vec2Uniform::Slot1(_, data) => {
                if slot <= 0 { update_data(data, slot, value, 2); }
            },
            Vec2Uniform::Slot2(_, data) =>  {
                if slot <= 1 { update_data(data, slot, value, 2); }
            },
            Vec2Uniform::Slot3(_, data) =>  {
                if slot <= 2 { update_data(data, slot, value, 2); }
            },
            Vec2Uniform::Slot4(_, data) =>  {
                if slot <= 3 { update_data(data, slot, value, 2); }
            },
            Vec2Uniform::Slot5(_, data) =>  {
                if slot <= 4 { update_data(data, slot, value, 2); }
            },
            Vec2Uniform::Slot6(_, data) =>  {
                if slot <= 5 { update_data(data, slot, value, 2); }
            },
            Vec2Uniform::Slot7(_, data) =>  {
                if slot <= 6 { update_data(data, slot, value, 2); }
            },
            Vec2Uniform::Slot8(_, data) =>  {
                if slot <= 7 { update_data(data, slot, value, 2); }
            },
        }
    }
}
impl Uniform for Vec2Uniform {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        match self {
            Vec2Uniform::Slot0(_) => {},
            Vec2Uniform::Slot1(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec2Uniform::Slot2(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec2Uniform::Slot3(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec2Uniform::Slot4(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec2Uniform::Slot5(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec2Uniform::Slot6(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec2Uniform::Slot7(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec2Uniform::Slot8(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
        }
    }
}
