use std::marker::PhantomData;

use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_ecs::{prelude::{Event, Res, Query, Component}, query::Write};
use pi_ecs_macros::{setup, listen};
use pi_engine_shell::{plugin::Plugin, object::{ObjectID, GameObject}};
use pi_hash::XHashMap;
use pi_render::rhi::{RenderQueue, device::RenderDevice};
use pi_share::Share;
use render_resource::sampler::{SamplerAssetKey, AssetSampler};

#[derive(Debug)]
pub struct SamplerAwait<K: std::ops::Deref<Target = SamplerAssetKey> + Component, D: From<Handle<AssetSampler>> + Component>(XHashMap<SamplerAssetKey, Vec<ObjectID>>, PhantomData<(K, D)>);
impl<K, D> Default for SamplerAwait<K, D>
where 
    K: std::ops::Deref<Target = SamplerAssetKey> + Component,
    D: From<Handle<AssetSampler>> + Component,
{
    fn default() -> Self {
        Self(XHashMap::default(), PhantomData)
    }
}

pub struct SysSamplerLoad<K: std::ops::Deref<Target = SamplerAssetKey> + Component, D: From<Handle<AssetSampler>> + Component>(PhantomData<(K, D)>);
#[setup]
impl<K, D> SysSamplerLoad<K, D>
where 
    K: std::ops::Deref<Target = SamplerAssetKey> + Component,
    D: From<Handle<AssetSampler>> + Component,
{
    #[listen(component=(GameObject, K, (Create, Modify)))]
    pub fn create(
        e: Event,
        mut query: Query<GameObject, (&K, Write<D>)>,
        texture_assets_mgr: Res<Share<AssetMgr<AssetSampler>>>,
        image_await: Res<SamplerAwait<K, D>>,
        queue: Res<RenderQueue>,
        device: Res<RenderDevice>,
    ) {

    }
    
}

pub struct PluginAssetSampler<K: std::ops::Deref<Target = SamplerAssetKey> + Component, D: From<Handle<AssetSampler>> + Component>(PhantomData<(K, D)>);
impl<K: std::ops::Deref<Target = SamplerAssetKey> + Component, D: From<Handle<AssetSampler>> + Component> Plugin for PluginAssetSampler<K, D> {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {

        let world = engine.world_mut();

        world.insert_resource(SamplerAwait::<K, D>::default());

        Ok(())
    }
}
