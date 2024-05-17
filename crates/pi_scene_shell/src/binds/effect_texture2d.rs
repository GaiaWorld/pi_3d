
use pi_render::renderer::{
    texture::*,
    bind::{KeyBindLayoutTexture2D, EKeyBind, KeyBindTexture2D},
};
use crate::shader::*;

pub fn texture_key_bind(tex: BindDataTexture2D, slotindex: usize, meta: &ShaderEffectMeta) -> Option<EKeyBind> {
    if let Some(layout) = texture_key_layout(slotindex, meta) { Some(EKeyBind::Texture2D(KeyBindTexture2D { data: tex, layout: layout })) } else { None }
}
pub fn texture_key_layout(slotindex: usize, meta: &ShaderEffectMeta) -> Option<KeyBindLayoutTexture2D> {
    if let Some(desc) = meta.textures.get(slotindex) { Some(KeyBindLayoutTexture2D { visibility: desc.stage, texture_sample_type: desc.tex_sampler_type, view_dimension: desc.dimision }) } else { None }
}


pub fn vs_define_texture(meta: &ShaderEffectMeta, index: usize, set: u32, bind: u32) -> String {
    
    if let Some(desc) = meta.textures.get(index) {
        if  desc.stage.mode() & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
            return define_texture(desc, index, set, bind);
        }
    }

    String::from("")
}
pub fn fs_define_texture(meta: &ShaderEffectMeta, index: usize, set: u32, bind: u32) -> String {
    if let Some(desc) = meta.textures.get(index) {
        if  desc.stage.mode() & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
            return define_texture(desc, index, set, bind);
        }
    }

    String::from("")
}

fn define_texture(desc: &UniformTexture2DDesc, index: usize, set: u32, bind: u32) -> String {
    let mut result = String::from("");
    // let idx = index.to_string();
    // let slotname = String::from("_Texture") + &idx;
    let slotname = desc.slotname.to_string();
    result += texture_bind_code_mat(&desc.tex_sampler_type, desc.dimision, &slotname, set, bind, index as u32).as_str();
    // result += "#define ";
    // result += desc.slotname.as_str();
    // result += crate::prelude::S_SPACE;
    // result += slotname.as_str();
    // result += crate::prelude::S_BREAK;
    // result += "vec4 texture2D"; result += &idx; result += "(vec2 uv) {\r\n";
    // // result += "    uv = floor(uv) + fract(uv) * uTexST"; result += &idx; result += ".xy + uTexST"; result += &idx; result += ".zw;\r\n";
    // result += "    return texture(sampler2D(_Texture"; result += &idx; result += ", sampler_Texture"; result += &idx; result += "), uv);\r\n";
    // result += "}\r\n";
    result
}