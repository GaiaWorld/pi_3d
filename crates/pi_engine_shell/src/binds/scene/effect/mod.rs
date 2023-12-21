use std::sync::Arc;

use pi_render::renderer::{
    bind_buffer::{BindBufferAllocator, BindBufferRange},
    shader::TShaderBindCode,
    buildin_var::ShaderVarUniform,
    bind::{TKeyBind, KeyBindBuffer, KeyBindLayoutBuffer},
    shader_stage::EShaderStage
};
use crate::shader::*;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindSceneAboutEffect {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindSceneAboutEffect {

    pub const OFFSET_TIME:                  wgpu::DynamicOffset = 0;
    pub const SIZE_TIME:                    wgpu::DynamicOffset = 4 * 4;
    pub const OFFSET_DELTA_TIME:            wgpu::DynamicOffset = Self::OFFSET_TIME + Self::SIZE_TIME;
    pub const SIZE_DELTA_TIME:              wgpu::DynamicOffset = 4 * 4;

    pub const OFFSET_FOG_INFO:              wgpu::DynamicOffset = Self::OFFSET_DELTA_TIME;
    pub const SIZE_FOG_INFO:                wgpu::DynamicOffset = 4 * 4;
    pub const OFFSET_FOG_PARAM:             wgpu::DynamicOffset = Self::OFFSET_FOG_INFO + Self::SIZE_FOG_INFO;
    pub const SIZE_FOG_PARAM:               wgpu::DynamicOffset = 4 * 4;

    pub const OFFSET_AMBIENT:               wgpu::DynamicOffset = Self::OFFSET_FOG_PARAM + Self::SIZE_FOG_PARAM;
    pub const SIZE_AMBIENT:                 wgpu::DynamicOffset = 4 * 4;

    pub const TOTAL_SIZE:                   wgpu::DynamicOffset = Self::OFFSET_AMBIENT + Self::SIZE_AMBIENT;

    pub fn new(
        allocator: &mut BindBufferAllocator,
    ) -> Option<Self> {
        if let Some(data) = allocator.allocate(Self::TOTAL_SIZE) {
            Some(Self { data })
        } else {
            None
        }
    }
    pub fn key_layout(&self) -> KeyBindLayoutBuffer {
        KeyBindLayoutBuffer {
            visibility: EShaderStage::VERTEXFRAGMENT,
            min_binding_size: self.data.size(),
        }
    }
    pub fn data(&self) -> &BindBufferRange {
        &self.data
    }
}
impl TShaderBindCode for ShaderBindSceneAboutEffect {
    fn vs_define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += " SceneEffect {\r\n";
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::TIME).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::DELTA_TIME).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::FOG_INFO).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::FOG_PARAM).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::AMBIENT_PARAM).as_str();
        result += "};\r\n";
        result
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        self.vs_define_code(set, bind)
    }
}
impl TKeyBind for ShaderBindSceneAboutEffect {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Buffer(
                KeyBindBuffer {
                    data: self.data.clone(),
                    layout: KeyBindLayoutBuffer {
                        visibility: EShaderStage::VERTEXFRAGMENT,
                        min_binding_size: self.data.size()
                    }
                }
            )
        )
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BindUseSceneAboutEffect {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindSceneAboutEffect>,
}
impl BindUseSceneAboutEffect {
    pub fn new(bind: u32, data: Arc<ShaderBindSceneAboutEffect>) -> Self {
        Self { bind, data }
    }
}