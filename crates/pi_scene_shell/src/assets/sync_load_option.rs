use std::{marker::PhantomData, ops::Deref, hash::Hash};
use pi_assets::{asset::{Handle, Asset, GarbageEmpty}, mgr::{AssetMgr, LoadResult}};
use pi_bevy_asset::ShareAssetMgr;
use pi_hash::XHashMap;
use pi_share::ThreadSync;
use pi_world::{alter::Alter, filter::Changed, insert::Bundle, prelude::App, query::Query, schedule::Update, single_res::{SingleRes, SingleResMut}};
use pi_world_extend_plugin::plugin::Plugin;

use crate::{run_stage::{TSystemStageInfo}, object::ObjectID};


#[derive(Default)]
pub struct AssetSyncWaitOption<
    K0: Clone + Hash + PartialEq + Eq ,
    K: Deref<Target = Option<K0>> ,
    D: Asset<Key = K0> ,
    R: From<Handle<D>> + Bundle<Item = R>
>(
    pub XHashMap<K0, Vec<ObjectID>>,
    pub Vec<(K0, Handle<D>)>,
    PhantomData<(K, D, R)>,
);

impl<
    K0: Clone + Hash + PartialEq + Eq ,
    K: Deref<Target = Option<K0>> ,
    D: Asset<Key = K0> ,
    R: From<Handle<D>> + Bundle<Item = R>
> AssetSyncWaitOption<K0, K, D, R> 
{
    pub fn loaded(&mut self, key: K0, data: Handle<D>) {
        self.1.push((key, data))
    }
}

    pub fn sys_sync_load_option_create<
        K0: Clone + Hash + PartialEq + Eq ,
        K: Deref<Target = Option<K0>> ,
        D: Asset<Key = K0> ,
        R: From<Handle<D>> + Bundle<Item = R>
    >(
        query: Query<(ObjectID, &K), Changed<K>>,
        mut data_cmd: Alter<(), (), R, ()>,
        assets_mgr: SingleRes<ShareAssetMgr<D>>,
        mut list_await: SingleResMut<AssetSyncWaitOption<K0, K, D, R>>,
    ) {
        // log::debug!("AssetSyncLoadOption: {} , {}", std::any::type_name::<K>(), std::any::type_name::<S>());
        query.iter().for_each(|(entity, key)| {
            if let Some(key) = key.deref() {
                let result = AssetMgr::load(&assets_mgr, key);
                // log::debug!("AssetSyncLoad: {:?}", key.deref());

                match result {
                    LoadResult::Ok(r) => {
                        // log::debug!("AssetSyncLoad: Loaded {:?}", key.deref());
                        if  data_cmd.get(entity).is_ok() {
                            // data_cmd.alter(entity, <R::from<R> as Bundle>::Item);
                        }
                    },
                    _ => {
                        // let list = if let Some(list) = list_await.0.get_mut(key) {
                        //     list
                        // } else {
                        //     list_await.0.insert(key.deref().clone(), vec![]);
                        //     list_await.0.get_mut(key).unwrap()
                        // };
        
                        // list.push(entity);
                    }
                }
            }
        });
    }
    pub fn sys_sync_load_option_check_await<
        K0: Clone + Hash + PartialEq + Eq ,
        K: Deref<Target = Option<K0>>,
        D: Asset<Key = K0>,
        R: From<Handle<D>> + Bundle<Item = R>
    >(
        mut list_await: SingleResMut<AssetSyncWaitOption<K0, K, D, R>>,
        query: Query<&K>,
        mut data_cmd: Alter<(), (), R, ()>,
    ) {
        // log::debug!("check_await: ");
        // let mut data_list = std::mem::replace(&mut list_await.1, vec![]);
        // data_list.drain(..).for_each(|(key, data)| {
        //     if let Some(list) = list_await.0.get_mut(&key) {
        //         let mut ids = std::mem::replace(list, vec![]);
        //         ids.drain(..).for_each(|id| {

        //             match query.get(id) {
        //                 Ok(key0) => {
        //                     if let Some(key0) = key0.deref() {
        //                         // key 已经修改，不需要设置
        //                         if &key == key0 {
        //                             // log::debug!("AssetSyncLoad: Loaded {:?}", key);
        //                             if data_cmd.get(id).is_ok() {
        //                                 data_cmd.alter(id, R::from(data.clone()));
        //                             }
        //                         }
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

///
/// K0: 资产在资源缓存表的 资产Key
/// K: 在功能中的 资产Key的Component 包装
/// D: 资产在资源缓存表的 资产数据
/// R: 在功能中的 资产数据的Component 包装
/// S: 资产Key 的更新System
pub struct PluginAssetSyncLoadOption<
    // T: Send + Sync +'static,
    K0: Clone + Hash + PartialEq + Eq + 'static + ThreadSync,
    K: Deref<Target = Option<K0>>  + 'static + ThreadSync,
    D: Asset<Key = K0>  + 'static + ThreadSync,
    R: From<Handle<D>>  + 'static + ThreadSync + Bundle<Item = R>,
    S: TSystemStageInfo + 'static + ThreadSync
>(bool, usize, usize, PhantomData<(K, D, R, S)>);

impl<K0, K, D, R, S> PluginAssetSyncLoadOption<K0, K, D, R, S>
where
    K0: Clone + Hash + PartialEq + Eq + 'static + ThreadSync,
    K: Deref<Target = Option<K0>> + 'static + ThreadSync,
    D: Asset<Key = K0> + 'static + ThreadSync,
    R: From<Handle<D>> + 'static + ThreadSync + Bundle<Item = R>,
    S: TSystemStageInfo + 'static + Send + Sync
{
    ///
    /// ref_garbage, capacity, timeout 为 AssetMgr::<D> 缓存大小, 内部会检查保证只创建一次
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(ref_garbage, capacity, timeout, PhantomData)
    }
}

impl<K0, K, D, R, S> Plugin for PluginAssetSyncLoadOption<K0, K, D, R, S>
where
    K0: Clone + Hash + PartialEq + Eq + 'static + ThreadSync,
    K: Deref<Target = Option<K0>>+ 'static + ThreadSync+ ,
    D: Asset<Key = K0> + 'static + ThreadSync,
    R: From<Handle<D>> + 'static + ThreadSync + Bundle<Item = R>,
    S: TSystemStageInfo + 'static + Send + Sync
{
    fn build(&self, app: &mut App) {
        app.world.register_single_res(AssetSyncWaitOption::<K0, K, D, R>(XHashMap::default(), vec![], PhantomData));
        if app.world.get_single_res::<ShareAssetMgr::<D>>().is_none() {
            app.world.register_single_res(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
        }

        app.add_system(Update, sys_sync_load_option_create::<K0, K, D, R>);
        app.add_system(Update, sys_sync_load_option_check_await::<K0, K, D, R>);
    }
}
