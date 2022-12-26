use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::query::Write;
use pi_engine_shell::assets::sync_load::InterfaceAssetSyncCreate;
use pi_share::Share;
use render_data_container::{KeyVertexBuffer, VertexBuffer};
use render_geometry::vertex_data::{EVertexDataKind, VertexBufferDesc};

use crate::geometry::vertex_buffer_useinfo;

use super::types::TInstancedData;


pub trait TInstancedBuffer {
    fn desc(&self) -> VertexBufferDesc;
    fn key(&self) -> KeyVertexBuffer;
    fn slot(&self) -> vertex_buffer_useinfo::EVertexBufferSlot;
    fn update<V: TInstancedData>(
        &self,
        items: impl Iterator<Item = V>,
    ) -> Arc<VertexBuffer> {
        assetmgr.create_asset(k, v)
    }
}