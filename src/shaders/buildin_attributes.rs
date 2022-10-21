
use render_data_container::{EVertexDataFormat};

use render_geometry::geometry::{VertexAttributeMeta, VertexAttributeBufferMeta};

use crate::geometry::GBID;


pub struct BuildinAttributePosition {
    pub meta: VertexAttributeBufferMeta<GBID>,
}
impl BuildinAttributePosition {
    pub const POSITION: u32 = 3;
    pub const POSITION_OFFSET: u32 = 0 * 4;
    pub const POSITION_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;
    pub const POSITION_LOCATION: u32 = 0;

    pub const ATTRIBUTES: [wgpu::VertexAttribute;1] = [
        wgpu::VertexAttribute {
            format: Self::POSITION_FORMAT,
            offset: Self::POSITION_OFFSET as wgpu::BufferAddress,
            shader_location: Self::POSITION_LOCATION,
        }
    ];
}
impl VertexAttributeMeta for BuildinAttributePosition {
    const SLOT: u32 = 0;

    const SIZE_PER_VERTEX: u32 = Self::POSITION_OFFSET + Self::POSITION * 4;

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;

    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}

pub struct BuildinAttributeColor4 {
    pub meta: VertexAttributeBufferMeta<GBID>,
}
impl BuildinAttributeColor4 {
    pub const COLOR: u32 = 4;
    pub const COLOR_OFFSET: u32 = 0 * 4;
    pub const COLOR_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;
    pub const COLOR_LOCATION: u32 = 0;

    pub const ATTRIBUTES: [wgpu::VertexAttribute; 1] = [
        wgpu::VertexAttribute {
            format: Self::COLOR_FORMAT,
            offset: Self::COLOR_OFFSET as wgpu::BufferAddress,
            shader_location: Self::COLOR_LOCATION,
        }
    ];
}
impl VertexAttributeMeta for BuildinAttributeColor4 {
    const SLOT: u32 = 0;

    const SIZE_PER_VERTEX: u32 = Self::COLOR_OFFSET + Self::COLOR * 4;

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}


pub struct BuildinAttributeNormal {
    pub meta: VertexAttributeBufferMeta<GBID>,
}
impl BuildinAttributeNormal {
    pub const NORMAL: u32 = 3;
    pub const NORMAL_OFFSET: u32 = 0 * 4;
    pub const NORMAL_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;
    pub const NORMAL_LOCATION: u32 = 0;
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 1] = [
        wgpu::VertexAttribute {
            format: Self::NORMAL_FORMAT,
            offset: Self::NORMAL_OFFSET as wgpu::BufferAddress,
            shader_location: Self::NORMAL_LOCATION,
        }
    ];
}
impl VertexAttributeMeta for BuildinAttributeNormal {
    const SLOT: u32 = 0;
    const SIZE_PER_VERTEX: u32 = Self::NORMAL_OFFSET + Self::NORMAL * 4;
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}

pub struct BuildinAttributeIndices {
    pub meta: VertexAttributeBufferMeta<GBID>,
    pub format: wgpu::IndexFormat,
}