use render_data_container::{VertexBufferPool, EVertexDataFormat, VertexBuffer};

use crate::geometry::vertex_buffer_useinfo;

use super::{instanced_buffer::TInstancedBuffer, sys_instance::SysInstancedBufferInitFunc, types::TInstanceFlag};


pub struct InstancedBufferWorldMatrix {
    pub slot: usize,
    key: render_data_container::KeyVertexBuffer,
}
impl TInstancedBuffer for InstancedBufferWorldMatrix {
    fn display_name() -> String {
        String::from("InstanceWorldMatrix")
    }
    fn new(index: usize, id: String, pool: &mut VertexBufferPool) -> Self {
        let buffer: VertexBuffer = VertexBuffer::new(true, EVertexDataFormat::F32, false);
        let key = render_data_container::KeyVertexBuffer::from(id + "WorldMat");
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

pub struct InstancedWorldMatrixDirty(pub bool);
impl TInstanceFlag for InstancedWorldMatrixDirty {
    fn dirty(&self) -> bool {
        self.0
    }
    fn reset(&mut self) {
        self.0 = false;
    }
}

pub type SysInstanceBufferWorldMatrixInit = SysInstancedBufferInitFunc<InstancedBufferWorldMatrix>;