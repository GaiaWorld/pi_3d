use std::sync::Arc;

use bevy::prelude::Deref;
use pi_render::renderer::{
    texture::BindDataTexture2D, sampler::BindDataSampler, buildin_var::ShaderVarUniform, shader::TShaderBindCode,
    bind::{TKeyBind, KeyBindTexture2D, KeyBindLayoutTexture2D, KeyBindSampler, KeyBindLayoutSampler, KeyBindLayoutBindingType},
    shader_stage::EShaderStage
};
use crate::shader::{texture_bind_code, sampler_bind_code};

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindBRDFTexture(pub BindDataTexture2D);
impl ShaderBindBRDFTexture {
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        texture_bind_code(&wgpu::TextureSampleType::Float { filterable: true }, wgpu::TextureViewDimension::D2, ShaderVarUniform::BRDF_TEXUTRE, set, binding)
    }
    pub fn fs_define_code(&self, set: u32, binding: u32) -> String {
        self.vs_define_code(set, binding)
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
impl TShaderBindCode for BindUseBRDFTexture {
    fn vs_define_code(&self, set: u32) -> String {
        String::from("")
    }
    fn fs_define_code(&self, set: u32) -> String {
        self.data.vs_define_code(set, self.bind)
    }
}
impl TKeyBind for BindUseBRDFTexture {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Texture2D(
                KeyBindTexture2D {
                    data: self.data.0.clone(),
                    layout: Arc::new(
                        KeyBindLayoutTexture2D {
                            binding: self.bind as KeyBindLayoutBindingType,
                            visibility: EShaderStage::FRAGMENT,
                            texture_sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        }
                    ) 
                }
            )
        )
    }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindBRDFSampler(pub BindDataSampler);

impl ShaderBindBRDFSampler {
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        sampler_bind_code(ShaderVarUniform::BRDF_TEXUTRE, wgpu::SamplerBindingType::Filtering, set, binding)
    }
    pub fn fs_define_code(&self, set: u32, binding: u32) -> String {
        self.vs_define_code(set, binding)
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
impl TShaderBindCode for BindUseBRDFSampler {
    fn vs_define_code(&self, set: u32) -> String {
        self.data.vs_define_code(set, self.bind)
    }
    fn fs_define_code(&self, set: u32) -> String {
        self.vs_define_code(set)
    }
}
impl TKeyBind for BindUseBRDFSampler {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Sampler(
                KeyBindSampler {
                    data: self.data.0.clone(),
                    layout: Arc::new(
                        KeyBindLayoutSampler {
                            binding: self.bind as KeyBindLayoutBindingType,
                            visibility: EShaderStage::FRAGMENT,
                            binding_type: wgpu::SamplerBindingType::Filtering
                        }
                    ) 
                }
            )
        )
    }
}
