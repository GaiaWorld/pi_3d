use std::sync::Arc;

use pi_render::renderer::{
    bind_buffer::{BindBufferAllocator, BindBufferRange},
    bind::{KeyBindLayoutBuffer, KeyBindBuffer, TKeyBind},
    shader_stage::EShaderStage,
    buildin_var::ShaderVarUniform,
    shader::TShaderBindCode
};
use crate::shader::ShaderSetBind;


////////////////////////////////// Lighting LightIndex
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BindModelLightIndexs {
    pub(crate) data: BindBufferRange,
    pub direct_count: u32,
    pub point_count: u32,
    pub spot_count: u32,
    pub hemi_count: u32,
    pub(crate) meta_offset: u32,
    pub(crate) max_type_count: u32,
}
impl BindModelLightIndexs {
    pub const KEY: &'static str = "ModelLightings";

    pub fn direct_light_data(&self, lightidxs: &[u32]) {
        if self.direct_count == 0 { return; }
        let data = if lightidxs.len() <= self.direct_count as usize { lightidxs } else { &lightidxs[0..self.direct_count as usize] };
        // if 0 < data.len() {
        //     self.data.0.write_data( (lightidxs * 4 + 0)as usize * 4 + 4 * 4, bytemuck::cast_slice(&data));
        // }
        let mut idx = 0;
        data.iter().for_each(|data| {
            self.data.0.write_data( (idx * 4 + 0)as usize * 4 + 4 * 4, bytemuck::cast_slice(&[*data])); idx += 1;
        });
        // log::warn!("Model Direct: {:?}", lightidxs);
        self.data.0.write_data( self.meta_offset as usize + 0 * 4, bytemuck::cast_slice(&[data.len() as u32]));
    }
    pub fn point_light_data(&self, lightidxs: &[u32]) {
        if self.point_count == 0 { return; }
        let data = if lightidxs.len() <= self.point_count as usize { lightidxs } else { &lightidxs[0..self.point_count as usize] };
        // if 0 < data.len() {
        //     self.data.0.write_data( (lightidxs * 4 + 1)as usize * 4 + 4 * 4, bytemuck::cast_slice(&data));
        // }
        let mut idx = 0;
        data.iter().for_each(|data| {
            // log::warn!("Model Point: {:?} {:?}", idx, (idx * 4 + 1)as usize * 4 + 4 * 4);
            self.data.0.write_data( (idx * 4 + 1)as usize * 4 + 4 * 4, bytemuck::cast_slice(&[*data])); idx += 1;
        });
        // log::warn!("Model Point: {:?}", lightidxs);
        self.data.0.write_data( self.meta_offset as usize + 1 * 4, bytemuck::cast_slice(&[data.len() as u32]));
    }
    pub fn spot_light_data(&self, lightidxs: &[u32]) {
        if self.spot_count == 0 { return; }
        let data = if lightidxs.len() <= self.spot_count as usize { lightidxs } else { &lightidxs[0..self.spot_count as usize] };
        // if 0 < data.len() {
        //     self.data.0.write_data( (lightidxs * 4 + 2) as usize * 4 + 4 * 4, bytemuck::cast_slice(&data));
        // }
        let mut idx = 0;
        data.iter().for_each(|data| {
            self.data.0.write_data( (idx * 4 + 2)as usize * 4 + 4 * 4, bytemuck::cast_slice(&[*data])); idx += 1;
        });
        // log::warn!("Model Spot: {:?}", lightidxs);
        self.data.0.write_data( self.meta_offset as usize + 2 * 4, bytemuck::cast_slice(&[data.len() as u32]));
    }
    pub fn hemi_light_data(&self, lightidxs: &[u32]) {
        if self.hemi_count == 0 { return; }
        let data = if lightidxs.len() <= self.hemi_count as usize { lightidxs } else { &lightidxs[0..self.hemi_count as usize] };
        // if 0 < data.len() {
        //     self.data.0.write_data( (lightidxs * 4 + 3)as usize * 4 + 4 * 4, bytemuck::cast_slice(&data));
        // }
        let mut idx = 0;
        data.iter().for_each(|data| {
            self.data.0.write_data( (idx * 4 + 3)as usize * 4 + 4 * 4, bytemuck::cast_slice(&[*data])); idx += 1;
        });
        // log::warn!("Model Hemi: {:?}", lightidxs);
        self.data.0.write_data( self.meta_offset as usize + 3 * 4, bytemuck::cast_slice(&[data.len() as u32]));
    }

