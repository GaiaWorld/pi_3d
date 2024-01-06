
use pi_render::renderer::{
    sampler::*,
    bind::{KeyBindLayoutSampler, EKeyBind, KeyBindSampler}
};
use crate::shader::*;

pub fn sampler_key_bind(sampler: BindDataSampler, slotidx: usize, meta: &ShaderEffectMeta) -> Option<EKeyBind> {
    if let Some(layout) = sampler_key_layout(slotidx, meta) { Some(EKeyBind::Sampler(KeyBindSampler { data: sampler, layout: layout })) } else { None }
}
pub fn sampler_key_layout(slotidx: usize, meta: &ShaderEffectMeta) -> Option<KeyBindLayoutSampler> {
    if let Some(desc) = meta.textures.get(slotidx) { Some(KeyBindLayoutSampler { visibility: desc.stage, binding_type: desc.sampler_type() }) } else { None }
}

pub fn vs_define_sampler(meta: &ShaderEffectMeta, index: usize, set: u32, bind: u32) -> String {
    
    if let Some(desc) = meta.textures.get(index) {
        if  desc.stage.mode() & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
            return define_sampler(desc, index, set, bind);
        }
    }
    String::from("")
}
pub fn fs_define_sampler(meta: &ShaderEffectMeta, index: usize, set: u32, bind: u32) -> String {
    if let Some(desc) = meta.textures.get(index) {
        if  desc.stage.mode() & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
            return define_sampler(desc, index, set, bind);
        }
    }

    String::from("")
}
fn define_sampler(desc: &UniformTexture2DDesc, index: usize, set: u32, bind: u32) -> String {
    let mut result = String::from("");
    let idx = index.to_string();
    let slotname = String::from("_Texture") + &idx;
    result += sampler_bind_code(slotname.as_str(), desc.sampler_type(), set, bind).as_str();
    result += "#define sampler";
    result += desc.slotname.as_str();
    result += " sampler";
    result += slotname.as_str();
    result += "\r\n";
    result += "vec4 texture2D"; result += &idx; result += "(vec2 uv) {\r\n";
    // result += "    uv = floor(uv) + fract(uv) * uTexST"; result += &idx; result += ".xy + uTexST"; result += &idx; result += ".zw;\r\n";
    result += "    return texture(sampler2D(_Texture"; result += &idx; result += ", sampler_Texture"; result += &idx; result += "), uv);\r\n";
    result += "}\r\n";
    result
}