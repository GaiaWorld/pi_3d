
use pi_engine_shell::prelude::*;

use crate::{geometry::vertex_buffer_useinfo};

#[derive(Component)]
pub struct InstanceBufferWorldMatrix {
    pub slot: usize,
    pub index: IDAssetVertexBuffer,
}
impl TInstanceBuffer for InstanceBufferWorldMatrix {
    fn display_name() -> String {
        String::from("InstanceWorldMatrix")
    }

    fn slot(&self) -> EVertexBufferSlot {
        EVertexBufferSlot::from_u8_unsafe(self.slot as u8)
    }

    fn id(&mut self) -> KeyVertexBuffer {
        // self.index += 1;
        // KeyVertexBuffer::from(self.id.clone() + self.index.to_string().as_str())
        KeyVertexBuffer::from(self.index)
    }
}

#[derive(Component)]
pub struct InstanceWorldMatrixDirty(pub bool);
impl Default for InstanceWorldMatrixDirty {
    fn default() -> Self {
        Self(false)
    }
}
impl TInstanceFlag for InstanceWorldMatrixDirty {
    fn dirty(&self) -> bool {
        self.0
    }
    fn reset(&mut self) {
        self.0 = false;
    }
}
