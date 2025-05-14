use pi_render::renderer::bind::{KeyBindBuffer, KeyBindLayoutBuffer, TKeyBind};

use crate::{prelude::*, run_stage::EngineCustomPlugins};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindEffectTextureTilloff {
    pub data: BindBufferRange,
    pub maxcount: u32,
}
// 每个 frame 数据为 4 个浮点数, uOffset vOffset, uScale, vScale
impl BindEffectTextureTilloff {
    pub const SUFFIX_TILLOFF: &'static str = "_Atlas";
    pub const TILLOFF_SIZE: usize = 4 * 4;
    pub fn new(maxcount: u32, bindbuffer: &mut BindBufferAllocator, _engineopt: &EngineCustomPlugins) -> Option<Self> {
        if let Some(buffer) = bindbuffer.allocate(maxcount * Self::TILLOFF_SIZE as u32) {
            let tilloff = [1f32, 1., 0., 0.];
            let val = bytemuck::cast_slice(&tilloff);
            for i in 0..maxcount {
                buffer.0.write_data(i as usize * Self::TILLOFF_SIZE, val);
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
    pub fn update(&self, matidx: usize, tilloff: &[u8]) {
        self.data.write_data(matidx * Self::TILLOFF_SIZE, tilloff);
    }
    fn define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += ShaderSetBind::code_uniform_array(&crate::prelude::S_VEC4, &(slotname.to_string() + Self::SUFFIX_TILLOFF), self.maxcount as u32).as_str();
        result
    }
}

impl BindEffectTextureTilloff {
    pub fn vs_define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        self.define_code(set, bind, slotname)
    }
    pub fn fs_define_code(&self, set: u32, bind: u32, slotname: &str) -> String {
        self.define_code(set, bind, slotname)
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
                        dynamic: self.data.2,
                        min_binding_size: self.data.size() as u32,
                    }
                }
            )
        )
    }
}
impl TBindDefine for BindEffectTextureTilloff {
    fn bind_include(&self) -> u32 {
        BindDefines::EFFECT_TEXTURE_ATLAS
    }
}
