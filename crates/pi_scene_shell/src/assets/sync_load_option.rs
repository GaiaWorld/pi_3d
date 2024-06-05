
// use crate::ecs::*;
// // use bevy_app::{App, Update, Plugin};
// // use bevy_ecs::{system::{Resource, Query, Commands, Res, ResMut}, component::Component, query::Changed, schedule::IntoSystemConfigs};

// use std::{marker::PhantomData, ops::Deref, hash::Hash};
// use pi_assets::{asset::{Handle, Asset, GarbageEmpty}, mgr::{AssetMgr, LoadResult}};
// use pi_bevy_asset::ShareAssetMgr;
// use pi_hash::XHashMap;
// use pi_share::ThreadSync;

// use crate::{run_stage::{TSystemStageInfo, ERunStageChap}, object::ObjectID};


// #[derive(Default, Resource)]
// pub struct AssetSyncWaitOption<
//     K0: Clone + Hash + PartialEq + Eq + Component,
//     K: Deref<Target = Option<K0>> + Component,
//     D: Asset<Key = K0> + Component,
//     R: From<Handle<D>> + Component
// >(
//     pub XHashMap<K0, Vec<ObjectID>>,
//     pub Vec<(K0, Handle<D>)>,
//     PhantomData<(K, D, R)>,
// );

// impl<
//     K0: Clone + Hash + PartialEq + Eq + Component,
//     K: Deref<Target = Option<K0>> + Component,
//     D: Asset<Key = K0> + Component,
//     R: From<Handle<D>> + Component
// > AssetSyncWaitOption<K0, K, D, R> 
// {
//     pub fn loaded(&mut self, key: K0, data: Handle<D>) {
//         self.1.push((key, data))
//     }
// }

//     pub fn sys_sync_load_option_create<
//         K0: Clone + Hash + PartialEq + Eq + Component,
//         K: Deref<Target = Option<K0>> + Component,
//         D: Asset<Key = K0> + Component,
//         R: From<Handle<D>> + Component
//     >(
//         query: Query<(ObjectID, &K), Changed<K>>,
//         mut data_cmd: Commands,
//         assets_mgr: Res<ShareAssetMgr<D>>,
//         mut list_await: ResMut<AssetSyncWaitOption<K0, K, D, R>>,
//     ) {
//         // log::debug!("AssetSyncLoadOption: {} , {}", std::any::type_name::<K>(), std::any::type_name::<S>());
//         query.iter().for_each(|(entity, key)| {
//             if let Some(key) = key.deref() {
//                 let result = AssetMgr::load(&assets_mgr, key);
//                 // log::debug!("AssetSyncLoad: {:?}", key.deref());

//                 match result {
//                     LoadResult::Ok(r) => {
//                         // log::debug!("AssetSyncLoad: Loaded {:?}", key.deref());
//                         if let Some(mut cmd) = data_cmd.get_entity(entity) {
//                             cmd.insert((R::from(r),));
//                         }
//                     },
//                     _ => {
//                         let list = if let Some(list) = list_await.0.get_mut(key) {
//                             list
//                         } else {
//                             list_await.0.insert(key.deref().clone(), vec![]);
//                             list_await.0.get_mut(key).unwrap()
//                         };
        
//                         list.push(entity);
//                     }
//                 }
//             }
//         });
//     }
//     pub fn sys_sync_load_option_check_await<
//         K0: Clone + Hash + PartialEq + Eq + Component,
//         K: Deref<Target = Option<K0>> + Component,
//         D: Asset<Key = K0> + Component,
//         R: From<Handle<D>> + Component
//     >(
//         mut list_await: ResMut<AssetSyncWaitOption<K0, K, D, R>>,
//         query: Query<&K>,
//         mut data_cmd: Commands,
//     ) {
//         // log::debug!("check_await: ");
//         let mut data_list = std::mem::replace(&mut list_await.1, vec![]);
//         data_list.drain(..).for_each(|(key, data)| {
//             if let Some(list) = list_await.0.get_mut(&key) {
//                 let mut ids = std::mem::replace(list, vec![]);
//                 ids.drain(..).for_each(|id| {

//                     match query.get(id) {
//                         Ok(key0) => {
//                             if let Some(key0) = key0.deref() {
//                                 // key 已经修改，不需要设置
//                                 if &key == key0 {
//                                     // log::debug!("AssetSyncLoad: Loaded {:?}", key);
//                                     if let Some(mut cmd) = data_cmd.get_entity(id) {
//                                         cmd.insert((R::from(data.clone()),));
//                                     }
//                                 }
//                             }
//                         }
//                         // 节点已经销毁，或 key 已经被删除，不需要设置
//                         _ => {
//                             list.push(id);
//                         },
//                     };
//                 });
//             };
//         });
//     }

// ///
// /// K0: 资产在资源缓存表的 资产Key
// /// K: 在功能中的 资产Key的Component 包装
// /// D: 资产在资源缓存表的 资产数据
// /// R: 在功能中的 资产数据的Component 包装
// /// S: 资产Key 的更新System
// pub struct PluginAssetSyncLoadOption<
//     K0: Clone + Hash + PartialEq + Eq + Component + 'static + Send + Sync,
//     K: Deref<Target = Option<K0>> + Component + 'static + Send + Sync,
//     D: Asset<Key = K0> + Component + 'static + Send + Sync,
//     R: From<Handle<D>> + Component + 'static + Send + Sync,
//     S: TSystemStageInfo + 'static + ThreadSync
// >(bool, usize, usize, PhantomData<(K, D, R, S)>);

// impl<K0, K, D, R, S> PluginAssetSyncLoadOption<K0, K, D, R, S>
// where
//     K0: Clone + Hash + PartialEq + Eq + Component + 'static + Send + Sync,
//     K: Deref<Target = Option<K0>> + Component + 'static + Send + Sync,
//     D: Asset<Key = K0> + Component + 'static + Send + Sync,
//     R: From<Handle<D>> + Component + 'static + Send + Sync,
//     S: TSystemStageInfo + 'static + Send + Sync
// {
//     ///
//     /// ref_garbage, capacity, timeout 为 AssetMgr::<D> 缓存大小, 内部会检查保证只创建一次
//     pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
//         Self(ref_garbage, capacity, timeout, PhantomData)
//     }
// }

// impl<K0, K, D, R, S> Plugin for PluginAssetSyncLoadOption<K0, K, D, R, S>
// where
//     K0: Clone + Hash + PartialEq + Eq + Component + 'static + Send + Sync,
//     K: Deref<Target = Option<K0>> + Component + 'static + Send + Sync,
//     D: Asset<Key = K0> + Component + 'static + Send + Sync,
//     R: From<Handle<D>> + Component + 'static + Send + Sync,
//     S: TSystemStageInfo + 'static + Send + Sync
// {
//     fn build(&self, app: &mut App) {
//         app.world.insert_resource(AssetSyncWaitOption::<K0, K, D, R>(XHashMap::default(), vec![], PhantomData));
//         if app.world.get_resource::<ShareAssetMgr::<D>>().is_none() {
//             app.world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
//         }

//         app.add_systems(Update, sys_sync_load_option_create::<K0, K, D, R>.in_set(ERunStageChap::Initial));
//         app.add_systems(Update, sys_sync_load_option_check_await::<K0, K, D, R>.in_set(ERunStageChap::Initial));
//     }
// }
