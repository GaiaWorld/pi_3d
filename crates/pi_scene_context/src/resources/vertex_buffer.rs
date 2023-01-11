use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use pi_engine_shell::{plugin::Plugin, assets::sync_load::{PluginAssetSyncLoad, AssetSyncLoad}, run_stage::SysCommonUserCommand};
use render_data_container::{VertexBuffer, KeyVertexBuffer, EVertexDataFormat};



#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVertexBuffer(pub KeyVertexBuffer);

#[derive(Deref, DerefMut)]
pub struct AssetResVertexBuffer(pub Handle<VertexBuffer>);
impl From<Handle<VertexBuffer>> for AssetResVertexBuffer {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(value)
    }
}

pub type SysVertexBufferLoad = AssetSyncLoad<KeyVertexBuffer, AssetKeyVertexBuffer, VertexBuffer, AssetResVertexBuffer, SysCommonUserCommand>;
type PluginVertexBufferLoad = PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVertexBuffer, VertexBuffer, AssetResVertexBuffer, SysCommonUserCommand>;

pub struct PluginVertexBuffer;
impl Plugin for PluginVertexBuffer {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        PluginVertexBufferLoad::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);

        Ok(())
    }
}