use std::sync::Arc;

use bevy::prelude::Deref;
use pi_render::renderer::{
    texture::BindDataTexture2D, sampler::BindDataSampler, buildin_var::ShaderVarUniform, shader::TShaderBindCode,
    bind::{TKeyBind, KeyBindTexture2D, KeyBindLayoutTexture2D, KeyBindSampler, KeyBindLayoutSampler},
    shader_stage::EShaderStage
};
use crate::shader::{texture_bind_code, sampler_bind_code};

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindBRDFTexture(pub BindDataTexture2D);
impl TShaderBindCode for ShaderBindBRDFTexture {
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        texture_bind_code(&wgpu::TextureSampleType::Float { filterable: true }, wgpu::TextureViewDimension::D2, ShaderVarUniform::BRDF_TEXUTRE, set, bind)
    }
}
impl TKeyBind for ShaderBindBRDFTexture {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Texture2D(
                KeyBindTexture2D {
                    data: self.0.clone(),
                    layout: KeyBindLayoutTexture2D {
                        visibility: EShaderStage::FRAGMENT,
                        texture_sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    }
                }
            )
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BindUseBRDFTexture {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindBRDFTexture>,
}
impl BindUseBRDFTexture {
    pub fn new(bind: u32, data: Arc<ShaderBindBRDFTexture>) -> Self {
        Self { bind, data }
    }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindBRDFSampler(pub BindDataSampler);
impl TShaderBindCode for ShaderBindBRDFSampler {
    fn vs_define_code(&self, _set: u32, _bind: u32) -> String {
        String::from("")
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        sampler_bind_code(ShaderVarUniform::BRDF_TEXUTRE, wgpu::SamplerBindingType::Filtering, set, bind)
    }
}
impl TKeyBind for ShaderBindBRDFSampler {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Sampler(
                KeyBindSampler {
                    data: self.0.clone(),
                    layout: KeyBindLayoutSampler {
                        visibility: EShaderStage::FRAGMENT,
                        binding_type: wgpu::SamplerBindingType::Filtering
                    }
                }
            )
        )
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BindUseBRDFSampler {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindBRDFSampler>,
}
impl BindUseBRDFSampler {
    pub fn new(bind: u32, data: Arc<ShaderBindBRDFSampler>) -> Self {
        Self { bind, data }
    }
}
