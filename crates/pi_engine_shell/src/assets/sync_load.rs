use std::{marker::PhantomData, ops::Deref, fmt::Debug, hash::Hash};

use pi_assets::{asset::{Handle, Asset, Garbageer, GarbageEmpty}, mgr::{AssetMgr, LoadResult}};
use pi_ecs::{prelude::{Id, Event, Query, Res, Component, ResMut, Setup}, query::Write, world::World};
use pi_ecs_macros::{setup, listen};
use pi_hash::XHashMap;
use pi_render::rhi::{RenderQueue, device::RenderDevice};
use pi_share::{ThreadSync, Share};

use crate::{run_stage::RunStage, plugin::{ErrorPlugin, Plugin}, object::{GameObject, ObjectID}, engine_shell::EnginShell};

#[derive(Debug, Default)]
pub struct AssetSyncWait<
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component
>(
    pub XHashMap<K0, Vec<ObjectID>>,
    pub Vec<(K0, Handle<D>)>,
    PhantomData<(K, D, R)>,
);

impl<
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component
> AssetSyncWait<K0, K, D, R> 
{
    pub fn loaded(&mut self, key: K0, data: Handle<D>) {
        self.1.push((key, data))
    }
}

pub struct AssetSyncLoad<
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component
>(PhantomData<(K, D, R)>);

#[setup]
impl<K0, K, D, R> AssetSyncLoad<K0, K, D, R>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component
{
    #[listen(component=(GameObject, K, (Create, Modify)))]
    pub fn create(
        e: Event,
        mut query: Query<GameObject, (&K, Write<R>)>,
        assets_mgr: Res<Share<AssetMgr<D>>>,
        mut list_await: ResMut<AssetSyncWait<K0, K, D, R>>,
        queue: Res<RenderQueue>,
        device: Res<RenderDevice>,
    ) {
        let (key, mut res) = query.get_unchecked_mut_by_entity(e.id);
        let result = AssetMgr::load(&assets_mgr, key);
        
        match result {
            LoadResult::Ok(r) => res.write(R::from(r)),
            _ => {
                let list = if let Some(list) = list_await.0.get_mut(key) {
                    list
                } else {
                    list_await.0.insert(key.deref().clone(), vec![]);
                    list_await.0.get_mut(key).unwrap()
                };

                list.push(unsafe { Id::new(e.id.local()) });
            }
        }
    }
    
    //
    #[system]
    pub fn check_await(
        mut list_await: ResMut<AssetSyncWait<K0, K, D, R>>,
        mut query: Query<GameObject, (&K, Write<R>)>
    ) {
        // println!("check_await: ");
        let mut data_list = std::mem::replace(&mut list_await.1, vec![]);
        data_list.drain(..).for_each(|(key, data)| {
            // println!("{:?}", key);
            if let Some(list) = list_await.0.get_mut(&key) {
                let mut ids = std::mem::replace(list, vec![]);
                ids.drain(..).for_each(|id| {

                    match query.get_mut(id) {
                        Some((key0, mut item)) => {
                            // key ??????????????????????????????
                            if key == *key0.deref() {
                                println!("{:?}", key);
                                item.write(R::from(data.clone()));
                            }
                        }
                        // ???????????????????????? key ?????????????????????????????????
                        None => {},
                    };
                });
            };
        });
    }
}


pub trait InterfaceAssetSyncCreate<K0, D>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    D: Asset<Key = K0> + Component,
 {
    fn create_asset(
        &self,
        key: K0,
        data: D,
    ) -> Handle<D>;
    fn check_asset(
        &self,
        key: &K0,
    ) -> bool;
}

impl<K0, D> InterfaceAssetSyncCreate<K0, D> for Share<AssetMgr<D>>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    D: Asset<Key = K0> + Component,
{
    fn create_asset(
        &self,
        key: K0,
        data: D,
    ) -> Handle<D> {
        self.insert(key.clone(), data).unwrap()
    }

    fn check_asset(
        &self,
        key: &K0,
    ) -> bool {
        self.contains_key(key)
    }
}

pub struct PluginAssetSyncLoad<
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component
>(bool, usize, usize, PhantomData<(K, D, R)>);

impl<K0, K, D, R> PluginAssetSyncLoad<K0, K, D, R>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component
{
    ///
    /// ref_garbage, capacity, timeout ??? AssetMgr::<D> ????????????, ????????????????????????????????????
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(ref_garbage, capacity, timeout, PhantomData)
    }
}

impl<K0, K, D, R> Plugin for PluginAssetSyncLoad<K0, K, D, R>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component
{
    fn init(
        &mut self,
        engine: &mut EnginShell,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin> {

        let world = engine.world_mut();
        world.insert_resource(AssetSyncWait::<K0, K, D, R>(XHashMap::default(), vec![], PhantomData));
        if world.get_resource::<AssetMgr::<D>>().is_none() {
            world.insert_resource(AssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
        }

        AssetSyncLoad::<K0, K, D, R>::setup(world, stages.command_stage());

        Ok(())
    }
}

pub struct PluginAssetSyncNotNeedLoad<
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    D: Asset<Key = K0> + Component,
>(bool, usize, usize, PhantomData<D>);

impl<K0, D> PluginAssetSyncNotNeedLoad<K0, D>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    D: Asset<Key = K0> + Component,
{
    ///
    /// ref_garbage, capacity, timeout ??? AssetMgr::<D> ????????????, ????????????????????????????????????
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(ref_garbage, capacity, timeout, PhantomData)
    }
}

impl<K0, D> Plugin for PluginAssetSyncNotNeedLoad<K0, D>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    D: Asset<Key = K0> + Component,
{
    fn init(
        &mut self,
        engine: &mut EnginShell,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin> {

        let world = engine.world_mut();
        if world.get_resource::<AssetMgr::<D>>().is_none() {
            world.insert_resource(AssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
        }

        Ok(())
    }
}