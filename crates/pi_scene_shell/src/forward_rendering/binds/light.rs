use std::sync::Arc;

use pi_render::renderer::{
    bind_buffer::{BindBufferRange, BindBufferAllocator},
    bind::{KeyBindLayoutBuffer, KeyBindBuffer, TKeyBind},
    shader_stage::EShaderStage,
    buildin_var::ShaderVarUniform, shader::TShaderBindCode
};
use crate::shader::ShaderSetBind;

#[derive(Hash, PartialEq, Eq)]
pub struct ShaderBindSceneLightInfos {
    pub(crate) data: BindBufferRange,
    pub direct_count: u32,
    pub point_count: u32,
    pub spot_count: u32,
    pub hemi_count: u32,
    pub(crate) direct_offset: u32,
    pub(crate) point_offset: u32,
    pub(crate) spot_offset: u32,
    pub(crate) hemi_offset: u32,
}
impl ShaderBindSceneLightInfos {
    pub const MIN_TYPE_LIGHT_COUNT: u32                     = 4;
    pub const SIZE_DIRECT_LIGHT: u32                        = (4 + 4) * 4;
    pub const SIZE_POINT_LIGHT: u32                         = (4 + 4 + 4) * 4;
    pub const SIZE_SPOT_LIGHT: u32                          = (4 + 4 + 4 + 4) * 4;
    pub const SIZE_HEMI_LIGHT: u32                          = (4 + 4 + 4 + 4) * 4;
    pub const SIZE_LIGHT_DATA_A: wgpu::DynamicOffset        = 4 * 4;
    pub const SIZE_LIGHT_DATA_B: wgpu::DynamicOffset        = 4 * 4;
    pub const SIZE_LIGHT_DATA_C: wgpu::DynamicOffset        = 4 * 4;
    pub const SIZE_LIGHT_DATA_D: wgpu::DynamicOffset        = 4 * 4;
    pub const TOTAL_SIZE: wgpu::DynamicOffset               = Self::SIZE_LIGHT_DATA_A + Self::SIZE_LIGHT_DATA_B + Self::SIZE_LIGHT_DATA_C + Self::SIZE_LIGHT_DATA_D;

