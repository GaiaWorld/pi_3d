use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::query::Write;
use pi_engine_shell::assets::sync_load::InterfaceAssetSyncCreate;
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_share::Share;
use render_data_container::{KeyVertexBuffer, VertexBuffer, VertexBufferPool};
use render_geometry::vertex_data::{EVertexDataKind, VertexBufferDesc};

use crate::geometry::{vertex_buffer_useinfo};

use super::types::TInstancedData;

pub trait TInstancedBuffer {
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
            item.write_instance_buffer(buffer, index * item.size());
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

        item.write_instance_buffer(buffer, index * item.size());

        buffer.update_buffer(device, queue);
    }
}