    pub fn new(
        allocator: &mut BindBufferAllocator,
        direct_count: u32,
        point_count: u32,
        spot_count: u32,
        hemi_count: u32,
    ) -> Option<Self> {
        let meta_offset = 0;

        let max_type_count = direct_count.max(point_count).max(spot_count).max(hemi_count);
        let size = 4 * 4 + max_type_count * 4 * 4;

        // log::warn!("IndexSize: {:?}", size);
        if let Some(data) = allocator.allocate( size as wgpu::DynamicOffset ) {
            let mut temp = Vec::with_capacity(size as usize);
            for _ in 0..size {
                temp.push(0);
            }
            data.0.write_data( 0, &temp);
            Some(Self { data, direct_count, point_count, spot_count, hemi_count, meta_offset, max_type_count })
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

impl TShaderBindCode for BindModelLightIndexs {
    fn vs_define_code(&self, set: u32, bind: u32) -> String {
        self.fs_define_code(set, bind)
    }
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += " ";
        result += Self::KEY;
        result += " {\r\n";
        // result += ShaderSetBind::code_uniform("uint", ShaderVarUniform::MODEL_DIRECTLIGHT_COUNT).as_str();
        // result += ShaderSetBind::code_uniform("uint", ShaderVarUniform::MODEL_POINTLIGHT_COUNT).as_str();
        // result += ShaderSetBind::code_uniform("uint", ShaderVarUniform::MODEL_SPOTLIGHT_COUNT).as_str();
        // result += ShaderSetBind::code_uniform("uint", ShaderVarUniform::MODEL_HEMILIGHT_COUNT).as_str();
        // result += ShaderSetBind::code_uniform_array("uint", ShaderVarUniform::MODEL_LIGHTS_COUNT, 4).as_str();
        result += ShaderSetBind::code_uniform("uvec4", ShaderVarUniform::MODEL_LIGHTS_COUNT).as_str();
        if 0 < self.max_type_count {
        result += ShaderSetBind::code_uniform_array("uvec4", ShaderVarUniform::MODEL_LIGHTS_INDEXS, self.max_type_count).as_str();
        }
        // if 0 < self.direct_count {
        // result += ShaderSetBind::code_uniform_array("uint", ShaderVarUniform::DIRECT_LIGHT_INDEXS, self.direct_count).as_str();
        // }
        // if 0 < self.point_count {
        // result += ShaderSetBind::code_uniform_array("uint", ShaderVarUniform::POINT_LIGHT_INDEXS, self.point_count).as_str();
        // }
        // if 0 < self.spot_count {
        // result += ShaderSetBind::code_uniform_array("uint", ShaderVarUniform::SPOT_LIGHT_INDEXS, self.spot_count).as_str();
        // }
        // if 0 < self.hemi_count {
        // result += ShaderSetBind::code_uniform_array("uint", ShaderVarUniform::HEMI_LIGHT_INDEXS, self.hemi_count).as_str();
        // }
        result += "};\r\n";
        result
    }
}
impl TKeyBind for BindModelLightIndexs {
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
pub struct BindUseModelLightIndexs {
    pub(crate) bind: u32,
    pub(crate) data: Arc<BindModelLightIndexs>,
}
impl BindUseModelLightIndexs {
    pub fn new(bind: u32, data: Arc<BindModelLightIndexs>) -> Self {
        Self { bind, data }
    }
}
