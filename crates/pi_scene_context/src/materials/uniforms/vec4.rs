use std::ops::Range;

use pi_render::rhi::{internal::bytemuck, dyn_uniform_buffer::Uniform};
use pi_scene_math::Number;
use crate::{bytes_write_to_memory, materials::{value::{FromValueUniformStatistics}, material_meta::UniformPropertyVec4}};

use super::{value_uniform::MaterialValueBind, update_data};


pub enum Vec4Uniform {
    Slot0(u32),
    Slot1(u32, [Number; 1 * 4]),
    Slot2(u32, [Number; 2 * 4]),
    Slot3(u32, [Number; 3 * 4]),
    Slot4(u32, [Number; 4 * 4]),
    Slot5(u32, [Number; 5 * 4]),
    Slot6(u32, [Number; 6 * 4]),
    Slot7(u32, [Number; 7 * 4]),
    Slot8(u32, [Number; 8 * 4]),
}
impl FromValueUniformStatistics for Vec4Uniform {
    fn new(value: &MaterialValueBind) -> Self {
        // println!("Vec4Uniform >>>>>>>");
        // println!("{:?}", value);
        if value.vec4_count == 1 {
            Self::Slot1(value.vec4_begin, [0., 0., 0., 0.])
        }
        else if value.vec4_count == 2 {
            Self::Slot2(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec4_count == 3 {
            Self::Slot3(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec4_count == 4 {
            Self::Slot4(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec4_count == 5 {
            Self::Slot5(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec4_count == 6 {
            Self::Slot6(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec4_count == 7 {
            Self::Slot7(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.vec4_count == 8 {
            Self::Slot8(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else {
            Self::Slot0(value.vec4_begin)
        }
    }
}
impl Vec4Uniform {
    const N: usize = 4;
    const M: [Number; Self::N] = [0., 0., 0., 0.];
    pub fn init(&mut self, desc: &Vec<UniformPropertyVec4>) {
        let mut index = 0;
        desc.iter().for_each(|item| {
            self.set(index, item.1.as_slice());
            index += 1;
        });
    }
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
            Vec4Uniform::Slot0(_) => {},
            Vec4Uniform::Slot1(_, data) => {
                if slot <= 0 { update_data(data, slot, value, 4); }
            },
            Vec4Uniform::Slot2(_, data) =>  {
                if slot <= 1 { update_data(data, slot, value, 4); }
            },
            Vec4Uniform::Slot3(_, data) =>  {
                if slot <= 2 { update_data(data, slot, value, 4); }
            },
            Vec4Uniform::Slot4(_, data) =>  {
                if slot <= 3 { update_data(data, slot, value, 4); }
            },
            Vec4Uniform::Slot5(_, data) =>  {
                if slot <= 4 { update_data(data, slot, value, 4); }
            },
            Vec4Uniform::Slot6(_, data) =>  {
                if slot <= 5 { update_data(data, slot, value, 4); }
            },
            Vec4Uniform::Slot7(_, data) =>  {
                if slot <= 6 { update_data(data, slot, value, 4); }
            },
            Vec4Uniform::Slot8(_, data) =>  {
                if slot <= 7 { update_data(data, slot, value, 4); }
            },
        }
    }
}
impl Uniform for Vec4Uniform {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        match self {
            Vec4Uniform::Slot0(_) => {},
            Vec4Uniform::Slot1(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec4Uniform::Slot2(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec4Uniform::Slot3(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec4Uniform::Slot4(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec4Uniform::Slot5(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec4Uniform::Slot6(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec4Uniform::Slot7(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            Vec4Uniform::Slot8(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
        }
    }
}
