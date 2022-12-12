use pi_render::rhi::{internal::bytemuck, dyn_uniform_buffer::Uniform};
use crate::{bytes_write_to_memory, materials::{value::{FromValueUniformStatistics}, material_meta::UniformPropertyUint}};

use super::value_uniform::MaterialValueBind;

pub enum UintUniform {
    Slot0(u32),
    Slot1(u32, [u32; 1]),
    Slot2(u32, [u32; 2]),
    Slot3(u32, [u32; 3]),
    Slot4(u32, [u32; 4]),
    Slot5(u32, [u32; 5]),
    Slot6(u32, [u32; 6]),
    Slot7(u32, [u32; 7]),
    Slot8(u32, [u32; 8]),
}
impl FromValueUniformStatistics for UintUniform {
    fn new(value: &MaterialValueBind) -> Self {
        if value.uint_count == 1 {
            Self::Slot1(value.uint_begin, [0])
        }
        else if value.uint_count == 2 {
            Self::Slot2(value.uint_begin, [0, 0])
        }
        else if value.uint_count == 3 {
            Self::Slot3(value.uint_begin, [0, 0, 0])
        }
        else if value.uint_count == 4 {
            Self::Slot4(value.uint_begin, [0, 0, 0, 0])
        }
        else if value.uint_count == 5 {
            Self::Slot5(value.uint_begin, [0, 0, 0, 0, 0])
        }
        else if value.uint_count == 6 {
            Self::Slot6(value.uint_begin, [0, 0, 0, 0, 0, 0])
        }
        else if value.uint_count == 7 {
            Self::Slot7(value.uint_begin, [0, 0, 0, 0, 0, 0, 0])
        }
        else if value.uint_count == 8 {
            Self::Slot8(value.uint_begin, [0, 0, 0, 0, 0, 0, 0, 0])
        }
        else {
            Self::Slot0(value.uint_begin)
        }
    }
}
impl UintUniform {
    pub fn init(&mut self, desc: &Vec<UniformPropertyUint>) {
        let mut index = 0;
        desc.iter().for_each(|item| {
            self.set(index, item.1);
            index += 1;
        });
    }
    pub fn value(&self, index: usize) -> u32 {
        match self {
            Self::Slot0(_) => 0,
            Self::Slot1(_, data) => data[index],
            Self::Slot2(_, data) => data[index],
            Self::Slot3(_, data) => data[index],
            Self::Slot4(_, data) => data[index],
            Self::Slot5(_, data) => data[index],
            Self::Slot6(_, data) => data[index],
            Self::Slot7(_, data) => data[index],
            Self::Slot8(_, data) => data[index],
        }
    }
    pub fn set(
        &mut self,
        slot: usize,
        value: u32,
    ) {
        match self {
            UintUniform::Slot0(_) => {},
            UintUniform::Slot1(_, data) => {
                if slot <= 0 { data[slot] = value; }
            },
            UintUniform::Slot2(_, data) =>  {
                if slot <= 1 { data[slot] = value; }
            },
            UintUniform::Slot3(_, data) =>  {
                if slot <= 2 { data[slot] = value; }
            },
            UintUniform::Slot4(_, data) =>  {
                if slot <= 3 { data[slot] = value; }
            },
            UintUniform::Slot5(_, data) =>  {
                if slot <= 4 { data[slot] = value; }
            },
            UintUniform::Slot6(_, data) =>  {
                if slot <= 5 { data[slot] = value; }
            },
            UintUniform::Slot7(_, data) =>  {
                if slot <= 6 { data[slot] = value; }
            },
            UintUniform::Slot8(_, data) =>  {
                if slot <= 7 { data[slot] = value; }
            },
        }
    }
}
impl Uniform for UintUniform {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        match self {
            UintUniform::Slot0(_) => {},
            UintUniform::Slot1(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            UintUniform::Slot2(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            UintUniform::Slot3(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            UintUniform::Slot4(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            UintUniform::Slot5(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            UintUniform::Slot6(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            UintUniform::Slot7(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            UintUniform::Slot8(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
        }
    }
}