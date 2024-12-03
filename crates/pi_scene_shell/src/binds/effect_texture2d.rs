
use pi_render::renderer::{
    bind::{EKeyBind, KeyBindLayoutTexture2D, KeyBindTexture2D}, shader_stage::EShaderStage, texture::*
};
use crate::shader::*;

pub fn texture_key_bind(tex: BindDataTexture2D, visibility: EShaderStage, texture_sample_type: wgpu::TextureSampleType, view_dimension: wgpu::TextureViewDimension) -> EKeyBind {
    EKeyBind::Texture2D(KeyBindTexture2D { data: tex, layout: KeyBindLayoutTexture2D { visibility, texture_sample_type, view_dimension } })
}

pub fn effect_texture_bind_name(idx: u16) -> String {
    String::from("EffecTex") + &idx.to_string()
}