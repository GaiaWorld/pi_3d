use std::{marker::PhantomData, ops::Deref, fmt::Debug, hash::Hash};

use pi_assets::{asset::{Handle, Asset, Garbageer, GarbageEmpty}, mgr::{AssetMgr, LoadResult}};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_hash::XHashMap;
use pi_render::rhi::{RenderQueue, device::RenderDevice};
use pi_share::{ThreadSync, Share};

use crate::{run_stage::{TSystemStageInfo, ERunStageChap}, plugin::{ErrorPlugin, Plugin}, object::{GameObject, ObjectID}, engine_shell::EnginShell};

use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct AssetSyncWait<
    K0: Debug + Clone + Hash + PartialEq + Eq + std::marker::Send + std::marker::Sync + 'static,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + std::marker::Send + std::marker::Sync,
    R: From<Handle<D>> + Component
>(
    pub XHashMap<K0, Vec<ObjectID>>,
    pub Vec<(K0, Handle<D>)>,
    PhantomData<(K, D, R)>,
);
impl<
    K0: Debug + Clone + Hash + PartialEq + Eq + std::marker::Send + std::marker::Sync + 'static,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + std::marker::Send + std::marker::Sync,
    R: From<Handle<D>> + Component
> Default for AssetSyncWait<K0, K, D, R>  {
    fn default() -> Self {
        Self(
            XHashMap::default(),
            vec![],
            PhantomData::default()
        )
    }
}

impl<
    K0: Debug + Clone + Hash + PartialEq + Eq + std::marker::Send + std::marker::Sync + 'static,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + std::marker::Send + std::marker::Sync,
    R: From<Handle<D>> + Component
> AssetSyncWait<K0, K, D, R> 
{
    pub fn loaded(&mut self, key: K0, data: Handle<D>) {
        self.1.push((key, data))
    }
}

///
/// * K0: 资产在资源缓存表的 资产Key
/// * K: 在功能中的 资产Key的Component 包装
/// * D: 资产在资源缓存表的 资产数据
/// * R: 在功能中的 资产数据的Component 包装
/// * S: 资产Key 的更新System
// pub struct AssetSyncLoad<
//     K0: Debug + Clone + Hash + PartialEq + Eq + Component,
//     K: Deref<Target = K0> + Component,
//     D: Asset<Key = K0> + Component,
//     R: From<Handle<D>> + Component,
//     S: TSystemStageInfo + 'static
// >(PhantomData<(K, D, R, S)>);

// impl<
//     K0: Debug + Clone + Hash + PartialEq + Eq + Component,
//     K: Deref<Target = K0> + Component,
//     D: Asset<Key = K0> + Component,
//     R: From<Handle<D>> + Component,
//     S: TSystemStageInfo + 'static
// > TSystemStageInfo for AssetSyncLoad<K0, K, D, R, S> {
//     fn depends() -> Vec<KeySystem> {
//         vec![
//             S::key(), 
//         ]
//     }
// }

// impl<K0, K, D, R, S> AssetSyncLoad<K0, K, D, R,S>
// where
//     K0: Debug + Clone + Hash + PartialEq + Eq + Component,
//     K: Deref<Target = K0> + Component,
//     D: Asset<Key = K0> + Component,
//     R: From<Handle<D>> + Component,
//     S: TSystemStageInfo + 'static
// {
    pub fn sys_sync_load_create<
        K0: Debug + Clone + Hash + PartialEq + Eq + std::marker::Send + std::marker::Sync + 'static,
        K: Deref<Target = K0> + Component,
        D: Asset<Key = K0> + std::marker::Send + std::marker::Sync,
        R: From<Handle<D>> + Component,
    >(
        query: Query<(ObjectID, &K), Changed<K>>,
        mut data_cmd: Commands,
        assets_mgr: Res<ShareAssetMgr<D>>,
        mut list_await: ResMut<AssetSyncWait<K0, K, D, R>>,
    ) {
        // log::debug!("AssetSyncLoad: {} , {}", std::any::type_name::<K>(), std::any::type_name::<S>());
        query.iter().for_each(|(entity, key)| {
            let result = AssetMgr::load(&assets_mgr, key.clone());
            // log::debug!("AssetSyncLoad: {:?}", key.deref());
            
            match result {
                LoadResult::Ok(r) => {
                    // log::debug!("AssetSyncLoad: Loaded {:?}", key.deref());
                    data_cmd.entity(entity).insert(R::from(r));
                },
                _ => {
                    let list = if let Some(list) = list_await.0.get_mut(key) {
                        list
                    } else {
                        list_await.0.insert(key.deref().clone(), vec![]);
                        list_await.0.get_mut(key).unwrap()
                    };
    
                    list.push(entity);
                }
            }
        });
    }
