use pi_engine_shell::prelude::*;
use pi_scene_math::Number;
use crate::{materials::{value::{FromValueUniformStatistics}}};



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
    Slot9(u32, [Number; 9]),
    Slot10(u32, [Number; 10]),
    Slot11(u32, [Number; 11]),
    Slot12(u32, [Number; 12]),
    Slot13(u32, [Number; 13]),
    Slot14(u32, [Number; 14]),
    Slot15(u32, [Number; 15]),
    Slot16(u32, [Number; 16]),
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
        else if value.float_count == 9 {
            Self::Slot9(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 10 {
            Self::Slot10(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 11 {
            Self::Slot11(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 12 {
            Self::Slot12(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 13 {
            Self::Slot13(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 14 {
            Self::Slot14(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 15 {
            Self::Slot15(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
        }
        else if value.float_count == 16 {
            Self::Slot16(value.float_begin, [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.])
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
            log::warn!("Float {:?} {:?}", item.tag(), item.1);
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
            Self::Slot9(_, data) => data[slot],
            Self::Slot10(_, data) => data[slot],
            Self::Slot11(_, data) => data[slot],
            Self::Slot12(_, data) => data[slot],
            Self::Slot13(_, data) => data[slot],
            Self::Slot14(_, data) => data[slot],
            Self::Slot15(_, data) => data[slot],
            Self::Slot16(_, data) => data[slot],
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
            FloatUniform::Slot9(_, data) => {
                if slot <= 8 { data[slot] = value; }
            },
            FloatUniform::Slot10(_, data) =>  {
                if slot <= 9 { data[slot] = value; }
            },
            FloatUniform::Slot11(_, data) =>  {
                if slot <= 10 { data[slot] = value; }
            },
            FloatUniform::Slot12(_, data) =>  {
                if slot <= 11 { data[slot] = value; }
            },
            FloatUniform::Slot13(_, data) =>  {
                if slot <= 12 { data[slot] = value; }
            },
            FloatUniform::Slot14(_, data) =>  {
                if slot <= 13 { data[slot] = value; }
            },
            FloatUniform::Slot15(_, data) =>  {
                if slot <= 14 { data[slot] = value; }
            },
            FloatUniform::Slot16(_, data) =>  {
                if slot <= 15 { data[slot] = value; }
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
            Self::Slot9(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot10(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot11(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot12(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot13(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot14(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot15(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
            Self::Slot16(offset, data) => range.write_data(*offset as usize, bytemuck::cast_slice(data)),
        }
    }
}
