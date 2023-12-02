use std::sync::Arc;

use bevy::prelude::Deref;
use pi_render::renderer::{
    texture::BindDataTexture2D, sampler::BindDataSampler, buildin_var::ShaderVarUniform, shader::TShaderBindCode,
    bind::{TKeyBind, KeyBindTexture2D, KeyBindLayoutTexture2D, KeyBindSampler, KeyBindLayoutSampler, KeyBindLayoutBindingType},
    shader_stage::EShaderStage
};
use crate::shader::{texture_bind_code, sampler_bind_code};


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindMainCameraOpaqueTexture(pub BindDataTexture2D);
impl ShaderBindMainCameraOpaqueTexture {
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        texture_bind_code(&wgpu::TextureSampleType::Float { filterable: true }, wgpu::TextureViewDimension::D2, ShaderVarUniform::CAMERA_OPAQUE_TEXUTRE, set, binding)
    }
    pub fn fs_define_code(&self, set: u32, binding: u32) -> String {
        self.vs_define_code(set, binding)
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
impl TShaderBindCode for BindUseMainCameraOpaqueTexture {
    fn vs_define_code(&self, set: u32) -> String {
        self.data.vs_define_code(set, self.bind)
    }
    fn fs_define_code(&self, set: u32) -> String {
        self.vs_define_code(set)
    }
}
impl TKeyBind for BindUseMainCameraOpaqueTexture {
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
pub struct ShaderBindMainCameraOpaqueSampler(pub BindDataSampler);

impl ShaderBindMainCameraOpaqueSampler {
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        sampler_bind_code(ShaderVarUniform::CAMERA_OPAQUE_TEXUTRE, wgpu::SamplerBindingType::Filtering, set, binding)
    }
    pub fn fs_define_code(&self, set: u32, binding: u32) -> String {
        self.vs_define_code(set, binding)
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
impl TShaderBindCode for BindUseMainCameraOpaqueSampler {
    fn vs_define_code(&self, set: u32) -> String {
        self.data.vs_define_code(set, self.bind)
    }
    fn fs_define_code(&self, set: u32) -> String {
        self.vs_define_code(set)
    }
}
impl TKeyBind for BindUseMainCameraOpaqueSampler {
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


///////////// Depth Texture
#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindMainCameraDepthTexture(pub BindDataTexture2D);
impl ShaderBindMainCameraDepthTexture {
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        texture_bind_code(&wgpu::TextureSampleType::Float { filterable: true }, wgpu::TextureViewDimension::D2, ShaderVarUniform::CAMERA_DEPTH_TEXUTRE, set, binding)
    }
    pub fn fs_define_code(&self, set: u32, binding: u32) -> String {
        self.vs_define_code(set, binding)
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
impl TShaderBindCode for BindUseMainCameraDepthTexture {
    fn vs_define_code(&self, set: u32) -> String {
        self.data.vs_define_code(set, self.bind)
    }
    fn fs_define_code(&self, set: u32) -> String {
        self.vs_define_code(set)
    }
}
impl TKeyBind for BindUseMainCameraDepthTexture {
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
pub struct ShaderBindMainCameraDepthSampler(pub BindDataSampler);
impl ShaderBindMainCameraDepthSampler {
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        sampler_bind_code(ShaderVarUniform::CAMERA_DEPTH_TEXUTRE, wgpu::SamplerBindingType::Filtering, set, binding)
    }
    pub fn fs_define_code(&self, set: u32, binding: u32) -> String {
        self.vs_define_code(set, binding)
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
impl TShaderBindCode for BindUseMainCameraDepthSampler {
    fn vs_define_code(&self, set: u32) -> String {
        self.data.vs_define_code(set, self.bind)
    }
    fn fs_define_code(&self, set: u32) -> String {
        self.vs_define_code(set)
    }
}
impl TKeyBind for BindUseMainCameraDepthSampler {
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

