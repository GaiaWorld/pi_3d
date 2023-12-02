use std::sync::Arc;

use pi_render::renderer::{
    texture::*,
    bind::{KeyBindLayoutTexture2D, EKeyBind, KeyBindTexture2D, KeyBindLayoutBindingType},
};
use crate::shader::*;

pub fn texture_key_bind(tex: BindDataTexture2D, slotindex: usize, meta: &ShaderEffectMeta, binding: KeyBindLayoutBindingType) -> Option<EKeyBind> {
    if let Some(layout) = texture_key_layout(slotindex, meta, binding) { Some(EKeyBind::Texture2D(KeyBindTexture2D { data: tex, layout: Arc::new(layout)  })) } else { None }
}
pub fn texture_key_layout(slotindex: usize, meta: &ShaderEffectMeta, binding: KeyBindLayoutBindingType) -> Option<KeyBindLayoutTexture2D> {
    if let Some(desc) = meta.textures.get(slotindex) { Some(KeyBindLayoutTexture2D { binding, visibility: desc.stage, texture_sample_type: desc.tex_sampler_type, view_dimension: desc.dimision }) } else { None }
}


pub fn vs_define_texture(meta: &ShaderEffectMeta, index: usize, set: u32, bind: u32) -> String {
    let mut result = String::from("");
    
    if let Some(desc) = meta.textures.get(index) {
        if  desc.stage.mode() & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
            result += desc.vs_code(set, bind).as_str();
        }
    }

    result
}
pub fn fs_define_texture(meta: &ShaderEffectMeta, index: usize, set: u32, bind: u32) -> String {
    let mut result = String::from("");

    if let Some(desc) = meta.textures.get(index) {
        if  desc.stage.mode() & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
            result += desc.fs_code(set, bind).as_str();
        }
    }

    result
}