use pi_render::rhi::{internal::bytemuck, dyn_uniform_buffer::Uniform};
use pi_scene_math::Number;
use render_shader::shader_bind::ShaderBindEffectValue;
use crate::{bytes_write_to_memory, materials::{value::{FromValueUniformStatistics}, shader_effect::UniformPropertyFloat}};



pub enum FloatUniform {
    Slot0(u32),
    Slot1(u32, [Number; 1]),
    Slot2(u32, [Number; 2]),
    Slot3(u32, [Number; 3]),
    Slot4(u32, [Number; 4]),
    Slot5(u32, [Number; 5]),
    Slot6(u32, [Number; 6]),
    Slot7(u32, [Number; 7]),
    Slot8(u32, [Number; 8]),
}
impl FromValueUniformStatistics for FloatUniform {
    fn new(value: &ShaderBindEffectValue) -> Self {
        if value.float_count == 1 {
            Self::Slot1(value.float_begin, [0.])
        }
        else if value.float_count == 2 {
            Self::Slot2(value.float_begin, [0., 0.])
        }
        else if value.float_count == 3 {
            Self::Slot3(value.float_begin, [0., 0., 0.])
        }
        else if value.float_count == 4 {
            Self::Slot4(value.float_begin, [0., 0., 0., 0.])
        }
        else if value.float_count == 5 {
            Self::Slot5(value.float_begin, [0., 0., 0., 0., 0.])
        }
        else if value.float_count == 6 {
            Self::Slot6(value.float_begin, [0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 7 {
            Self::Slot7(value.float_begin, [0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 8 {
            Self::Slot8(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else {
            Self::Slot0(value.float_begin)
        }
    }
}
impl FloatUniform {
    pub fn init(&mut self, desc: &Vec<UniformPropertyFloat>) {
        let mut index = 0;
        desc.iter().for_each(|item| {
            self.set(index, item.1);
            index += 1;
        });
    }
    pub fn value(&self, slot: usize) -> Number {
        match self {
            Self::Slot0(_) => 0.,
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
    pub fn set(
        &mut self,
        slot: usize,
        value: Number,
    ) {
        match self {
            FloatUniform::Slot0(_) => {},
            FloatUniform::Slot1(_, data) => {
                if slot <= 0 { data[slot] = value; }
            },
            FloatUniform::Slot2(_, data) =>  {
                if slot <= 1 { data[slot] = value; }
            },
            FloatUniform::Slot3(_, data) =>  {
                if slot <= 2 { data[slot] = value; }
            },
            FloatUniform::Slot4(_, data) =>  {
                if slot <= 3 { data[slot] = value; }
            },
            FloatUniform::Slot5(_, data) =>  {
                if slot <= 4 { data[slot] = value; }
            },
            FloatUniform::Slot6(_, data) =>  {
                if slot <= 5 { data[slot] = value; }
            },
            FloatUniform::Slot7(_, data) =>  {
                if slot <= 6 { data[slot] = value; }
            },
            FloatUniform::Slot8(_, data) =>  {
                if slot <= 7 { data[slot] = value; }
            },
        }
    }
}
impl Uniform for FloatUniform {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        match self {
            FloatUniform::Slot0(_) => {},
            FloatUniform::Slot1(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            FloatUniform::Slot2(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            FloatUniform::Slot3(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            FloatUniform::Slot4(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            FloatUniform::Slot5(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            FloatUniform::Slot6(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            FloatUniform::Slot7(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
            FloatUniform::Slot8(offset, data) => bytes_write_to_memory(bytemuck::cast_slice(data), index as usize + *offset as usize, buffer),
        }
    }
}
