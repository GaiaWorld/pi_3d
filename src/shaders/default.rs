use ncollide3d::na::U32;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, device::RenderDevice};
use render_data_container::EVertexDataFormat;
use render_geometry::geometry::GeometryBufferDesc;
use render_material::{binding::BindingDesc, material::{UniformDesc, EUniformDataFormat}};

use crate::{geometry::VDK, materials::MBKK};

use super::BuildinShaderDefined;

pub struct DefaultShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl DefaultShader {
    pub const A_POSITION: MBKK                          = BuildinShaderDefined::A_POSITION;
    pub const A_POSITION_SLOT: u32                      = 0;
    pub const A_POSITION_SIZE: u32                      = 3 * 4;
    pub const U_EMISSIVE: MBKK                          = BuildinShaderDefined::MBKK_START_FOR_OTHER + 01 as MBKK;
    pub const BIND_DESC: Vec<BindingDesc<MBKK>> = vec![
        BindingDesc {
            uniforms: vec![
                UniformDesc {
                    kind: Self::U_EMISSIVE,
                    bind: 1,
                    format: EUniformDataFormat::Color4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    byte_offset_in_bind: 0,
                }
            ],
            size: 4 * 4,
            id: 1
        }
    ];
    pub const ATTRIBUTES_DESC: Vec<GeometryBufferDesc<VDK>> = vec![
        GeometryBufferDesc {
            slot: Self::A_POSITION_SLOT,
            format: EVertexDataFormat::F32,
            kind: Self::A_POSITION,
            size_per_vertex: Self::A_POSITION_SIZE as usize,
        }
    ];
}