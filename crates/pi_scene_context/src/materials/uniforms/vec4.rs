use std::ops::Range;

use pi_engine_shell::prelude::*;
use pi_scene_math::Number;
use crate::materials::value::FromValueUniformStatistics;

use super::update_data;

const MAX: u8 = 128;
/// 最多 32 个 vec4 - 共 512 byte
pub struct Vec4Uniform {
    pub(crate) slot: u8,
    pub(crate) data: [Number; 128], // MAX as usize],
    begin: u32,
}
impl FromValueUniformStatistics for Vec4Uniform {
    fn new(value: &ShaderBindEffectValue) -> Self {
        let slot = value.vec4_count.min(MAX / Self::N as u8);
        Self {
            slot,
            data: [0.; MAX as usize],
            begin: value.vec4_begin,
        }
    }
}
impl Vec4Uniform {
    pub const N: usize = 4;
    // const M: [Number; Self::N] = [0., 0., 0., 0.];
    pub fn init(&mut self, desc: &Vec<UniformPropertyVec4>) {
        let mut index = 0;
        desc.iter().for_each(|item| {
            // log::warn!("Float {:?} {:?}", item.tag(), item.1);
            self.set(index, item.1.as_slice());
            index += 1;
        });
    }
    pub fn value(&self, slot: usize) -> &[Number] {
        let slot = if slot < self.slot as usize {
            slot
        } else {
            0
        };

        let range = Range { start: slot * Self::N, end: (slot + 1) * Self::N };
        &self.data[range]
    }
    pub fn set(
        &mut self,
        slot: usize,
        value: &[Number],
    ) {
        if slot < self.slot as usize {
            update_data(&mut self.data, slot, value, Self::N);
        }
    }
    pub fn value_mut(&mut self, slot: usize) -> Option<&mut [Number]> {
        if slot < self.slot as usize {
            let range = Range { start: slot * Self::N, end: (slot + 1) * Self::N };
            Some(&mut self.data.as_mut_slice()[range])
        } else {
            None
        }
    }
    pub fn update(&self, range: &BindBufferRange) {
        if self.slot > 0 {
            let rang = Range { start: 0 * Self::N, end: self.slot as usize * Self::N };
            range.write_data(self.begin as usize, bytemuck::cast_slice(&self.data[rang]))
        }
    }
}

