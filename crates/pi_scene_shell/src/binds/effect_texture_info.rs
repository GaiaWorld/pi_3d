use pi_render::renderer::bind::{KeyBindBuffer, KeyBindLayoutBuffer, TKeyBind};

use crate::{prelude::*, run_stage::EngineCustomPlugins};


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindEffectTextureInfo {
    pub data: BindBufferRange,
    pub maxcount: u32,
}
// 每个 frame 数据为 4 个浮点数, uOffset vOffset, uScale, vScale
impl BindEffectTextureInfo {
    pub const SUFFIX_ADDRESS: &'static str = "_Address";
    pub const ADDRESS_SIZE: usize = 4 * 4;
    pub const ITEM_SIZE: usize = 4 * 4;
    pub fn new(maxcount: u32, bindbuffer: &mut BindBufferAllocator, _engineopt: &EngineCustomPlugins) -> Option<Self> {
        if let Some(buffer) = bindbuffer.allocate(maxcount * Self::ITEM_SIZE as u32) {
            let tilloff = [0u32, 0, 0, 0];
            let val = bytemuck::cast_slice(&tilloff);
            for i in 0..maxcount {
                buffer.0.write_data(i as usize * Self::ITEM_SIZE, val);
            }

            Some(
                Self {
                    data: buffer,
                    maxcount
                }
            )
        } else {
            None
        }
    }
    pub fn update(&self, matidx: usize, wrap_u: u8, wrap_v: u8, wrap_w: u8, coord: u8) {
        self.data.write_data(matidx * Self::ITEM_SIZE, bytemuck::cast_slice(&[wrap_u as u32, wrap_v as u32, wrap_w as u32, coord as u32]));
    }
    fn define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += ShaderSetBind::code_uniform_array(&crate::prelude::S_UVEC4, &(slotname.to_string() + Self::SUFFIX_ADDRESS), self.maxcount as u32).as_str();
        result
    }
}

impl BindEffectTextureInfo {
    pub fn vs_define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        self.define_code(set, bind, slotname)
    }
    pub fn fs_define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        self.define_code(set, bind, slotname)
    }
}
impl TKeyBind for BindEffectTextureInfo {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Buffer(
                KeyBindBuffer {
                    data: self.data.clone(),
                    layout: KeyBindLayoutBuffer {
                        visibility: EShaderStage::VERTEXFRAGMENT,
                        dynamic: self.data.2,
                        min_binding_size: self.data.size() as u32,
                    }
                }
            )
        )
    }
}
impl TBindDefine for BindEffectTextureInfo {
    fn bind_include(&self) -> u32 {
        BindDefines::EFFECT_TEXTURE_ATLAS
    }
}


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindEffectTextureInfoAndTilloff {
    pub data: BindBufferRange,
    pub maxcount: u32,
}
// 每个 frame 数据为 4 个浮点数, uOffset vOffset, uScale, vScale
impl BindEffectTextureInfoAndTilloff {
    pub const SUFFIX_ADDRESS: &'static str = "_Address";
    pub const SUFFIX_TILLOFF: &'static str = "_Atlas";
    pub const TILLOFF_SIZE: usize = 4 * 4;
    pub const ADDRESS_SIZE: usize = 4 * 4;
    pub const ITEM_SIZE: usize = 4 * 4 * 2;
    pub fn new(maxcount: u32, bindbuffer: &mut BindBufferAllocator, _engineopt: &EngineCustomPlugins) -> Option<Self> {
        if let Some(buffer) = bindbuffer.allocate(maxcount * Self::ITEM_SIZE as u32) {
            let tilloff = [1f32, 1f32, 0f32, 0f32];
            let val = bytemuck::cast_slice(&tilloff);
            for i in 0..maxcount {
                buffer.0.write_data(i as usize * Self::TILLOFF_SIZE, val);
            }
            let address = [0u32, 0u32, 0u32, 0u32];
            let val = bytemuck::cast_slice(&address);
            for i in 0..maxcount {
                buffer.0.write_data(maxcount as usize * Self::TILLOFF_SIZE + i as usize * Self::ADDRESS_SIZE, val);
            }

            Some(
                Self {
                    data: buffer,
                    maxcount
                }
            )
        } else {
            None
        }
    }
    pub fn update(&self, matidx: usize, tilloff: &[u8], wrap_u: u8, wrap_v: u8, wrap_w: u8, coord: u8) {
        self.data.write_data(matidx * Self::TILLOFF_SIZE, tilloff);
        self.data.write_data(self.maxcount as usize * Self::TILLOFF_SIZE + matidx * Self::ADDRESS_SIZE, bytemuck::cast_slice(&[wrap_u as u32, wrap_v as u32, wrap_w as u32, coord as u32]));

    }
    fn define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += slotname;
        result += "InfoArr {";
        result += crate::prelude::S_BREAK;
        result += ShaderSetBind::code_uniform_array(&crate::prelude::S_VEC4, &(slotname.to_string() + Self::SUFFIX_TILLOFF), self.maxcount as u32).as_str();
        result += ShaderSetBind::code_uniform_array(&crate::prelude::S_UVEC4, &(slotname.to_string() + Self::SUFFIX_ADDRESS), self.maxcount as u32).as_str();
        result += "};\n";
        result
    }
}

impl BindEffectTextureInfoAndTilloff {
    pub fn vs_define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        self.define_code(set, bind, slotname)
    }
    pub fn fs_define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        self.define_code(set, bind, slotname)
    }
}
impl TKeyBind for BindEffectTextureInfoAndTilloff {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Buffer(
                KeyBindBuffer {
                    data: self.data.clone(),
                    layout: KeyBindLayoutBuffer {
                        visibility: EShaderStage::VERTEXFRAGMENT,
                        dynamic: self.data.2,
                        min_binding_size: self.data.size() as u32,
                    }
                }
            )
        )
    }
}
impl TBindDefine for BindEffectTextureInfoAndTilloff {
    fn bind_include(&self) -> u32 {
        BindDefines::EFFECT_TEXTURE_ATLAS
    }
}