// }

// pub struct AssetSyncLoadCheck<
//     K0: Debug + Clone + Hash + PartialEq + Eq + Component,
//     K: Deref<Target = K0> + Component,
//     D: Asset<Key = K0> + Component,
//     R: From<Handle<D>> + Component
// >(PhantomData<(K, D, R)>);
// impl<
//     K0: Debug + Clone + Hash + PartialEq + Eq + Component,
//     K: Deref<Target = K0> + Component,
//     D: Asset<Key = K0> + Component,
//     R: From<Handle<D>> + Component
// > TSystemStageInfo for AssetSyncLoadCheck<K0, K, D, R> {
// }

// impl<K0, K, D, R> AssetSyncLoadCheck<K0, K, D, R>
// where
//     K0: Debug + Clone + Hash + PartialEq + Eq + Component,
//     K: Deref<Target = K0> + Component,
//     D: Asset<Key = K0> + Component,
//     R: From<Handle<D>> + Component
// {
    pub fn sys_sync_load_check_await<
        K0: Debug + Clone + Hash + PartialEq + Eq + Send + Sync + 'static,
        K: Deref<Target = K0> + Component,
        D: Asset<Key = K0> + Send + Sync,
        R: From<Handle<D>> + Component,
    >(
        mut list_await: ResMut<AssetSyncWait<K0, K, D, R>>,
        query: Query<&K>,
        mut data_cmd: Commands,
    ) {
        // // log::debug!("check_await: ");
        // let mut data_list = std::mem::replace(&mut list_await.1, vec![]);
        // data_list.drain(..).for_each(|(key, data)| {
        //     if let Some(list) = list_await.0.get_mut(&key) {
        //         let mut ids = std::mem::replace(list, vec![]);
        //         ids.drain(..).for_each(|id| {

        //             match query.get(id) {
        //                 Ok(key0) => {
        //                     // key 已经修改，不需要设置
        //                     if key == *key0.deref() {
        //                         // log::debug!("AssetSyncLoad: Loaded {:?}", key);
        //                         data_cmd.entity(id).insert(R::from(data.clone()));
        //                     }
        //                 }
        //                 // 节点已经销毁，或 key 已经被删除，不需要设置
        //                 _ => {
        //                     list.push(id);
        //                 },
        //             };
        //         });
        //     };
        // });
    }
// }


// pub trait InterfaceAssetSyncCreate<K0, D>
// where
//     K0: Debug + Clone + Hash + PartialEq + Send + Sync + 'static,
//     D: Asset<Key = K0>  + Send + Sync,
//  {
//     fn create_asset(
//         &self,
//         key: K0,
//         data: D,
//     ) -> Handle<D>;
//     fn check_asset(
//         &self,
//         key: &K0,
//     ) -> bool;
// }

// impl<K0, D> InterfaceAssetSyncCreate<K0, D> for ShareAssetMgr<D>
// where
//     K0: Debug + Clone + Hash + PartialEq + Eq + Send + Sync + 'static,
//     D: Asset<Key = K0>  + Send + Sync,
// {
//     fn create_asset(
//         &self,
//         key: K0,
//         data: D,
//     ) -> Handle<D> {
//         self.insert(key.clone(), data).unwrap()
//     }