// pub enum Vec4Uniform {
//     Slot0(u32),
//     Slot1(u32, [Number; 1 * 4]),
//     Slot2(u32, [Number; 2 * 4]),
//     Slot3(u32, [Number; 3 * 4]),
//     Slot4(u32, [Number; 4 * 4]),
//     Slot5(u32, [Number; 5 * 4]),
//     Slot6(u32, [Number; 6 * 4]),
//     Slot7(u32, [Number; 7 * 4]),
//     Slot8(u32, [Number; 8 * 4]),
//     Slot9(u32, [Number; 9 * 4]),
//     Slot10(u32, [Number; 10 * 4]),
//     Slot11(u32, [Number; 11 * 4]),
//     Slot12(u32, [Number; 12 * 4]),
// }
// impl FromValueUniformStatistics for Vec4Uniform {
//     fn new(value: &ShaderBindEffectValue) -> Self {
//         // log::debug!("Vec4Uniform >>>>>>>");
//         // log::debug!("{:?}", value);
//         if value.vec4_count == 1 {
//             Self::Slot1(value.vec4_begin, [0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 2 {
//             Self::Slot2(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 3 {
//             Self::Slot3(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 4 {
//             Self::Slot4(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 5 {
//             Self::Slot5(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 6 {
//             Self::Slot6(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 7 {
//             Self::Slot7(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 8 {
//             Self::Slot8(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 9 {
//             Self::Slot9(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 10 {
//             Self::Slot10(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 11 {
//             Self::Slot11(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else if value.vec4_count == 12 {
//             Self::Slot12(value.vec4_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
//         }
//         else {
//             Self::Slot0(value.vec4_begin)
//         }
//     }
// }
// impl Vec4Uniform {
//     const N: usize = 4;
//     const M: [Number; Self::N] = [0., 0., 0., 0.];
//     pub fn init(&mut self, desc: &Vec<UniformPropertyVec4>) {
//         let mut index = 0;
//         desc.iter().for_each(|item| {
//             self.set(index, item.1.as_slice());
//             index += 1;
//         });
//     }
//     pub fn value(&self, slot: usize) -> &[Number] {
//         let range = Range { start: slot * Self::N, end: (slot + 1) * Self::N };
//         match self {
//             Self::Slot0(_) => &Self::M[0..Self::N],
//             Self::Slot1(_, data) => &data[range],
//             Self::Slot2(_, data) => &data[range],
//             Self::Slot3(_, data) => &data[range],
//             Self::Slot4(_, data) => &data[range],
//             Self::Slot5(_, data) => &data[range],
//             Self::Slot6(_, data) => &data[range],
//             Self::Slot7(_, data) => &data[range],
//             Self::Slot8(_, data) => &data[range],
//             Self::Slot9(_, data) => &data[range],
//             Self::Slot10(_, data) => &data[range],
//             Self::Slot11(_, data) => &data[range],
//             Self::Slot12(_, data) => &data[range],
//         }
//     }
//     pub fn value_mut(&mut self, slot: usize) -> Option<&mut [Number]> {
//         let range = Range { start: slot * Self::N, end: (slot + 1) * Self::N };
//         match self {
//             Self::Slot0(_) => None,
//             Self::Slot1(_, data) => Some(&mut data[range]),
//             Self::Slot2(_, data) => Some(&mut data[range]),
//             Self::Slot3(_, data) => Some(&mut data[range]),
//             Self::Slot4(_, data) => Some(&mut data[range]),
//             Self::Slot5(_, data) => Some(&mut data[range]),
//             Self::Slot6(_, data) => Some(&mut data[range]),
//             Self::Slot7(_, data) => Some(&mut data[range]),
//             Self::Slot8(_, data) => Some(&mut data[range]),
//             Self::Slot9(_, data) => Some(&mut data[range]),
//             Self::Slot10(_, data) => Some(&mut data[range]),
//             Self::Slot11(_, data) => Some(&mut data[range]),
//             Self::Slot12(_, data) => Some(&mut data[range]),
//         }
//     }
//     pub fn set(
//         &mut self,
//         slot: usize,
//         value: &[Number],
//     ) {
//         match self {
//             Vec4Uniform::Slot0(_) => {},
//             Vec4Uniform::Slot1(_, data) => {
//                 if slot <= 0 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot2(_, data) =>  {
//                 if slot <= 1 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot3(_, data) =>  {
//                 if slot <= 2 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot4(_, data) =>  {
//                 if slot <= 3 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot5(_, data) =>  {
//                 if slot <= 4 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot6(_, data) =>  {
//                 if slot <= 5 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot7(_, data) =>  {
//                 if slot <= 6 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot8(_, data) =>  {
//                 if slot <= 7 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot9(_, data) =>  {
//                 if slot <= 8 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot10(_, data) =>  {
//                 if slot <= 9 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot11(_, data) =>  {
//                 if slot <= 10 { update_data(data, slot, value, 4); }
//             },
//             Vec4Uniform::Slot12(_, data) =>  {
//                 if slot <= 11 { update_data(data, slot, value, 4); }
//             },
//         }
//     }
//     pub fn update(&self, range: &BindBufferRange) {
//         match self {
//             Self::Slot0(_) => {},
//             Self::Slot1(offset, data) => {
//                 range.write_data(*offset as usize, bytemuck::cast_slice(data)); 
//             },
//             Self::Slot2(offset, data) => {
//                 range.write_data(*offset as usize, bytemuck::cast_slice(data));
//             },
//             Self::Slot3(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot4(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot5(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot6(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot7(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot8(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot9(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot10(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot11(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot12(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//         }
//     }
// }
