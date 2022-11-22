use pi_render::rhi::{bind_group_layout::BindGroupLayout, dyn_uniform_buffer::{BindOffset}, texture::{TextureView, Sampler}};
use render_geometry::vertex_data::VertexAttributeDesc;

use crate::{geometry::VDK, materials::MBKK, resources::RenderDynUniformBuffer};

pub struct BindInfo {
    pub set: u32,
    pub bind: u32,
    pub size: u32,
    pub layout: BindGroupLayout,
}

pub struct AttributeInfo {
    pub location: usize,
    pub kind: VertexAttributeDesc<VDK>,
}

pub struct Shader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,

}

pub struct BuildinShaderDefined;

impl BuildinShaderDefined {
    pub const VDK_START_FOR_OTHER: VDK      = 16 as VDK;
    pub const A_POSITION: VDK               = 01 as VDK;
    pub const A_POSITION_2D: VDK            = 02 as VDK;
    pub const A_COLOR: VDK                  = 03 as VDK;
    pub const A_NORMAL: VDK                 = 04 as VDK;
    pub const A_UV: VDK                     = 05 as VDK;
    pub const A_TANGENT: VDK                = 06 as VDK;
    pub const A_MATRICES_INDICES: VDK       = 07 as VDK;
    pub const A_MATRICES_WEIGHTS: VDK       = 08 as VDK;
    pub const A_MATRICES_INDICES_EXTRA: VDK = 09 as VDK;
    pub const A_MATRICES_WEIGHTS_EXTRA: VDK = 10 as VDK;
    
    pub const MBKK_START_FOR_OTHER: MBKK    = 32 as MBKK;
    pub const U_MATRIX_V: MBKK              = 01 as MBKK;
    pub const U_MATRIX_P: MBKK              = 02 as MBKK;
    pub const U_MATRIX_VP: MBKK             = 03 as MBKK;
    pub const U_TIME: MBKK                  = 04 as MBKK;
    pub const U_DELTA_TIME: MBKK            = 05 as MBKK;
    pub const U_SCREEN: MBKK                = 06 as MBKK;
    pub const U_FOG_PARAM: MBKK             = 07 as MBKK;
    pub const U_FOG_COLOR: MBKK             = 08 as MBKK;
    pub const U_BRDF: MBKK                  = 09 as MBKK;
    pub const U_OBJECT_TO_WORLD: MBKK       = 10 as MBKK;
    pub const U_WORLD_TO_OBJECT: MBKK       = 11 as MBKK;
}

pub trait FragmentUniformBind {
    const ID: u32;
    const SIZE: usize;

    const ENTRY: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: Self::ID as u32,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: true,
            min_binding_size: wgpu::BufferSize::new(Self::SIZE as wgpu::BufferAddress),
        },
        count: None,
    };

    fn entry<'a>(
        bind_offset: &BindOffset,
        buff: &'a RenderDynUniformBuffer,
    ) -> wgpu::BindGroupEntry<'a> {
        bind_group_entry_buffer(Self::ID as u32, buff.buffer().unwrap(), **bind_offset, Self::SIZE as u32)
    }

    fn dyn_entry<'a>(
        buff: &'a RenderDynUniformBuffer,
    ) -> wgpu::BindGroupEntry<'a> {
        bind_group_entry_buffer(Self::ID as u32, buff.buffer().unwrap(), 0, Self::SIZE as u32)
    }
}

pub trait FragmentUniformBindTexture {
    const TEXTURE_BIND: u8;
    const TEXTURE_SAMPLER_TYPE: wgpu::TextureSampleType;
    const DIM: wgpu::TextureViewDimension;
    const MULTI: bool;

    const ENTRY_TEXTURE: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: Self::TEXTURE_BIND as u32,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: Self::TEXTURE_SAMPLER_TYPE,
            view_dimension: Self::DIM,
            multisampled: Self::MULTI
        },
        count: None,
    };

    fn entry_texture<'a>(
        view: &'a wgpu::TextureView,
    ) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding: Self::TEXTURE_BIND as u32,
            resource: wgpu::BindingResource::TextureView(view),
        }
    }
}

pub trait FragmentUniformBindTextureSampler {
    const SAMPLER_BIND: u8;
    const SAMPLER_TYPE: wgpu::SamplerBindingType;

    const ENTRY_SAMPLER: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: Self::SAMPLER_BIND as u32,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(Self::SAMPLER_TYPE),
        count: None,
    };

    fn entry_sampler<'a>(
        value: &'a Sampler,
    ) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding: Self::SAMPLER_BIND as u32,
            resource: wgpu::BindingResource::Sampler(value),
        }
    }
}

pub fn bind_group_entry_buffer(
    id: u32,
    buffer: &wgpu::Buffer,
    offset: u32,
    size: u32,
) -> wgpu::BindGroupEntry {
    wgpu::BindGroupEntry {
        binding: id,
        resource: wgpu::BindingResource::Buffer(
            wgpu::BufferBinding {
                buffer,
                offset:  offset as wgpu::BufferAddress,
                size: wgpu::BufferSize::new(size as wgpu::BufferAddress),
            }
        ),
    }
}