//     fn check_asset(
//         &self,
//         key: &K0,
//     ) -> bool {
//         self.contains_key(key)
//     }
// }

///
/// K0: 资产在资源缓存表的 资产Key
/// K: 在功能中的 资产Key的Component 包装
/// D: 资产在资源缓存表的 资产数据
/// R: 在功能中的 资产数据的Component 包装
/// S: 资产Key 的更新System
pub struct PluginAssetSyncLoad<
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component,
    S: TSystemStageInfo + 'static + ThreadSync
>(bool, usize, usize, PhantomData<(K, D, R, S)>);

impl<K0, K, D, R, S> PluginAssetSyncLoad<K0, K, D, R, S>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component,
    S: TSystemStageInfo + 'static + ThreadSync
{
    ///
    /// ref_garbage, capacity, timeout 为 AssetMgr::<D> 缓存大小, 内部会检查保证只创建一次
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(ref_garbage, capacity, timeout, PhantomData)
    }
}

impl<K0, K, D, R, S> Plugin for PluginAssetSyncLoad<K0, K, D, R, S>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    K: Deref<Target = K0> + Component,
    D: Asset<Key = K0> + Component,
    R: From<Handle<D>> + Component,
    S: TSystemStageInfo + 'static + ThreadSync
{
    // fn init(
    //     &mut self,
    //     engine: &mut EnginShell,
    //     stages: &mut RunStage,
    // ) -> Result<(), ErrorPlugin> {

    //     let world = engine.world_mut();
    //     world.insert_resource(AssetSyncWait::<K0, K, D, R>(XHashMap::default(), vec![], PhantomData));
    //     if world.get_resource::<AssetMgr::<D>>().is_none() {
    //         world.insert_resource(AssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
    //     }

    //     AssetSyncLoad::<K0, K, D, R, S>::setup(world, stages.query_stage::<AssetSyncLoad::<K0, K, D, R, S>>(ERunStageChap::Initial));
    //     AssetSyncLoadCheck::<K0, K, D, R>::setup(world, stages.query_stage::<AssetSyncLoadCheck::<K0, K, D, R>>(ERunStageChap::Initial));

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        
        let world = &mut app.world;
        world.insert_resource(AssetSyncWait::<K0, K, D, R>(XHashMap::default(), vec![], PhantomData));
        if world.get_resource::<ShareAssetMgr::<D>>().is_none() {
            world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
        }

        app.add_system(sys_sync_load_create::<K0, K, D, R>.in_set(ERunStageChap::Initial)); 
        app.add_system(sys_sync_load_check_await::<K0, K, D, R>.in_set(ERunStageChap::Initial)); 
        // AssetSyncLoad::<K0, K, D, R, S>::setup(world, stages.query_stage::<AssetSyncLoad::<K0, K, D, R, S>>(ERunStageChap::Initial));
        // AssetSyncLoadCheck::<K0, K, D, R>::setup(world, stages.query_stage::<AssetSyncLoadCheck::<K0, K, D, R>>(ERunStageChap::Initial));

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
    /// ref_garbage, capacity, timeout 为 AssetMgr::<D> 缓存大小, 内部会检查保证只创建一次
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(ref_garbage, capacity, timeout, PhantomData)
    }
}

impl<K0, D> Plugin for PluginAssetSyncNotNeedLoad<K0, D>
where
    K0: Debug + Clone + Hash + PartialEq + Eq + Component,
    D: Asset<Key = K0> + Component,
{
    // fn init(
    //     &mut self,
    //     engine: &mut EnginShell,
    //     stages: &mut RunStage,
    // ) -> Result<(), ErrorPlugin> {

    //     let world = engine.world_mut();
    //     if world.get_resource::<AssetMgr::<D>>().is_none() {
    //         world.insert_resource(AssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
    //     }

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        
        let world = &mut app.world;
        if world.get_resource::<ShareAssetMgr::<D>>().is_none() {
            world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
        }
    }
}