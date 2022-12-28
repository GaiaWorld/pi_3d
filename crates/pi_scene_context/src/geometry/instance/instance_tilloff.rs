use pi_scene_math::Vector4;
use render_data_container::{VertexBufferPool, EVertexDataFormat, VertexBuffer};

use crate::geometry::vertex_buffer_useinfo;

use super::{instanced_buffer::TInstancedBuffer, types::TInstancedData};

pub struct InstanceTillOff(pub Vector4);
impl TInstancedData for InstanceTillOff {
    fn vertex_kind(&self) -> render_geometry::vertex_data::EVertexDataKind {
        todo!()
    }

    fn value(&self) -> &super::types::InstancedValue {
        todo!()
    }

    fn size() -> usize {
        4
    }
    fn bytes_size() -> usize {
        4 * 4
    }

    fn local_offset(&self) -> usize {
        0
    }

    fn write_instance_buffer(&self, buffer: &mut VertexBuffer, offset: usize) {
        buffer.update_f32(self.0.as_slice(), offset);
    }
}

pub struct InstancedBufferTillOff {
    pub slot: usize,
    key: render_data_container::KeyVertexBuffer,
}
impl TInstancedBuffer for InstancedBufferTillOff {
    fn display_name() -> String {
        String::from("InstanceTillOff")
    }
    fn new(index: usize, id: String, pool: &mut VertexBufferPool) -> Self {
        let buffer: VertexBuffer = VertexBuffer::new(true, EVertexDataFormat::F32, false);
        let key = render_data_container::KeyVertexBuffer::from(id + "TillOff");
        pool.map.insert(key.clone(), buffer);

        Self {
            slot: index,
            key
        }
    }
    fn key(&self) -> render_data_container::KeyVertexBuffer {
        self.key.clone()
    }

    fn slot(&self) -> crate::geometry::vertex_buffer_useinfo::EVertexBufferSlot {
        vertex_buffer_useinfo::EVertexBufferSlot::from_u8_unsafe(self.slot as u8)
    }
}