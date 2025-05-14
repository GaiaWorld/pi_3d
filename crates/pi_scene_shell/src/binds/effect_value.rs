use std::sync::Arc;

use pi_assets::asset::Handle;

use pi_render::{
    rhi::device::RenderDevice,
    renderer::{
        shader::{KeyShaderMeta, TShaderBindCode},
        bind_buffer::{BindBufferAllocator, BindBufferRange},
        bind::{TKeyBind, KeyBindBuffer, KeyBindLayoutBuffer},
        shader_stage::EShaderStage
    }
};

use crate::{prelude::{BindDefines, TBindDefine}, run_stage::EngineCustomPlugins, shader::*};

use super::BindEffectTextureInfo;

#[derive(Clone)]
pub struct ShaderBindEffectValueArr {
    pub total_size: usize,
    pub item_size: usize,
    pub mat4_count: u8,
    // pub mat2_count: u8,
    pub vec4_count: u8,
    pub vec2_count: u8,
    pub float_count: u8,
    // pub int_count: u8,
    pub uint_count: u8,

    pub fill_vec2_count: u8,
    pub fill_int_count: u8,
    
    pub mat4_begin: u32,
    // pub mat2_begin: u32,
    pub vec4_begin: u32,
    pub vec2_begin: u32,
    pub float_begin: u32,
    // pub int_begin: u32,
    pub uint_begin: u32,
    pub key_meta: KeyShaderMeta,
    pub meta: Handle<ShaderEffectMeta>,
    pub(crate) data: BindBufferRange,
    pub maxcount: u32,
}
impl ShaderBindEffectValueArr {
    pub const BIND: u32 = 0;

    pub const LABEL_MASK: &'static str = "#";
    pub const MAT4_BYTES: u32 = 16 * 4;
    pub const MAT2_BYTES: u32 = 4 * 4;
    pub const VEC4_BYTES: u32 = 4 * 4;
    pub const VEC2_BYTES: u32 = 2 * 4;
    pub const FLOAT_BYTES: u32 = 1 * 4;
    pub const INT_BYTES: u32 = 1 * 4;
    pub const UINT_BYTES: u32 = 1 * 4;

    pub fn new(
        device: &RenderDevice,
        key_meta: KeyShaderMeta,
        meta: Handle<ShaderEffectMeta>,
        allocator: &mut BindBufferAllocator,
        maxlen_material_array: u32
    ) -> Option<Self> {
        let limit = device.limits();
        let uniforms = &meta.uniforms;
        let mat4_count      = uniforms.mat4_list.len() as u8;
        // let mat2_count      = uniforms.mat2_list.len() as u8;
        let vec4_count      = uniforms.vec4_list.len() as u8;
        let vec3_count      = uniforms.vec3_list.len() as u8;
        let vec2_count      = uniforms.vec2_list.len() as u8;
        let float_count     = uniforms.float_list.len() as u8;
        let int_count       = 0; // uniforms.int_list.len() as u8;
        let uint_count      = uniforms.uint_list.len() as u8;
        // let align_bytes     = 16;
        
        let mut fill_vec2_count    = vec2_count % 2;
        fill_vec2_count = if fill_vec2_count == 0 { 0 } else { 2 - fill_vec2_count };
        let mut fill_int_count     = (float_count + int_count + uint_count) % 4;
        fill_int_count = if fill_int_count == 0 { 0 } else { 4 - fill_int_count };

        let mut total_size = 0;

        let mat4_begin: u32  = total_size;
        total_size += mat4_count as u32 * Self::MAT4_BYTES;

        // let mat2_begin: u32  = total_size;
        // total_size += mat2_count as u32 * Self::MAT2_BYTES;

        let vec4_begin: u32  = total_size;
        total_size += vec4_count as u32 * Self::VEC4_BYTES;
        
        // let vec4_begin: u32  = total_size;
        total_size += vec3_count as u32 * Self::VEC4_BYTES;

        let vec2_begin: u32  = total_size;
        total_size += (vec2_count as u32 + fill_vec2_count as u32) * Self::VEC2_BYTES;

        let float_begin: u32 = total_size;
        total_size += float_count as u32 * Self::FLOAT_BYTES;

        // let int_begin: u32   = total_size;
        // total_size += int_count as u32 * Self::INT_BYTES;

        let uint_begin: u32  = total_size;
        total_size += uint_count as u32 * Self::UINT_BYTES;

        total_size += fill_int_count as u32 * Self::INT_BYTES;

        // if total_size == 0 {
        //     total_size += 4 * Self::UINT_BYTES; // 4 个 占位u32; 对应 MaterialValueBindDesc 中也有处理
        // }

        if total_size == 0 {
            None
        } else {
            let step = 4 * 4;
            let item_size = (total_size + step - 1) / step * step;
            let maxcount = maxlen_material_array.min(limit.max_uniform_buffer_binding_size / item_size).min(limit.max_uniform_buffer_binding_size / BindEffectTextureInfo::ITEM_SIZE as u32);
            // let maxcount = 1;
            total_size = item_size * maxcount;
            match allocator.allocate(total_size ) {
                Some(data) => {
                    Some(
                        Self {
                            total_size: total_size as usize,
                            item_size: item_size as usize,
                            mat4_count,
                            // mat2_count,
                            vec4_count,
                            vec2_count,
                            float_count,
                            // int_count,
                            uint_count,
                            fill_vec2_count,
                            fill_int_count,
                            mat4_begin,
                            // mat2_begin,
                            vec4_begin,
                            vec2_begin,
                            float_begin,
                            // int_begin,
                            uint_begin,
                            key_meta,
                            meta,
                            data,
                            maxcount
                        }
                    )
                },
                None => None,
            }
        }
    }
    
