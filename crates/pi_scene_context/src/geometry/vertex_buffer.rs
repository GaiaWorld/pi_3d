use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use pi_engine_shell::{plugin::Plugin, assets::sync_load::{PluginAssetSyncLoad, AssetSyncLoad}};
use render_data_container::{VertexBuffer, KeyVertexBuffer};

use super::sys_vertex_buffer_use::SysGeometryStatesInit;



#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVertexBuffer(pub KeyVertexBuffer);

#[derive(Deref, DerefMut)]
pub struct AssetResVertexBuffer(pub Handle<VertexBuffer>);
impl From<Handle<VertexBuffer>> for AssetResVertexBuffer {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(value)
    }
}

pub type SysVertexBufferLoad = AssetSyncLoad<KeyVertexBuffer, AssetKeyVertexBuffer, VertexBuffer, AssetResVertexBuffer, SysGeometryStatesInit>;
type PluginVertexBufferLoad = PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVertexBuffer, VertexBuffer, AssetResVertexBuffer, SysGeometryStatesInit>;