    pub fn direct_light_data(&self, indexlight: u32, enabled: bool, layer: f32, dx: f32, dy: f32, dz: f32, r: f32, g: f32, b: f32) {
        if indexlight < self.direct_count {
            let enabled = if enabled { 1.0 } else { -1.0 };
            let data = [dx, dy, dz, enabled];
            // log::warn!("Direct Light: {:?} {:?}", indexlight, data);
            self.data.0.write_data( self.direct_offset as usize + (0 * self.direct_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [r, g, b, layer];
            self.data.0.write_data( self.direct_offset as usize + (1 * self.direct_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
        }
    }
    pub fn point_light_data(&self, indexlight: u32, enabled: bool, layer: f32, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, range: f32, inv_quare_range: f32) {
        if indexlight < self.point_count {
            let enabled = if enabled { 1.0 } else { -1.0 };
            let data = [x, y, z, enabled];
            // log::warn!("Point Light: {:?}", data);
            self.data.0.write_data(self.point_offset  as usize + (0 * self.point_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [r, g, b, layer];
            self.data.0.write_data(self.point_offset  as usize + (1 * self.point_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [0., 0., range, inv_quare_range];
            self.data.0.write_data(self.point_offset  as usize + (2 * self.point_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
        }
    }
    pub fn spot_light_data(&self, indexlight: u32, enabled: bool, layer: f32, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, range: f32, inv_quare_range: f32, inanlge: f32, outangle: f32, dx: f32, dy: f32, dz: f32) {
        if indexlight < self.spot_count {
            let enabled = if enabled { 1.0 } else { -1.0 };
            let data = [x, y, z, enabled];
            // log::warn!("Spot Light: {:?}", data);
            self.data.0.write_data(self.spot_offset as usize + (0 * self.spot_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [r, g, b, layer];
            self.data.0.write_data(self.spot_offset as usize + (1 * self.spot_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [inanlge, outangle, range, inv_quare_range];
            self.data.0.write_data(self.spot_offset as usize + (2 * self.spot_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [dx, dy, dz, 0.];
            self.data.0.write_data(self.spot_offset as usize + (3 * self.spot_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
        }
    }
    pub fn hemi_light_data(&self, indexlight: u32, enabled: bool, layer: f32, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, range: f32, inv_quare_range: f32, p0: f32, p1: f32, p2: f32, p3: f32) {
        if indexlight < self.hemi_count {
            let enabled = if enabled { 1.0 } else { -1.0 };
            let data = [x, y, z, enabled];
            self.data.0.write_data(self.hemi_offset as usize + (0 * self.hemi_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [r, g, b, layer];
            self.data.0.write_data(self.hemi_offset as usize + (1 * self.hemi_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [0., 0., range, inv_quare_range];
            self.data.0.write_data(self.hemi_offset as usize + (2 * self.hemi_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
            let data = [p0, p1, p2, p3];
            self.data.0.write_data(self.hemi_offset as usize + (3 * self.hemi_count + indexlight) as usize * 4 * 4, bytemuck::cast_slice(&data));
        }
    }

    pub fn new(
        allocator: &mut BindBufferAllocator,
        mut direct_count: u32,
        mut point_count: u32,
        mut spot_count: u32,
        mut hemi_count: u32,
    ) -> Option<Self> {
        direct_count = Self::MIN_TYPE_LIGHT_COUNT.max(direct_count);
        point_count = point_count;
        spot_count = spot_count;
        hemi_count = hemi_count;
        
        let direct_offset = 0;
        let point_offset = direct_offset + direct_count * Self::SIZE_DIRECT_LIGHT;
        let spot_offset = point_offset + point_count * Self::SIZE_POINT_LIGHT;
        let hemi_offset = spot_offset + spot_count * Self::SIZE_SPOT_LIGHT;

        let size = hemi_offset + hemi_count * Self::SIZE_HEMI_LIGHT;

        if let Some(data) = allocator.allocate( size as wgpu::DynamicOffset) {
            Some(Self { data, direct_count, point_count, spot_count, hemi_count, direct_offset, point_offset, spot_offset, hemi_offset })
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
impl TShaderBindCode for ShaderBindSceneLightInfos {
    fn vs_define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += " ";
        result += ShaderVarUniform::LIGHTING_INFOS;
        result += " {\r\n";

        result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::DIRECT_LIGHT_DIRECTION, self.direct_count).as_str();
        result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::DIRECT_LIGHT_COLOR, self.direct_count).as_str();

        if 0 < self.point_count {
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::POINT_LIGHT_POSITION, self.point_count).as_str();
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::POINT_LIGHT_COLOR, self.point_count).as_str();
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::POINT_LIGHT_DATA, self.point_count).as_str();
        }

        if 0 < self.spot_count {
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::SPOT_LIGHT_POSITION, self.spot_count).as_str();
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::SPOT_LIGHT_COLOR, self.spot_count).as_str();
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::SPOT_LIGHT_DATA, self.spot_count).as_str();
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::SPOT_LIGHT_DIRECTION, self.spot_count).as_str();
        }

        if 0 < self.hemi_count {
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::HEMI_LIGHT_POSITION, self.hemi_count).as_str();
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::HEMI_LIGHT_COLOR, self.hemi_count).as_str();
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::HEMI_LIGHT_DATA, self.hemi_count).as_str();
            result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::HEMI_LIGHT_DIRECTION, self.hemi_count).as_str();
        }

        // result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::HEMI_LIGHT_POSITION, self.spot_count).as_str();
        // result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::HEMI_LIGHT_COLOR, self.spot_count).as_str();
        // result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::HEMI_LIGHT_DATA, self.spot_count).as_str();
        // result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::HEMI_LIGHT_DIRECTION, self.spot_count).as_str();

        result += "};\r\n";
        result += "const uint MAX_DIRECT_LIGHT = "; result += self.direct_count.to_string().as_str(); result += ";\r\n";
        result += "const uint MAX_POINT_LIGHT = "; result += self.point_count.to_string().as_str(); result += ";\r\n";
        result += "const uint MAX_SPOT_LIGHT = "; result += self.spot_count.to_string().as_str(); result += ";\r\n";
        result += "const uint MAX_HEMI_LIGHT = "; result += self.hemi_count.to_string().as_str(); result += ";\r\n";
        result
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        self.vs_define_code(set, bind)
    }
}
impl TKeyBind for ShaderBindSceneLightInfos {
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
pub struct BindUseSceneLightInfos {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindSceneLightInfos>,
}
impl BindUseSceneLightInfos {
    pub fn new(bind: u32, data: Arc<ShaderBindSceneLightInfos>) -> Self {
        Self { bind, data }
    }
}
