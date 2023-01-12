
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use render_data_container::{KeyVertexBuffer, VertexBufferPool};

use crate::geometry::{vertex_buffer_useinfo};

use super::types::TInstancedData;

pub trait TInstancedBuffer {
    fn display_name() -> String;
    fn new(index: usize, id: String, pool: &mut VertexBufferPool) -> Self;
    fn key(&self) -> KeyVertexBuffer;
    fn slot(&self) -> vertex_buffer_useinfo::EVertexBufferSlot;
    fn update<V: TInstancedData>(
        &self,
        items: &[&V],
        pool: &mut VertexBufferPool,
        device: &RenderDevice,
        queue: &RenderQueue,
    ) {
        let buffer = pool.map.get_mut(&self.key()).unwrap();
        let mut index = 0;
        items.into_iter().for_each(|item| {
            item.write_instance_buffer(buffer, index * V::size());
            index += 1;
        });

        buffer.update_buffer(device, queue);
    }
    fn update_item<V: TInstancedData>(
        &self,
        item: &V,
        index: usize,
        pool: &mut VertexBufferPool,
        device: &RenderDevice,
        queue: &RenderQueue,
    ) {
        let buffer = pool.map.get_mut(&self.key()).unwrap();

        item.write_instance_buffer(buffer, index * V::size());

        buffer.update_buffer(device, queue);
    }
}