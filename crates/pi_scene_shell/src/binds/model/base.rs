
use pi_render::renderer::{
        bind_buffer::{BindBufferAllocator, BindBufferRange},
        shader::TShaderBindCode,
        bind::{TKeyBind, KeyBindLayoutBuffer, KeyBindBuffer},
        shader_stage::EShaderStage
};
use pi_scene_math::Matrix;
use crate::{binds::keybind_for_buffer, prelude::{BindDefines, TBindDefine}, shader::{ShaderSetBind, ShaderVarUniform}};


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindModelAboutMatrix {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindModelAboutMatrix {
    pub const OFFSET_WORLD_MATRIX:          wgpu::DynamicOffset = 0;
    pub const SIZE_WORLD_MATRIX:            wgpu::DynamicOffset = 16 * 4;
    pub const TOTAL_SIZE:                   wgpu::DynamicOffset = Self::OFFSET_WORLD_MATRIX + Self::SIZE_WORLD_MATRIX;
    pub fn new(
        allocator: &mut BindBufferAllocator,
    ) -> Option<Self> {
        if let Some(range) = allocator.allocate(ShaderBindModelAboutMatrix::TOTAL_SIZE) {
            let matrix = Matrix::identity();
            range.write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(matrix.as_slice()));
            Some( Self { data: range, } )
        } else { None }
    }
    pub fn data(&self) -> &BindBufferRange { &self.data }
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, binding).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_MAT4, ShaderVarUniform::_WORLD_MATRIX).as_str();
        result
    }
    pub fn fs_define_code(&self, _: u32, _: u32) -> String {
        String::from("")
    }

}
impl TKeyBind for ShaderBindModelAboutMatrix {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        keybind_for_buffer(self.data.clone(), EShaderStage::VERTEXFRAGMENT, Self::TOTAL_SIZE as u32)
    }
}
impl TBindDefine for ShaderBindModelAboutMatrix {
    fn bind_include(&self) -> u32 {
        BindDefines::MODEL_BIND
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindModelMatrixInv {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindModelMatrixInv {
    pub const OFFSET_WORLD_MATRIX_INV:      wgpu::DynamicOffset = 0;
    pub const SIZE_WORLD_MATRIX_INV:        wgpu::DynamicOffset = 16 * 4;
    pub const TOTAL_SIZE:                   wgpu::DynamicOffset = Self::OFFSET_WORLD_MATRIX_INV + Self::SIZE_WORLD_MATRIX_INV;
    pub fn new( allocator: &mut BindBufferAllocator, ) -> Option<Self> {
        if let Some(range) = allocator.allocate(ShaderBindModelMatrixInv::TOTAL_SIZE) {
            let matrix = Matrix::identity();
            range.write_data(ShaderBindModelMatrixInv::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(matrix.as_slice()));
            Some( Self { data: range, } )
        } else { None }
    }
    pub fn data(&self) -> &BindBufferRange { &self.data }
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, binding).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_MAT4, ShaderVarUniform::_WORLD_MATRIX_INV).as_str();
        result
    }
    pub fn fs_define_code(&self, _: u32, _: u32) -> String {
        String::from("")
    }

}
impl TKeyBind for ShaderBindModelMatrixInv {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        keybind_for_buffer(self.data.clone(), EShaderStage::VERTEXFRAGMENT, Self::TOTAL_SIZE as u32)
    }
}
impl TBindDefine for ShaderBindModelMatrixInv {
    fn bind_include(&self) -> u32 {
        BindDefines::MODEL_MATRIX_INV
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindModelVelocity {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindModelVelocity {
    pub const OFFSET_VELOCITY:              wgpu::DynamicOffset = 0;
    pub const SIZE_VELOCITY:                wgpu::DynamicOffset = 4 * 4;
    pub const TOTAL_SIZE:                   wgpu::DynamicOffset = Self::OFFSET_VELOCITY + Self::SIZE_VELOCITY;
    pub fn new( allocator: &mut BindBufferAllocator, ) -> Option<Self> {
        if let Some(range) = allocator.allocate(ShaderBindModelVelocity::TOTAL_SIZE) {
            range.write_data(ShaderBindModelVelocity::OFFSET_VELOCITY as usize, bytemuck::cast_slice(&[0f32, 0f32, 0f32, 0f32]));
            Some( Self { data: range, } )
        } else { None }
    }
    pub fn data(&self) -> &BindBufferRange { &self.data }
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, binding).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_VEC4, ShaderVarUniform::_VELOCITY).as_str();
        result
    }
    pub fn fs_define_code(&self, _: u32, _: u32) -> String {
        String::from("")
    }
}
impl TKeyBind for ShaderBindModelVelocity {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        keybind_for_buffer(self.data.clone(), EShaderStage::VERTEXFRAGMENT, Self::TOTAL_SIZE as u32)
    }
}
impl TBindDefine for ShaderBindModelVelocity {
    fn bind_include(&self) -> u32 {
        BindDefines::MODEL_VELOCITY
    }
}


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindModelMorphinfluence {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindModelMorphinfluence {
    pub const TOTAL_SIZE:                   wgpu::DynamicOffset = 4 * 4;
    pub fn new( allocator: &mut BindBufferAllocator, ) -> Option<Self> {
        if let Some(range) = allocator.allocate(ShaderBindModelVelocity::TOTAL_SIZE) {
            range.write_data(0, bytemuck::cast_slice(&[0f32, 0f32, 0f32, 0f32]));
            Some( Self { data: range, } )
        } else { None }
    }
    pub fn data(&self) -> &BindBufferRange { &self.data }
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, binding).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_VEC4, ShaderVarUniform::MODEL_MORPHINFLUENCE).as_str();
        result
    }
    pub fn fs_define_code(&self, _: u32, _: u32) -> String {
        String::from("")
    }
}
impl TKeyBind for ShaderBindModelMorphinfluence {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        keybind_for_buffer(self.data.clone(), EShaderStage::VERTEXFRAGMENT, Self::TOTAL_SIZE as u32)
    }
}
impl TBindDefine for ShaderBindModelMorphinfluence {
    fn bind_include(&self) -> u32 {
        BindDefines::MODEL_MORPHINFLUENCE
    }
}


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindModelSkinOffset {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindModelSkinOffset {
    pub const TOTAL_SIZE: wgpu::DynamicOffset = 4 * 4;
    pub fn new(allocator: &mut BindBufferAllocator) -> Option<Self> {
        if let Some(range) = allocator.allocate(Self::TOTAL_SIZE) {
            range.write_data(0, bytemuck::cast_slice(&[0u32, 0u32, 0u32, 0u32]));
            Some( Self { data: range, } )
        } else { None }
    }
    pub fn update_matidxs(&self, passindex: usize, data: u32) {
        let offset = passindex * 2;
        let data = data as u16;
        self.data.write_data(offset, bytemuck::cast_slice(&[data]));
    }
    pub fn data(&self) -> &BindBufferRange { &self.data }
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, binding).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_UVEC4, ShaderVarUniform::_SKIN_BONE_OFFSET).as_str();
        result
    }
    pub fn fs_define_code(&self, _: u32, _: u32) -> String {
        String::from("")
    }
}
impl TKeyBind for ShaderBindModelSkinOffset {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        keybind_for_buffer(self.data.clone(), EShaderStage::VERTEXFRAGMENT, Self::TOTAL_SIZE as u32)
    }
}
impl TBindDefine for ShaderBindModelSkinOffset {
    fn bind_include(&self) -> u32 {
        BindDefines::MODEL_SKIN_INS
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindModelMatIdx {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindModelMatIdx {
    pub const TOTAL_SIZE: wgpu::DynamicOffset = 4 * 4;
    pub fn new(allocator: &mut BindBufferAllocator) -> Option<Self> {
        if let Some(range) = allocator.allocate(Self::TOTAL_SIZE) {
            range.write_data(0, bytemuck::cast_slice(&[0u32, 0u32, 0u32, 0u32]));
            Some( Self { data: range, } )
        } else { None }
    }
    pub fn update_matidxs(&self, passindex: usize, data: u32) {
        let offset = passindex * 2;
        let data = data as u16;
        self.data.write_data(offset, bytemuck::cast_slice(&[data]));
    }
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, binding).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_UVEC4, ShaderVarUniform::_MATIDX).as_str();
        result
    }

    pub fn fs_define_code(&self, _: u32, _: u32) -> String {
        String::from("")
    }
}
impl TKeyBind for ShaderBindModelMatIdx {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        keybind_for_buffer(self.data.clone(), EShaderStage::VERTEXFRAGMENT, Self::TOTAL_SIZE as u32)
    }
}
impl TBindDefine for ShaderBindModelMatIdx {
    fn bind_include(&self) -> u32 {
        BindDefines::MAT_INDEX
    }
}