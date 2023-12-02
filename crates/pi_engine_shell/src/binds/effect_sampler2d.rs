use std::sync::Arc;

use pi_render::renderer::{
    sampler::*,
    bind::{KeyBindLayoutSampler, EKeyBind, KeyBindSampler, KeyBindLayoutBindingType}
};
use crate::shader::*;

pub fn sampler_key_bind(sampler: BindDataSampler, slotidx: usize, meta: &ShaderEffectMeta, binding: KeyBindLayoutBindingType) -> Option<EKeyBind> {
    if let Some(layout) = sampler_key_layout(slotidx, meta, binding) { Some(EKeyBind::Sampler(KeyBindSampler { data: sampler, layout: Arc::new(layout)  })) } else { None }
}
pub fn sampler_key_layout(slotidx: usize, meta: &ShaderEffectMeta, binding: KeyBindLayoutBindingType) -> Option<KeyBindLayoutSampler> {
    if let Some(desc) = meta.textures.get(slotidx) { Some(KeyBindLayoutSampler { binding, visibility: desc.stage, binding_type: desc.sampler_type() }) } else { None }
}

pub fn vs_define_sampler(meta: &ShaderEffectMeta, index: usize, set: u32, bind: u32) -> String {
    let mut result = String::from("");
    
    if let Some(desc) = meta.textures.get(index) {
        if  desc.stage.mode() & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
            result += sampler_bind_code(desc.slotname.as_str(), desc.sampler_type(), set, bind).as_str();
        }
    }

    result
}
pub fn fs_define_sampler(meta: &ShaderEffectMeta, index: usize, set: u32, bind: u32) -> String {
    let mut result = String::from("");

    if let Some(desc) = meta.textures.get(index) {
        if  desc.stage.mode() & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
            result += sampler_bind_code(desc.slotname.as_str(), desc.sampler_type(), set, bind).as_str();
        }
    }

    result
}