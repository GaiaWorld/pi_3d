use std::sync::Arc;

use derive_deref::Deref;
use pi_render::renderer::{
    texture::BindDataTexture2D, sampler::BindDataSampler, buildin_var::ShaderVarUniform, shader::TShaderBindCode,
    bind::{TKeyBind, KeyBindTexture2D, KeyBindLayoutTexture2D, KeyBindSampler, KeyBindLayoutSampler, KeyBindLayoutBuffer, KeyBindBuffer},
    shader_stage::EShaderStage, bind_buffer::{BindBufferAllocator, BindBufferRange}
};
use crate::shader::{texture_bind_code, sampler_bind_code, ShaderSetBind};

#[derive(Hash, PartialEq, Eq)]
pub struct ShaderBindShadowData{
    pub(crate) data: BindBufferRange,
    pub direct_count: u32,
    pub point_count: u32,
    pub spot_count: u32,
    pub shadow_count: u32,
    pub(crate) shadow_data_offset: u32,
    pub(crate) max_type_count: u32,
}
impl ShaderBindShadowData {
    pub const KEY: &'static str                 = "ShadowDatas";
    pub const SIZE_SHADOW_DATA: u32             = ((4 + 4 + 4 + 4) + 4 + 4 + 4) * 4;

    pub fn direct_shadow_data(
        &self, indexlight: u32, indexshadow: u32, matrix: &[f32], bias: f32, normalbias: f32, depthscale: f32, v: f32, uscale: f32, vscale: f32, uoff: f32, voff: f32
    ) {
        self.data.0.write_data( (indexlight as usize * 4 + 0) * 4, bytemuck::cast_slice(&[indexshadow]));
        let mut temp = [0.; 28];
        for i in 0..16 {
            temp[i] = matrix[i];
        }
        temp[16] = bias; temp[17] = normalbias; temp[18] = depthscale;
        temp[20] = depthscale; temp[21] = v;
        temp[24] = uscale; temp[25] = vscale; temp[26] = uoff; temp[27] = voff;
        self.data.0.write_data( (self.shadow_data_offset + indexshadow * Self::SIZE_SHADOW_DATA) as usize , bytemuck::cast_slice(&temp));
    }

    pub fn new(
        allocator: &mut BindBufferAllocator,
        direct_count: u32,
        point_count: u32,
        spot_count: u32,
        hemi_count: u32,
        shadow_count: u32,
    ) -> Option<Self> {
        let max_type_count = direct_count.max(point_count).max(spot_count).max(hemi_count);
        let shadow_data_offset = max_type_count * 4 * 4;
        let size = shadow_data_offset + shadow_count * Self::SIZE_SHADOW_DATA;
        if let Some(data) = allocator.allocate( size as wgpu::DynamicOffset ) {
            let mut temp = Vec::with_capacity(max_type_count as usize * 4);
            for _ in 0..max_type_count {
                temp.push(u32::MAX); temp.push(u32::MAX); temp.push(u32::MAX); temp.push(u32::MAX);
            }
            let mut tempf32 = Vec::with_capacity((shadow_count * Self::SIZE_SHADOW_DATA / 4) as usize);
            let matrix: [f32;16] = [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.];
            let bias_and_scale: [f32;4] = [0.001, 0.001, 1., 0.];
            let depth_scale: [f32;4] = [1., 1., 0., 0.];
            let tilloff: [f32;4] = [1., 1., 0., 0.];
            for _ in 0..shadow_count {
                matrix.iter().for_each(|v| { tempf32.push(*v) });
                bias_and_scale.iter().for_each(|v| { tempf32.push(*v) });
                depth_scale.iter().for_each(|v| { tempf32.push(*v) });
                tilloff.iter().for_each(|v| { tempf32.push(*v) });
            }
            data.0.write_data( 0, bytemuck::cast_slice(&temp));
            data.0.write_data( shadow_data_offset as usize, bytemuck::cast_slice(&tempf32));
            Some(Self { data, direct_count, point_count, spot_count, shadow_data_offset, shadow_count, max_type_count })
        } else {
            None
        }
    }

    pub fn reset(&self) {
        let mut temp = Vec::with_capacity(self.max_type_count as usize);
        for _ in 0..self.max_type_count {
            temp.push(u32::MAX); temp.push(u32::MAX); temp.push(u32::MAX); temp.push(u32::MAX);
        }
        self.data.0.write_data( 0, bytemuck::cast_slice(&temp));
    }
}
impl TShaderBindCode for ShaderBindShadowData {
    fn fs_define_code(&self, set: u32, bind: u32) -> String {
        let mut result = String::from("");
        result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
        result += " ";
        result += Self::KEY;
        result += " {\r\n";
        result += ShaderSetBind::code_uniform_array("uvec4", ShaderVarUniform::SHADOWMAP_LIGHT_INDEXS, self.max_type_count).as_str();
        result += ShaderSetBind::code_uniform_array("mat4", ShaderVarUniform::SHADOWMAP_MATRIX, self.shadow_count).as_str();
        result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::SHADOWMAP_BIAS_ANS_SCALE, self.shadow_count).as_str();
        result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::SHADOWMAP_DEPTH_VALUES, self.shadow_count).as_str();
        result += ShaderSetBind::code_uniform_array("vec4", ShaderVarUniform::SHADOWMAP_TILLOFF, self.shadow_count).as_str();
        result += "};\r\n";
        result += "const uint MAX_SHADOW = "; result += self.shadow_count.to_string().as_str(); result += ";\r\n";
        result
    }
}
impl TKeyBind for ShaderBindShadowData {
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
pub struct BindUseShadowData {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindShadowData>,
}
impl BindUseShadowData {
    pub fn new(bind: u32, data: Arc<ShaderBindShadowData>) -> Self {
        Self { bind, data }
    }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindShadowTexture(pub BindDataTexture2D);
impl TShaderBindCode for ShaderBindShadowTexture {
    fn fs_define_code(&self, set: u32, binding: u32) -> String {
        texture_bind_code(&wgpu::TextureSampleType::Float { filterable: true }, wgpu::TextureViewDimension::D2, ShaderVarUniform::SHADOWMAP_TEXTURE, set, binding)
    }
}
impl TKeyBind for ShaderBindShadowTexture {
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
pub struct BindUseShadowTexture {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindShadowTexture>,
}
impl BindUseShadowTexture {
    pub fn new(bind: u32, data: Arc<ShaderBindShadowTexture>) -> Self {
        Self { bind, data }
    }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq)]
pub struct ShaderBindShadowSampler(pub BindDataSampler);

impl TShaderBindCode for ShaderBindShadowSampler {
    fn fs_define_code(&self, set: u32, binding: u32) -> String {
        sampler_bind_code(ShaderVarUniform::SHADOWMAP_TEXTURE, wgpu::SamplerBindingType::Filtering, set, binding)
    }
}
impl TKeyBind for ShaderBindShadowSampler {
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
pub struct BindUseShadowSampler {
    pub(crate) bind: u32,
    pub(crate) data: Arc<ShaderBindShadowSampler>,
}
impl BindUseShadowSampler {
    pub fn new(bind: u32, data: Arc<ShaderBindShadowSampler>) -> Self {
        Self { bind, data }
    }
}
