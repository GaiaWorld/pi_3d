use pi_engine_shell::prelude::*;
use pi_render::renderer::{vertex_buffer_loader::VertexBufferLoader, vertex_buffer::VertexBufferAllocator};

#[derive(Deref, DerefMut)]
pub struct VBLoaderSlot<T: Clone + Hash + PartialEq + Eq, D: From<EVerticesBufferUsage>>(pub VertexBufferLoader<T, D>);

#[derive(Deref, DerefMut)]
pub struct VBAllocator(pub VertexBufferAllocator);