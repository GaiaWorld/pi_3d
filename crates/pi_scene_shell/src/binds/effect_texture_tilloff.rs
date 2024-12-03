use pi_render::renderer::bind::{KeyBindBuffer, KeyBindLayoutBuffer, TKeyBind};

use crate::prelude::*;


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindEffectTextureTilloff {
    pub data: BindBufferRange,
    pub maxcount: usize,
    pub slotname: Atom,
}
// 每个 frame 数据为 4 个浮点数, uOffset vOffset, uScale, vScale
impl BindEffectTextureTilloff {
    pub const SUFFIX: &'static str = "_Atlas";
    pub const ITEM_SIZE: usize = 4 * 4;
    pub fn new(bindbuffer: &mut BindBufferAllocator, maxcount: usize, slotname: Atom) -> Option<Self> {
        if let Some(buffer) = bindbuffer.allocate((maxcount * Self::ITEM_SIZE) as u32) {
            let mut data: Vec<f32> = Vec::with_capacity(maxcount * 4);
            for _ in 0..maxcount {
                data.push(0.); data.push(0.); data.push(0.); data.push(0.);
            }
            buffer.0.write_data(0, bytemuck::cast_slice(&data));
            Some(
                Self {
                    data: buffer,
                    maxcount,
                    slotname,
                }
            )
        } else {
            None
        }
    }
    pub fn update(&mut self, matidx: usize, sx: f32, sy: f32, ox: f32, oy: f32) {
        self.data.write_data(matidx * Self::ITEM_SIZE, bytemuck::cast_slice(&[sx, sy, ox, oy]));
    }
    fn define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += &self.slotname.to_string();
        result += "AtlasArr {";
        result += crate::prelude::S_BREAK;
        result += ShaderSetBind::code_uniform_array(&crate::prelude::S_VEC4, &(self.slotname.to_string() + Self::SUFFIX), self.maxcount as u32).as_str();
        result += "};";
        result
    }
}

impl TShaderBindCode for BindEffectTextureTilloff {
    fn vs_define_code(&self, set: u32, bind: u32) -> String {
        self.define_code(set, bind)
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        self.define_code(set, bind)
    }
}
impl TKeyBind for BindEffectTextureTilloff {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Buffer(
                KeyBindBuffer {
                    data: self.data.clone(),
                    layout: KeyBindLayoutBuffer {
                        visibility: EShaderStage::VERTEXFRAGMENT,
                        min_binding_size: self.data.size() as u32,
                    }
                }
            )
        )
    }
}
