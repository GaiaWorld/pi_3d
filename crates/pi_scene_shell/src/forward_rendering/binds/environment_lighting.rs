use std::sync::Arc;

use derive_deref::Deref;
use pi_assets::asset::Handle;
use pi_render::renderer::{
    texture::{BindDataTexture2D, ImageTexture}, sampler::BindDataSampler, buildin_var::ShaderVarUniform, shader::TShaderBindCode,
    bind::{TKeyBind, KeyBindTexture2D, KeyBindLayoutTexture2D, KeyBindSampler, KeyBindLayoutSampler, KeyBindLayoutBuffer, KeyBindBuffer},
    shader_stage::EShaderStage, bind_buffer::{BindBufferRange, BindBufferAllocator}
};
use crate::shader::{texture_bind_code, ShaderSetBind};

use crate::assets::environment_texture_loader::EnvironmentTextureTools;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindEnvIrradiance {
    pub(crate) data: BindBufferRange,
}
impl BindEnvIrradiance {
    const KEY: &'static str = "IBL";
    pub fn new(
        allocator: &mut BindBufferAllocator,
        texture: &Handle<ImageTexture>,
    ) -> Option<Self> {
        let size = texture.extend.len();
        if size == EnvironmentTextureTools::IRRADIANCE_SIZE {
            if let Some(data) = allocator.allocate( size as wgpu::DynamicOffset ) {
                data.0.write_data( 0, &texture.extend);
                Some(Self { data })
            } else {
                None
            }
        } else  { None }
    }
    pub fn key_layout(&self) -> KeyBindLayoutBuffer {
        KeyBindLayoutBuffer {
            visibility: EShaderStage::FRAGMENT,
            min_binding_size: self.data.size(),
        }
    }
    pub fn data(&self) -> &BindBufferRange {
        &self.data
    }
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, binding).as_str();
        result += " ";
        result += Self::KEY;
        result += " {\r\n";
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_X).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_Y).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_Z).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_XX_ZZ).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_YY_ZZ).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_ZZ).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_XY).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_YZ).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_ZX).as_str();
        result += "};\r\n";
        result
    }
    pub fn fs_define_code(&self, set: u32, binding: u32) -> String {
        self.vs_define_code(set, binding)
    }
}
impl TShaderBindCode for BindEnvIrradiance {
    fn vs_define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += " ";
        result += Self::KEY;
        result += " {\r\n";
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_X).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_Y).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_Z).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_XX_ZZ).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_YY_ZZ).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_ZZ).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_XY).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_YZ).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::IBL_ZX).as_str();
        result += "};\r\n";
        result
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        self.vs_define_code(set, bind)
    }
}
impl TKeyBind for BindEnvIrradiance {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Buffer(
                KeyBindBuffer {
                    data: self.data.clone(),
                    layout: KeyBindLayoutBuffer {
                        visibility: EShaderStage::FRAGMENT,
                        min_binding_size: self.data.size()
                    }
                }
            )
        )
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindUseEnvIrradiance {
    pub(crate) bind: u32,
    pub(crate) data: Arc<BindEnvIrradiance>,
}
impl BindUseEnvIrradiance {
    pub fn new(bind: u32, data: Arc<BindEnvIrradiance>) -> Self {
        Self { bind, data }
    }
}

#[derive(Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindEnvTexture(pub BindDataTexture2D);
impl TShaderBindCode for ShaderBindEnvTexture {
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        texture_bind_code(&wgpu::TextureSampleType::Float { filterable: true }, wgpu::TextureViewDimension::Cube, ShaderVarUniform::ENVIRONMENT_TEXUTRE, set, bind)
    }
}
impl TKeyBind for ShaderBindEnvTexture {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Texture2D(
                KeyBindTexture2D {
                    data: self.0.clone(),
                    layout: KeyBindLayoutTexture2D {
                        visibility: EShaderStage::FRAGMENT,
                        texture_sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::Cube,
                    }
                }
            )
        )
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindUseEnvTexture {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindEnvTexture>,
}
impl BindUseEnvTexture {
    pub fn new(bind: u32, data: Arc<ShaderBindEnvTexture>) -> Self {
        Self { bind, data }
    }
}

#[derive(Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindEnvSampler(pub BindDataSampler);

impl ShaderBindEnvSampler {
    pub fn vs_define_code(&self, _set: u32, _binding: u32) -> String {
        String::from("")
    }
    pub fn fs_define_code(&self, set: u32, binding: u32) -> String {
        // self.vs_define_code(set, binding)
        let mut result = ShaderSetBind::code_set_bind_head(set, binding);
        result += " sampler ";
        result += "sampler";
        result += ShaderVarUniform::ENVIRONMENT_TEXUTRE;
        result += ";\r\n";
        // sampler_bind_code(ShaderVarUniform::ENVIRONMENT_TEXUTRE, wgpu::SamplerBindingType::Filtering, set, binding)
        result
    }
}
impl TShaderBindCode for ShaderBindEnvSampler {
    fn vs_define_code(&self, _set: u32, _bind: u32) -> String {
        String::from("")
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        // self.vs_define_code(set, binding)
        let mut result = ShaderSetBind::code_set_bind_head(set, bind);
        result += " sampler ";
        result += "sampler";
        result += ShaderVarUniform::ENVIRONMENT_TEXUTRE;
        result += ";\r\n";
        // sampler_bind_code(ShaderVarUniform::ENVIRONMENT_TEXUTRE, wgpu::SamplerBindingType::Filtering, set, binding)
        result
    }
}
impl TKeyBind for ShaderBindEnvSampler {
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

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindUseEnvSampler {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindEnvSampler>,
}
impl BindUseEnvSampler {
    pub fn new(bind: u32, data: Arc<ShaderBindEnvSampler>) -> Self {
        Self { bind, data }
    }
}
