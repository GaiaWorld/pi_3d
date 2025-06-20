
use pi_render::renderer::{
        bind_buffer::{BindBufferAllocator, BindBufferRange},
        bind::TKeyBind,
        shader_stage::EShaderStage
};
use pi_scene_math::Matrix;
use crate::{binds::keybind_for_buffer, prelude::{BindDefines, TBindDefine}, shader::{ShaderSetBind, ShaderVarUniform}};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindModelAbout {
    pub(crate) data: BindBufferRange,
}
impl ShaderBindModelAbout {
    pub const OFFSET_WORLD_MATRIX:          wgpu::DynamicOffset = 0;
    pub const SIZE_WORLD_MATRIX:            wgpu::DynamicOffset = 16 * 4;
    pub const OFFSET_WORLD_MATRIX_INV:      wgpu::DynamicOffset = Self::SIZE_WORLD_MATRIX + Self::OFFSET_WORLD_MATRIX;
    pub const SIZE_WORLD_MATRIX_INV:        wgpu::DynamicOffset = 16 * 4;
    pub const OFFSET_VELOCITY:              wgpu::DynamicOffset = Self::SIZE_WORLD_MATRIX_INV + Self::OFFSET_WORLD_MATRIX_INV;
    pub const SIZE_VELOCITY:                wgpu::DynamicOffset = 4 * 4;
    pub const OFFSET_MORPHINFLUENCE:        wgpu::DynamicOffset = Self::SIZE_VELOCITY + Self::OFFSET_VELOCITY;
    pub const SIZE_MORPHINFLUENCE:          wgpu::DynamicOffset = 4 * 4;
    pub const OFFSET_MATIDX:                wgpu::DynamicOffset = Self::SIZE_MORPHINFLUENCE + Self::OFFSET_MORPHINFLUENCE;
    pub const SIZE_MATIDX:                  wgpu::DynamicOffset = 4 * 4;
    pub const OFFSET_SKINOFFSET:            wgpu::DynamicOffset = Self::SIZE_MATIDX + Self::OFFSET_MATIDX;
    pub const SIZE_SKINOFFSET:              wgpu::DynamicOffset = 4 * 4;
    pub const TOTAL_SIZE:                   wgpu::DynamicOffset = Self::SIZE_SKINOFFSET + Self::OFFSET_SKINOFFSET;
    pub fn new(
        allocator: &mut BindBufferAllocator,
    ) -> Option<Self> {
        if let Some(range) = allocator.allocate(ShaderBindModelAbout::TOTAL_SIZE) {
            let matrix = Matrix::identity();
            range.write_data(ShaderBindModelAbout::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(matrix.as_slice()));
            range.write_data(ShaderBindModelAbout::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(matrix.as_slice()));
            Some( Self { data: range, } )
        } else { None }
    }
    pub fn data(&self) -> &BindBufferRange { &self.data }
    pub fn update_matrix(&self, data: &[u8]) -> &Self {
        self.data.write_data(Self::OFFSET_WORLD_MATRIX as usize, data);
        self
    }
    pub fn update_matrix_inv(&self, data: &[u8]) -> &Self {
        self.data.write_data(Self::OFFSET_WORLD_MATRIX_INV as usize, data);
        self
    }
    pub fn update_velocity(&self, data: &[u8]) -> &Self {
        self.data.write_data(Self::OFFSET_VELOCITY as usize, data);
        self
    }
    pub fn update_morphinfluence(&self, data: &[u8]) -> &Self {
        self.data.write_data(Self::OFFSET_MORPHINFLUENCE as usize, data);
        self
    }
    pub fn update_matidxs(&self, passindex: usize, data: u32) -> &Self {
        let offset = passindex * 2;
        let data = data as u16;
        self.data.write_data(Self::OFFSET_MATIDX as usize + offset , bytemuck::cast_slice(&[data]));
        self
    }
    pub fn update_skinoffset(&self, data: &[u8]) -> &Self {
        self.data.write_data(Self::OFFSET_SKINOFFSET as usize, data);
        self
    }
    pub fn vs_define_code(&self, set: u32, binding: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, binding).as_str();
        result += "Model {\n";
        result += ShaderSetBind::code_uniform(crate::prelude::S_MAT4, ShaderVarUniform::_WORLD_MATRIX).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_MAT4, ShaderVarUniform::_WORLD_MATRIX_INV).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_VEC4, ShaderVarUniform::_VELOCITY).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_VEC4, ShaderVarUniform::MODEL_MORPHINFLUENCE).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_UVEC4, ShaderVarUniform::_MATIDX).as_str();
        result += ShaderSetBind::code_uniform(crate::prelude::S_UVEC4, ShaderVarUniform::_SKIN_BONE_OFFSET).as_str();
        result += "};\n";
        result
    }
    pub fn fs_define_code(&self, _: u32, _: u32) -> String {
        String::from("")
    }

}
impl TKeyBind for ShaderBindModelAbout {
    fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
        keybind_for_buffer(self.data.clone(), EShaderStage::VERTEXFRAGMENT, Self::TOTAL_SIZE as u32)
    }
}
impl TBindDefine for ShaderBindModelAbout {
    fn bind_include(&self) -> u32 {
        BindDefines::MODEL_BIND
    }
}
