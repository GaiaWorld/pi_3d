use std::ops::Range;

use pi_engine_shell::prelude::*;
use crate::{materials::{value::{FromValueUniformStatistics}}};



pub enum IntUniform {
    Slot0(u32),
    Slot1(u32, [i32; 1]),
    Slot2(u32, [i32; 2]),
    Slot3(u32, [i32; 3]),
    Slot4(u32, [i32; 4]),
    Slot5(u32, [i32; 5]),
    Slot6(u32, [i32; 6]),
    Slot7(u32, [i32; 7]),
    Slot8(u32, [i32; 8]),
}
impl FromValueUniformStatistics for IntUniform {
    fn new(value: &ShaderBindEffectValue) -> Self {
        if value.int_count == 1 {
            Self::Slot1(value.int_begin, [0])
        }
        else if value.int_count == 2 {
            Self::Slot2(value.int_begin, [0, 0])
        }
        else if value.int_count == 3 {
            Self::Slot3(value.int_begin, [0, 0, 0])
        }
        else if value.int_count == 4 {
            Self::Slot4(value.int_begin, [0, 0, 0, 0])
        }
        else if value.int_count == 5 {
            Self::Slot5(value.int_begin, [0, 0, 0, 0, 0])
        }
        else if value.int_count == 6 {
            Self::Slot6(value.int_begin, [0, 0, 0, 0, 0, 0])
        }
        else if value.int_count == 7 {
            Self::Slot7(value.int_begin, [0, 0, 0, 0, 0, 0, 0])
        }
        else if value.int_count == 8 {
            Self::Slot8(value.int_begin, [0, 0, 0, 0, 0, 0, 0, 0])
        }
        else {
            Self::Slot0(value.int_begin)
        }
    }
}
impl IntUniform {
    pub fn init(&mut self, desc: &Vec<UniformPropertyInt>) {
        let mut index = 0;
        desc.iter().for_each(|item| {
            self.set(index, item.1);
            index += 1;
        });
    }
    pub fn value(&self, slot: usize) -> i32 {
        match self {
            Self::Slot0(_) => 0,
            Self::Slot1(_, data) => data[slot],
            Self::Slot2(_, data) => data[slot],
            Self::Slot3(_, data) => data[slot],
            Self::Slot4(_, data) => data[slot],
            Self::Slot5(_, data) => data[slot],
            Self::Slot6(_, data) => data[slot],
            Self::Slot7(_, data) => data[slot],
            Self::Slot8(_, data) => data[slot],
        }
    }
    // pub fn value_mut(&mut self, slot: usize) -> Option<&mut [i32]> {
    //     let range = Range { start: slot * 1, end: (slot + 1) * 1 };
    //     match self {
    //         Self::Slot0(_) => None,
    //         Self::Slot1(_, data) => Some(&mut data[range]),
    //         Self::Slot2(_, data) => Some(&mut data[range]),
    //         Self::Slot3(_, data) => Some(&mut data[range]),
    //         Self::Slot4(_, data) => Some(&mut data[range]),
    //         Self::Slot5(_, data) => Some(&mut data[range]),
    //         Self::Slot6(_, data) => Some(&mut data[range]),
    //         Self::Slot7(_, data) => Some(&mut data[range]),
    //         Self::Slot8(_, data) => Some(&mut data[range]),
    //         // Self::Slot9(_, data) => Some(&mut data[range]),
    //         // Self::Slot10(_, data) => Some(&mut data[range]),
    //         // Self::Slot11(_, data) => Some(&mut data[range]),
    //         // Self::Slot12(_, data) => Some(&mut data[range]),
    //     }
    // }
    pub fn set(
        &mut self,
        slot: usize,
        value: i32,
    ) {
        match self {
            IntUniform::Slot0(_) => {},
            IntUniform::Slot1(_, data) => {
                if slot <= 0 { data[slot] = value; }
            },
            IntUniform::Slot2(_, data) =>  {
                if slot <= 1 { data[slot] = value; }
            },
            IntUniform::Slot3(_, data) =>  {
                if slot <= 2 { data[slot] = value; }
            },
            IntUniform::Slot4(_, data) =>  {
                if slot <= 3 { data[slot] = value; }
            },
            IntUniform::Slot5(_, data) =>  {
                if slot <= 4 { data[slot] = value; }
            },
            IntUniform::Slot6(_, data) =>  {
                if slot <= 5 { data[slot] = value; }
            },
            IntUniform::Slot7(_, data) =>  {
                if slot <= 6 { data[slot] = value; }
            },
            IntUniform::Slot8(_, data) =>  {
                if slot <= 7 { data[slot] = value; }
            },
        }
    }
    pub fn update(&self, range: &BindBufferRange) {
        match self {
            Self::Slot0(_) => {},
            Self::Slot1(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot2(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot3(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot4(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot5(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot6(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot7(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot8(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
        }
    }
}
