
use pi_scene_shell::prelude::*;
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
