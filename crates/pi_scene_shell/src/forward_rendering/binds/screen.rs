use std::sync::Arc;

use derive_deref::Deref;
use pi_render::renderer::{
    texture::BindDataTexture2D, sampler::BindDataSampler, buildin_var::ShaderVarUniform, shader::TShaderBindCode,
    bind::{TKeyBind, KeyBindTexture2D, KeyBindLayoutTexture2D, KeyBindSampler, KeyBindLayoutSampler},
    shader_stage::EShaderStage
};
use crate::shader::{texture_bind_code, sampler_bind_code};


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindMainCameraOpaqueTexture(pub BindDataTexture2D);
impl TShaderBindCode for ShaderBindMainCameraOpaqueTexture {
    fn fs_define_code(&self, set: u32, binding: u32) -> String {
        texture_bind_code(&wgpu::TextureSampleType::Float { filterable: true }, wgpu::TextureViewDimension::D2, ShaderVarUniform::CAMERA_OPAQUE_TEXUTRE, set, binding)
    }
}
impl TKeyBind for ShaderBindMainCameraOpaqueTexture {
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
pub struct BindUseMainCameraOpaqueTexture {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindMainCameraOpaqueTexture>,
}
impl BindUseMainCameraOpaqueTexture {
    pub fn new(bind: u32, data: Arc<ShaderBindMainCameraOpaqueTexture>) -> Self {
        Self { bind, data }
    }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindMainCameraOpaqueSampler(pub BindDataSampler);

impl TShaderBindCode for ShaderBindMainCameraOpaqueSampler {
    fn fs_define_code(&self, set: u32, binding: u32) -> String {
        sampler_bind_code(ShaderVarUniform::CAMERA_OPAQUE_TEXUTRE, wgpu::SamplerBindingType::Filtering, set, binding)
    }
}
impl TKeyBind for ShaderBindMainCameraOpaqueSampler {
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
pub struct BindUseMainCameraOpaqueSampler {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindMainCameraOpaqueSampler>,
}
impl BindUseMainCameraOpaqueSampler {
    pub fn new(bind: u32, data: Arc<ShaderBindMainCameraOpaqueSampler>) -> Self {
        Self { bind, data }
    }
}


///////////// Depth Texture
#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindMainCameraDepthTexture(pub BindDataTexture2D);
impl TShaderBindCode for ShaderBindMainCameraDepthTexture {
    fn fs_define_code(&self, set: u32, binding: u32) -> String {
        texture_bind_code(&wgpu::TextureSampleType::Float { filterable: true }, wgpu::TextureViewDimension::D2, ShaderVarUniform::CAMERA_DEPTH_TEXUTRE, set, binding)
    }
}
impl TKeyBind for ShaderBindMainCameraDepthTexture {
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
pub struct BindUseMainCameraDepthTexture {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindMainCameraDepthTexture>,
}
impl BindUseMainCameraDepthTexture {
    pub fn new(bind: u32, data: Arc<ShaderBindMainCameraDepthTexture>) -> Self {
        Self { bind, data }
    }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindMainCameraDepthSampler(pub BindDataSampler);
impl TShaderBindCode for ShaderBindMainCameraDepthSampler {
    fn fs_define_code(&self, set: u32, binding: u32) -> String {
        sampler_bind_code(ShaderVarUniform::CAMERA_DEPTH_TEXUTRE, wgpu::SamplerBindingType::Filtering, set, binding)
    }
}
impl TKeyBind for ShaderBindMainCameraDepthSampler {
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
pub struct BindUseMainCameraDepthSampler {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindMainCameraDepthSampler>,
}
impl BindUseMainCameraDepthSampler {
    pub fn new(bind: u32, data: Arc<ShaderBindMainCameraDepthSampler>) -> Self {
        Self { bind, data }
    }
}
