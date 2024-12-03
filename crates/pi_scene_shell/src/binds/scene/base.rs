use std::sync::Arc;

use pi_render::{renderer::{
    bind::{KeyBindBuffer, KeyBindLayoutBuffer, TKeyBind}, bind_buffer::{BindBufferAllocator, BindBufferRange}, shader::TShaderBindCode, shader_stage::EShaderStage
}, rhi::device::RenderDevice};
use pi_world_macros::Resource;
use crate::{pass, prelude::{BindDefines, TBindDefine}, shader::*};


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
    pub fn data(&self) -> &BindBufferRange {
        &self.data
    }
}
impl TShaderBindCode for ShaderBindViewer {
    fn vs_define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += " Camera {";
        result += crate::prelude::S_BREAK;
        result += ShaderSetBind::code_uniform(crate::prelude::S_MAT4, ShaderVarUniform::VIEW_MATRIX).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_MAT4, ShaderVarUniform::PROJECT_MATRIX).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_MAT4, ShaderVarUniform::VIEW_PROJECT_MATRIX).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_VEC4, ShaderVarUniform::CAMERA_POSITION).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_VEC4, ShaderVarUniform::CAMERA_DIRECTION).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_MAT4, ShaderVarUniform::VIEW_ROTATION_MATRIX_INV).as_str();
        result += "};";
        result += crate::prelude::S_BREAK;
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
                        min_binding_size: Self::TOTAL_SIZE as u32,
                    }
                }
            )
        )
    }
}
impl TBindDefine for ShaderBindViewer {
    fn bind_include(&self) -> u32 {
        BindDefines::VIEWER
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindPassIndex {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindPassIndex {
    pub const SIZE: usize = 4 * 2;
    pub const MASK_0: u32 = 0b0000_0000_0000_0000_1111_1111_1111_1111;
    pub const MASK_1: u32 = 0b1111_1111_1111_1111_0000_0000_0000_0000;
    pub fn new(
        allocator: &mut BindBufferAllocator,
        passidx: u32,
    ) -> Option<Self> {
        if let Some(data) = allocator.allocate(Self::SIZE as wgpu::DynamicOffset) {
            data.write_data(0, bytemuck::cast_slice(&[passidx / 2, if passidx % 2 == 0 { Self::MASK_0 } else { Self::MASK_1 }]));
            Some(Self { data })
        } else {
            None
        }
    }
    pub fn data(&self) -> &BindBufferRange {
        &self.data
    }
}
impl TShaderBindCode for ShaderBindPassIndex {
    fn vs_define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += " IDX_PASS {";
        result += crate::prelude::S_BREAK;
        result += ShaderSetBind::code_uniform(&crate::prelude::S_UVEC2, ShaderVarUniform::IDX_PASS).as_str();
        result += "};";
        result += crate::prelude::S_BREAK;
        result
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        self.vs_define_code(set, bind)
    }
}
impl TKeyBind for ShaderBindPassIndex {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        Some(
            pi_render::renderer::bind::EKeyBind::Buffer(
                KeyBindBuffer {
                    data: self.data.clone(),
                    layout: KeyBindLayoutBuffer {
                        visibility: EShaderStage::VERTEXFRAGMENT,
                        min_binding_size: Self::SIZE as u32,
                    }
                }
            )
        )
    }
}
impl TBindDefine for ShaderBindPassIndex {
    fn bind_include(&self) -> u32 {
        BindDefines::PASS_INDEX
    }
}

#[derive(Resource)]
pub struct BindPassIndexPool {
    pub indexs: [ShaderBindPassIndex; 8],
}
impl BindPassIndexPool {
    pub fn new(allocator: &mut BindBufferAllocator) -> Self {
        Self {
            indexs: [
                ShaderBindPassIndex::new(allocator, 0).unwrap(),
                ShaderBindPassIndex::new(allocator, 1).unwrap(),
                ShaderBindPassIndex::new(allocator, 2).unwrap(),
                ShaderBindPassIndex::new(allocator, 3).unwrap(),
                ShaderBindPassIndex::new(allocator, 4).unwrap(),
                ShaderBindPassIndex::new(allocator, 5).unwrap(),
                ShaderBindPassIndex::new(allocator, 6).unwrap(),
                ShaderBindPassIndex::new(allocator, 7).unwrap(),
            ]
        }
    }
    pub fn get(& self, passindex: usize) -> Option<ShaderBindPassIndex> {
        if let Some(item) = self.indexs.get(passindex) {
            Some(item.clone())
        } else {
            None
        }
    }
}
