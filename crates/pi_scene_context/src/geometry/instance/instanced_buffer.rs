
use pi_render::{renderer::vertex_buffer::{KeyVertexBuffer}};

use crate::geometry::{vertex_buffer_useinfo};

pub trait TInstanceBuffer {
    fn display_name() -> String;
    fn slot(&self) -> vertex_buffer_useinfo::EVertexBufferSlot;
    fn id(&mut self) -> KeyVertexBuffer;
}