    pub fn label(
        &self
    ) -> String {
        String::from("")
        + Self::LABEL_MASK + &self.mat4_count.to_string() 
        // + Self::LABEL_MASK + &self.mat2_count.to_string() 
        + Self::LABEL_MASK + &self.vec4_count.to_string() 
        + Self::LABEL_MASK + &self.vec2_count.to_string() 
        + Self::LABEL_MASK + &self.float_count.to_string() 
        // + Self::LABEL_MASK + &self.int_count.to_string()
        + Self::LABEL_MASK + &self.uint_count.to_string()
    }

    pub fn data(&self) -> &BindBufferRange {
        &self.data
    }

}
impl std::hash::Hash for ShaderBindEffectValueArr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_meta.hash(state);
        self.data.id_buffer().hash(state);
    }
}
impl PartialEq for ShaderBindEffectValueArr {
    fn eq(&self, other: &Self) -> bool {
        self.key_meta == other.key_meta && self.data.id_buffer() == other.data.id_buffer()
    }
}
impl Eq for ShaderBindEffectValueArr {
    fn assert_receiver_is_total_eq(&self) {}
}
impl TKeyBind for ShaderBindEffectValueArr {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Buffer(
                KeyBindBuffer {
                    data: self.data.clone(),
                    layout: KeyBindLayoutBuffer {
                        visibility: EShaderStage::VERTEXFRAGMENT,
                        dynamic: self.data.2,
                        min_binding_size: self.total_size as u32,
                    }
                }
            )
        )
    }
}
impl ShaderBindEffectValueArr {
    pub fn vs_define_code(&self, set: u32, bind: u32, engineopt: &EngineCustomPlugins) -> String {
        self.meta.uniforms.vs_code(set, bind, self.maxcount, engineopt)
    }
    pub fn fs_define_code(&self, set: u32, bind: u32, engineopt: &EngineCustomPlugins) -> String {
        self.meta.uniforms.fs_code(set, bind, self.maxcount, engineopt)
    }
}
impl TBindDefine for ShaderBindEffectValueArr {
    fn bind_include(&self) -> u32 {
        BindDefines::EFFECT_VALUE_BIND
    }
}
