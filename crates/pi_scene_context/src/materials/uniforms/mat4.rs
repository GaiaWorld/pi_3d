use std::ops::Range;

use pi_scene_shell::prelude::*;
use pi_scene_math::Number;
use crate::{materials::{value::{FromValueUniformStatistics}}};

use super::{update_data};

const MAX: u8 = 64;

/// 最多 4 个 Mat4 - 共 256 byte
pub struct Mat4Uniform {
    slot: u8,
    data: [Number; 64], // MAX as usize],
    begin: u32,
}
impl FromValueUniformStatistics for Mat4Uniform {
    fn new(value: &ShaderBindEffectValue) -> Self {
        let slot = value.mat4_count.min(MAX / Self::N as u8);
        Self {
            slot,
            data: [0.; MAX as usize],
            begin: value.mat4_begin,
        }
    }
}
impl Mat4Uniform {
    const N: usize = 16;
    pub fn init(&mut self, desc: &Vec<UniformPropertyMat4>) {
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
            Some(&mut self.data[range])
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
