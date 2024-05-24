use std::sync::Arc;

use pi_render::renderer::{
    bind::{KeyBindBuffer, KeyBindLayoutBuffer, TKeyBind}, bind_buffer::{BindBufferAllocator, BindBufferRange}, buffer::RWBufferRange, buildin_var::ShaderVarUniform, shader::TShaderBindCode, shader_stage::EShaderStage
};
use crate::shader::*;


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindViewer {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindViewer {

    pub const OFFSET_VIEW_MATRIX:           wgpu::DynamicOffset = 0;
    pub const SIZE_VIEW_MATRIX:             wgpu::DynamicOffset = 16 * 4;
    pub const OFFSET_PROJECT_MATRIX:        wgpu::DynamicOffset = Self::OFFSET_VIEW_MATRIX + Self::SIZE_VIEW_MATRIX;
    pub const SIZE_PROJECT_MATRIX:          wgpu::DynamicOffset = 16 * 4;
    pub const OFFSET_VIEW_PROJECT_MATRIX:   wgpu::DynamicOffset = Self::OFFSET_PROJECT_MATRIX + Self::SIZE_PROJECT_MATRIX;
    pub const SIZE_VIEW_PROJECT_MATRIX:     wgpu::DynamicOffset = 16 * 4;
    pub const OFFSET_CAMERA_POSITION:       wgpu::DynamicOffset = Self::OFFSET_VIEW_PROJECT_MATRIX + Self::SIZE_VIEW_PROJECT_MATRIX;
    pub const SIZE_CAMERA_POSITION:         wgpu::DynamicOffset = 4 * 4;
    pub const OFFSET_CAMERA_DIRECTION:      wgpu::DynamicOffset = Self::OFFSET_CAMERA_POSITION + Self::SIZE_CAMERA_POSITION;
    pub const SIZE_CAMERA_DIRECTION:        wgpu::DynamicOffset = 4 * 4;
    pub const OFFSET_CAMERA_ROTATION:       wgpu::DynamicOffset = Self::OFFSET_CAMERA_DIRECTION + Self::SIZE_CAMERA_DIRECTION;
    pub const SIZE_CAMERA_ROTATION:         wgpu::DynamicOffset = 16 * 4;
    
    pub const TOTAL_SIZE:                   wgpu::DynamicOffset = Self::OFFSET_CAMERA_ROTATION + Self::SIZE_CAMERA_ROTATION;
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
impl TShaderBindCode for ShaderBindViewer {
    fn vs_define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += " Camera {\r\n";
        result += ShaderSetBind::code_uniform("mat4", ShaderVarUniform::VIEW_MATRIX).as_str();
        result += ShaderSetBind::code_uniform("mat4", ShaderVarUniform::PROJECT_MATRIX).as_str();
        result += ShaderSetBind::code_uniform("mat4", ShaderVarUniform::VIEW_PROJECT_MATRIX).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::CAMERA_POSITION).as_str();
        result += ShaderSetBind::code_uniform("vec4", ShaderVarUniform::CAMERA_DIRECTION).as_str();
        result += ShaderSetBind::code_uniform("mat4", ShaderVarUniform::VIEW_ROTATION_MATRIX_INV).as_str();
        result += "};\r\n";
        result
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        self.vs_define_code(set, bind)
    }
}
impl TKeyBind for ShaderBindViewer {
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

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindUseViewer {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindViewer>,
}
impl BindUseViewer {
    pub fn new(bind: u32, data: Arc<ShaderBindViewer>) -> Self {
        Self { bind, data }
    }
}
