
use pi_render::renderer::{vertex_buffer::{KeyVertexBuffer}, vertex_buffer_desc::EVertexBufferSlot, instance::{types::TInstanceFlag, instanced_buffer::TInstancedBuffer}};

use crate::{geometry::vertex_buffer_useinfo};


pub struct InstancedBufferWorldMatrix {
    pub slot: usize,
    pub id: String,
    pub index: usize,
}
impl TInstancedBuffer for InstancedBufferWorldMatrix {
    fn display_name() -> String {
        String::from("InstanceWorldMatrix")
    }

    fn slot(&self) -> EVertexBufferSlot {
        EVertexBufferSlot::from_u8_unsafe(self.slot as u8)
    }

    fn id(&mut self) -> KeyVertexBuffer {
        self.index += 1;
        KeyVertexBuffer::from(self.id.clone() + self.index.to_string().as_str())
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