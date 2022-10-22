use pi_idtree::Node;
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use render_data_container::{GeometryBuffer, EVertexDataFormat, GeometryBufferPool};
use render_geometry::geometry::VertexAttributeBufferMeta;

use crate::{shaders::buildin_attributes::{BuildinAttributePosition, BuildinAttributeNormal, BuildinAttributeIndices}, resources::SingleGeometryBufferPool, geometry::GBID};


pub struct CubeBuilder;

impl CubeBuilder {
    // pub const VERTEX_COUNT: u32 = 36;
    // pub const TRIANGLE_COUNT: u32 = 12;
    // pub const INDICES_COUNT: u32 = 36;
    // pub const FACE_COUNT: u32 = 6;
    // pub const INDICES_BUFFER_ID: Option<GBID> = None;
    // pub const NORMAL_BUFFER_ID: Option<GBID> = None;
    pub fn build(
        device: &RenderDevice,
        queue: &mut RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> (BuildinAttributePosition, BuildinAttributeNormal, BuildinAttributeIndices) {
        
        let data = [0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23];
        let mut indices = GeometryBuffer::new(false, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);
        let id_indices = gbp.insert(indices);

        let data = [0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0.];        let mut indices = GeometryBuffer::new(false, EVertexDataFormat::U16, true);
        let mut normals = GeometryBuffer::new(false, EVertexDataFormat::F32, false);
        normals.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        let id_normal = gbp.insert(normals);

        let mut position = GeometryBuffer::new(false, EVertexDataFormat::F32, false);
        let data = [1., -1., 1., -1., -1., 1., -1., 1., 1., 1., 1., 1., 1., 1., -1., -1., 1., -1., -1., -1., -1., 1., -1., -1., 1., 1., -1., 1., -1., -1., 1., -1., 1., 1., 1., 1., -1., 1., 1., -1., -1., 1., -1., -1., -1., -1., 1., -1., -1., 1., 1., -1., 1., -1., 1., 1., -1., 1., 1., 1., 1., -1., 1., 1., -1., -1., -1., -1., -1., -1., -1., 1.];
        position.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        let id_position = gbp.insert(position);

        (
            BuildinAttributePosition {
                meta: VertexAttributeBufferMeta {
                    buffer_id: id_position,
                    start: 0,
                    end: 72 * 4,
                    data_bytes_size: 3 * 4,
                    data_count: 24,
                },
            },
            BuildinAttributeNormal {
                meta: VertexAttributeBufferMeta {
                    buffer_id: id_normal,
                    start: 0,
                    end: 72 * 4,
                    data_bytes_size: 3 * 4,
                    data_count: 24,
                },
            },
            BuildinAttributeIndices {
                meta: VertexAttributeBufferMeta {
                    buffer_id: id_indices,
                    start: 0,
                    end: 36 * 2,
                    data_bytes_size: 1 * 2,
                    data_count: 36,
                },
                format: wgpu::IndexFormat::Uint16,
            },
        )
    }
}