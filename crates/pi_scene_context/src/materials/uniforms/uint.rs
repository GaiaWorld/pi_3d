
use pi_engine_shell::prelude::*;
use crate::materials::value::FromValueUniformStatistics;


const MAX: u8 = 64;

/// 最多 64 个 i32 - 共 256 byte
pub struct UintUniform {
    slot: u8,
    data: [u32; 64], // MAX as usize],
    begin: u32,
}
impl FromValueUniformStatistics for UintUniform {
    fn new(value: &ShaderBindEffectValue) -> Self {
        let slot = value.uint_count.min(MAX / Self::N as u8);
        Self {
            slot,
            data: [0; MAX as usize],
            begin: value.uint_begin,
        }
    }
}
impl UintUniform {
    const N: usize = 1;
    pub fn init(&mut self, desc: &Vec<UniformPropertyUint>) {
        let mut index = 0;
        desc.iter().for_each(|item| {
            // log::warn!("Float {:?} {:?}", item.tag(), item.1);
            self.set(index, item.1);
            index += 1;
        });
    }
    pub fn value(&self, slot: usize) -> u32 {
        if slot < self.slot as usize {
            self.data[slot]
        } else {
            0
        }
    }
    pub fn set(
        &mut self,
        slot: usize,
        value: u32,
    ) {
        if slot < self.slot as usize {
            self.data[slot] = value;
        }
    }
    pub fn update(&self, range: &BindBufferRange) {
        let len = self.slot as usize;
        if self.slot > 0 {
            range.write_data(self.begin as usize, bytemuck::cast_slice(&self.data[0..len]))
        }
    }
}

// pub enum UintUniform {
//     Slot0(u32),
//     Slot1(u32, [u32; 1]),
//     Slot2(u32, [u32; 2]),
//     Slot3(u32, [u32; 3]),
//     Slot4(u32, [u32; 4]),
//     Slot5(u32, [u32; 5]),
//     Slot6(u32, [u32; 6]),
//     Slot7(u32, [u32; 7]),
//     Slot8(u32, [u32; 8]),
// }
// impl FromValueUniformStatistics for UintUniform {
//     fn new(value: &ShaderBindEffectValue) -> Self {
//         if value.uint_count == 1 {
//             Self::Slot1(value.uint_begin, [0])
//         }
//         else if value.uint_count == 2 {
//             Self::Slot2(value.uint_begin, [0, 0])
//         }
//         else if value.uint_count == 3 {
//             Self::Slot3(value.uint_begin, [0, 0, 0])
//         }
//         else if value.uint_count == 4 {
//             Self::Slot4(value.uint_begin, [0, 0, 0, 0])
//         }
//         else if value.uint_count == 5 {
//             Self::Slot5(value.uint_begin, [0, 0, 0, 0, 0])
//         }
//         else if value.uint_count == 6 {
//             Self::Slot6(value.uint_begin, [0, 0, 0, 0, 0, 0])
//         }
//         else if value.uint_count == 7 {
//             Self::Slot7(value.uint_begin, [0, 0, 0, 0, 0, 0, 0])
//         }
//         else if value.uint_count == 8 {
//             Self::Slot8(value.uint_begin, [0, 0, 0, 0, 0, 0, 0, 0])
//         }
//         else {
//             Self::Slot0(value.uint_begin)
//         }
//     }
// }
// impl UintUniform {
//     pub fn init(&mut self, desc: &Vec<UniformPropertyUint>) {
//         let mut index = 0;
//         desc.iter().for_each(|item| {
//             self.set(index, item.1);
//             index += 1;
//         });
//     }
//     pub fn value(&self, index: usize) -> u32 {
//         match self {
//             Self::Slot0(_) => 0,
//             Self::Slot1(_, data) => data[index],
//             Self::Slot2(_, data) => data[index],
//             Self::Slot3(_, data) => data[index],
//             Self::Slot4(_, data) => data[index],
//             Self::Slot5(_, data) => data[index],
//             Self::Slot6(_, data) => data[index],
//             Self::Slot7(_, data) => data[index],
//             Self::Slot8(_, data) => data[index],
//         }
//     }
//     // pub fn value_mut(&mut self, slot: usize) -> Option<&mut [u32]> {
//     //     let range = Range { start: slot * 1, end: (slot + 1) * 1 };
//     //     match self {
//     //         Self::Slot0(_) => None,
//     //         Self::Slot1(_, data) => Some(&mut data[range]),
//     //         Self::Slot2(_, data) => Some(&mut data[range]),
//     //         Self::Slot3(_, data) => Some(&mut data[range]),
//     //         Self::Slot4(_, data) => Some(&mut data[range]),
//     //         Self::Slot5(_, data) => Some(&mut data[range]),
//     //         Self::Slot6(_, data) => Some(&mut data[range]),
//     //         Self::Slot7(_, data) => Some(&mut data[range]),
//     //         Self::Slot8(_, data) => Some(&mut data[range]),
//     //         // Self::Slot9(_, data) => Some(&mut data[range]),
//     //         // Self::Slot10(_, data) => Some(&mut data[range]),
//     //         // Self::Slot11(_, data) => Some(&mut data[range]),
//     //         // Self::Slot12(_, data) => Some(&mut data[range]),
//     //     }
//     // }
//     pub fn set(
//         &mut self,
//         slot: usize,
//         value: u32,
//     ) {
//         match self {
//             UintUniform::Slot0(_) => {},
//             UintUniform::Slot1(_, data) => {
//                 if slot <= 0 { data[slot] = value; }
//             },
//             UintUniform::Slot2(_, data) =>  {
//                 if slot <= 1 { data[slot] = value; }
//             },
//             UintUniform::Slot3(_, data) =>  {
//                 if slot <= 2 { data[slot] = value; }
//             },
//             UintUniform::Slot4(_, data) =>  {
//                 if slot <= 3 { data[slot] = value; }
//             },
//             UintUniform::Slot5(_, data) =>  {
//                 if slot <= 4 { data[slot] = value; }
//             },
//             UintUniform::Slot6(_, data) =>  {
//                 if slot <= 5 { data[slot] = value; }
//             },
//             UintUniform::Slot7(_, data) =>  {
//                 if slot <= 6 { data[slot] = value; }
//             },
//             UintUniform::Slot8(_, data) =>  {
//                 if slot <= 7 { data[slot] = value; }
//             },
//         }
//     }
//     pub fn update(&self, range: &BindBufferRange) {
//         match self {
//             Self::Slot0(_) => {},
//             Self::Slot1(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot2(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot3(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot4(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot5(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot6(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot7(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//             Self::Slot8(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
//         }
//     }
// }