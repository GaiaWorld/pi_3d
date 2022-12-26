use std::{ops::Range, sync::Arc};

use pi_engine_shell::object::ObjectID;
use render_data_container::{VertexBuffer, EVertexDataFormat};
use render_geometry::vertex_data::{VertexBufferDesc, VertexAttribute, EVertexDataKind};

use super::instanced_buffer::TInstancedBuffer;

pub struct InstancedBufferWorldMatrix {
    pub slot: crate::geometry::vertex_buffer_useinfo::EVertexBufferSlot, 
    pub key: render_data_container::KeyVertexBuffer,
    pub buffer: Arc<VertexBuffer>,
}
impl InstancedBufferWorldMatrix {
    pub fn new(index: usize, id: String) -> Self {
        let buffer: VertexBuffer = VertexBuffer::new(true, EVertexDataFormat::F32, false);
        let a = Arc::new(buffer);

        Self {
            slot: todo!(),
            key: render_data_container::KeyVertexBuffer::from(id + "WorldMat"),
        }
    }
}
impl TInstancedBuffer for InstancedBufferWorldMatrix {
    fn desc(&self) -> VertexBufferDesc {
        VertexBufferDesc {
            bufferkey: self.key(),
            range: None,
            attributes: vec![
                VertexAttribute { kind: EVertexDataKind::InsWorldRow1, format: wgpu::VertexFormat::Float32x4 },
                VertexAttribute { kind: EVertexDataKind::InsWorldRow2, format: wgpu::VertexFormat::Float32x4 },
                VertexAttribute { kind: EVertexDataKind::InsWorldRow3, format: wgpu::VertexFormat::Float32x4 },
                VertexAttribute { kind: EVertexDataKind::InsWorldRow4, format: wgpu::VertexFormat::Float32x4 },
            ],
            step_mode: wgpu::VertexStepMode::Instance,
        }
    }

    fn key(&self) -> render_data_container::KeyVertexBuffer {
        self.key.clone()
    }

    fn slot(&self) -> crate::geometry::vertex_buffer_useinfo::EVertexBufferSlot {
        self.slot
    }
}