
use derive_deref::Deref;
use pi_render::renderer::{
    texture::BindDataTexture2D, sampler::BindDataSampler, shader::TShaderBindCode,
    bind::{TKeyBind, KeyBindTexture2D, KeyBindLayoutTexture2D, KeyBindSampler, KeyBindLayoutSampler, KeyBindLayoutBuffer, KeyBindBuffer},
    shader_stage::EShaderStage, bind_buffer::{BindBufferAllocator, BindBufferRange}
};
use crate::{prelude::{BindDefines, TBindDefine}, shader::{sampler_bind_code, texture_bind_code, ShaderSetBind, ShaderVarUniform}};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindShadowData{
    pub(crate) data: BindBufferRange,
    pub shadow_count: u16,
    pub(crate) shadow_data_offset: u32,
    pub(crate) max_type_count: u32,
    pub(crate) totalsize: u32,
}
impl ShaderBindShadowData {
    pub const KEY: &'static str                 = "ShadowDatas";
    pub const SIZE_SHADOW_DATA: u32             = ((4 + 4 + 4 + 4) + 4 + 4 + 4) * 4;

    pub fn direct_shadow_data(
        &self, indexlight: u32, indexshadow: u32, matrix: &[f32], bias: f32, normalbias: f32, minz: f32, maxz: f32, uscale: f32, vscale: f32, uoff: f32, voff: f32
    ) {
        self.data.0.write_data( (indexlight as usize * 4 + 0) * 4, bytemuck::cast_slice(&[indexshadow]));
        let mut temp = [0.; 28];
        for i in 0..16 {
            temp[i] = matrix[i];
        }
        temp[16] = bias; temp[17] = normalbias; temp[18] = minz; temp[19] = maxz;
        temp[20] = minz; temp[21] = maxz;
        temp[24] = uscale; temp[25] = vscale; temp[26] = uoff; temp[27] = voff;
        self.data.0.write_data( (self.shadow_data_offset + indexshadow * Self::SIZE_SHADOW_DATA) as usize , bytemuck::cast_slice(&temp));
    }

    pub fn new(
        allocator: &mut BindBufferAllocator,
        direct_count: u16,
        point_count: u16,
        spot_count: u16,
        hemi_count: u16,
        shadow_count: u16,
    ) -> Option<Self> {
        let max_type_count = direct_count.max(point_count).max(spot_count).max(hemi_count) as u32;
        let shadow_data_offset = max_type_count * 4 * 4 as u32;
        let size = shadow_data_offset + shadow_count as u32 * Self::SIZE_SHADOW_DATA;
        if let Some(data) = allocator.allocate( size as wgpu::DynamicOffset ) {
            let mut temp = Vec::with_capacity(max_type_count as usize * 4);
            for _ in 0..max_type_count {
                temp.push(u32::MAX); temp.push(u32::MAX); temp.push(u32::MAX); temp.push(u32::MAX);
            }
            let mut tempf32 = Vec::with_capacity((shadow_count as u32 * Self::SIZE_SHADOW_DATA / 4) as usize);
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
            Some(Self { data, shadow_data_offset, shadow_count: shadow_count as u16, max_type_count, totalsize: size })
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
        result += crate::prelude::S_SPACE;
        result += Self::KEY;
        result += crate::prelude::S_SPACE;
        result += "{";
        result += crate::prelude::S_BREAK;
        result += ShaderSetBind::code_uniform_array(crate::prelude::S_UVEC4, ShaderVarUniform::SHADOWMAP_LIGHT_INDEXS, self.max_type_count).as_str();
        result += ShaderSetBind::code_uniform_array(crate::prelude::S_MAT4, ShaderVarUniform::SHADOWMAP_MATRIX, self.shadow_count as u32).as_str();
        result += ShaderSetBind::code_uniform_array(crate::prelude::S_VEC4, ShaderVarUniform::SHADOWMAP_BIAS_ANS_SCALE, self.shadow_count as u32).as_str();
        result += ShaderSetBind::code_uniform_array(crate::prelude::S_VEC4, ShaderVarUniform::SHADOWMAP_DEPTH_VALUES, self.shadow_count as u32).as_str();
        result += ShaderSetBind::code_uniform_array(crate::prelude::S_VEC4, ShaderVarUniform::SHADOWMAP_TILLOFF, self.shadow_count as u32).as_str();
        result += "};";
        result += crate::prelude::S_BREAK;
        result += "const uint MAX_SHADOW = "; result += self.shadow_count.to_string().as_str(); result += ";";
        result += crate::prelude::S_BREAK;
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
                        dynamic: self.data.2,
                        min_binding_size: self.totalsize as u32,
                    }
                }
            )
        )
    }
}
impl TBindDefine for ShaderBindShadowData {
    fn bind_include(&self) -> u32 {
        BindDefines::SHADOWMAP
    }
}


#[derive(Clone, Deref, Hash, PartialEq, Eq)]
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
impl TBindDefine for ShaderBindShadowTexture {
    fn bind_include(&self) -> u32 {
        BindDefines::SHADOWMAP
    }
}


#[derive(Clone, Deref, Hash, PartialEq, Eq)]
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
impl TBindDefine for ShaderBindShadowSampler {
    fn bind_include(&self) -> u32 {
        BindDefines::SHADOWMAP
    }
}

