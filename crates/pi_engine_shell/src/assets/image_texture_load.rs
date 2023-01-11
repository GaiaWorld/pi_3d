use std::marker::PhantomData;

use crossbeam::queue::SegQueue;
use derive_deref::{Deref, DerefMut};
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult},
};
use pi_async::prelude::AsyncRuntime;
use pi_atom::Atom;
use pi_ecs::{
    entity::Id,
    monitor::Event,
    prelude::{Query, Res, Write, Commands, StageBuilder, Setup}, query::Changed, sys::system, world::World,
};
use pi_ecs_macros::{listen, setup};
use pi_hal::{loader::AsyncLoader, runtime::MULTI_MEDIA_RUNTIME};
use pi_render::rhi::{
    asset::{ImageTextureDesc, TextureRes},
    device::RenderDevice,
    RenderQueue,
};
use pi_share::{Share, ThreadSync};
use crate::{object::{ObjectID, GameObject}, run_stage::{TSystemStageInfo, SysCommonUserCommand}};

#[derive(Clone, DerefMut, Deref)]
pub struct ImageAwait<T>(Share<SegQueue<(ObjectID, Atom, Handle<TextureRes>)>>, PhantomData<T>);

impl<T> Default for ImageAwait<T> {
    fn default() -> Self { Self(Share::new(SegQueue::new()), PhantomData) }
}

pub struct SysImageLoad;
impl TSystemStageInfo for SysImageLoad {
    fn depends() -> Vec<crate::run_stage::KeySystem> {
        vec![
            SysCommonUserCommand::key()
        ]
    }
}

pub struct CalcImageLoad<S: std::ops::Deref<Target = Atom>, D: From<Handle<TextureRes>>>(PhantomData<(S, D)>);
impl<S, D> CalcImageLoad<S, D> 
where
    S: std::ops::Deref<Target = Atom> + 'static + ThreadSync,
    D: From<Handle<TextureRes>> + 'static + ThreadSync,
{
    pub fn setup(world: &mut World, stage_builder: &mut StageBuilder) {
        SysKeyImageChange::<S, D>::setup(world, stage_builder);
        SysKeyImageCheck::<S, D>::setup(world, stage_builder);
    }
}

// #[setup]
// impl<S, D> CalcImageLoad<S, D>
// where
//     S: std::ops::Deref<Target = Atom> + 'static + ThreadSync,
//     D: From<Handle<TextureRes>> + 'static + ThreadSync,
// {
//     /// Image创建，加载对应的图片
//     /// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
//     /// 因为BorderImageTexture未加锁，其他线程可能正在使用
//     /// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
//     #[listen(component=(GameObject, S, (Create, Modify)))]
//     pub fn image_change(
//         e: Event,
//         query: Query<GameObject, (ObjectID, &S)>,
//         mut image_cmd: Commands<GameObject, D>,
//         texture_assets_mgr: Res<Share<AssetMgr<TextureRes>>>,
//         image_await: Res<ImageAwait<S>>,
//         queue: Res<RenderQueue>,
//         device: Res<RenderDevice>,
//     ) {
//         log::debug!("image_change: ");
//         let (obj, key) = query.get_unchecked_by_entity(e.id);
//         let result = AssetMgr::load(&texture_assets_mgr, &(key.get_hash() as u64));
//         match result {
//             LoadResult::Ok(r) => {
//                 image_cmd.insert(
//                     unsafe {
//                         Id::<GameObject>::new(e.id.local())
//                     }, 
//                     D::from(r)
//                 )
//             }
//             ,
//             _ => {
//                 let (awaits, device, queue) = ((*image_await).clone(), (*device).clone(), (*queue).clone());
//                 let (id, key) = (unsafe {
//                     Id::<GameObject>::new(e.id.local())
//                 }, (*key).clone());

//                 MULTI_MEDIA_RUNTIME
//                     .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
//                         let desc = ImageTextureDesc {
//                             url: &key,
//                             device: &device,
//                             queue: &queue,
//                         };

//                         let r = TextureRes::async_load(desc, result).await;
//                         match r {
//                             Ok(r) => {
//                                 awaits.push((id, key.clone(), r));
//                             }
//                             Err(e) => {
//                                 log::error!("load image fail, {:?}", e);
//                             }
//                         };
//                     })
//                     .unwrap();
//             }
//         }
//     }

//     //
//     #[system]
//     pub fn check_await_texture(
//         image_await: Res<ImageAwait<S>>,
//         mut query: Query<GameObject, &S>,
//         mut image_cmd: Commands<GameObject, D>,
//     ) {
//         // log::debug!("check_await_texture: ");
//         // let awaits = std::mem::replace(&mut border_image_await.0, Share::new(SegQueue::new()));
//         let mut r = image_await.0.pop();
//         while let Some((id, key, texture)) = r {
//             r = image_await.0.pop();

//             let mut nowkey = match query.get_mut(id.clone()) {
//                 Some(img) => {
//                     // image已经修改，不需要设置texture
//                     if **img != key {
//                         continue;
//                     }
//                     image_cmd.insert(id, D::from(texture));
//                 }
//                 // 节点已经销毁，或image已经被删除，不需要设置texture
//                 None => continue,
//             };
//             log::debug!("Write texture_item $$$");
//         }
//     }
// }

pub struct SysKeyImageChange<S: std::ops::Deref<Target = Atom>, D: From<Handle<TextureRes>>>(PhantomData<(S, D)>);
#[setup]
impl<S, D> SysKeyImageChange<S, D> 
where
    S: std::ops::Deref<Target = Atom> + 'static + ThreadSync,
    D: From<Handle<TextureRes>> + 'static + ThreadSync,
{
    #[system]
    pub fn image_change(
        query: Query<GameObject, (ObjectID, &S), Changed<S>>,
        mut image_cmd: Commands<GameObject, D>,
        texture_assets_mgr: Res<Share<AssetMgr<TextureRes>>>,
        image_await: Res<ImageAwait<S>>,
        queue: Res<RenderQueue>,
        device: Res<RenderDevice>,
    ) {
        log::debug!("image_change: ");
        query.iter().for_each(|(obj, key)| {
            let result = AssetMgr::load(&texture_assets_mgr, &(key.get_hash() as u64));
            match result {
                LoadResult::Ok(r) => {
                    log::info!("image_loaded: {:?}", key.as_str());
                    image_cmd.insert(
                        obj, 
                        D::from(r)
                    )
                }
                ,
                _ => {
                    let (awaits, device, queue) = ((*image_await).clone(), (*device).clone(), (*queue).clone());
                    let (id, key) = (obj, (*key).clone());
    
                    MULTI_MEDIA_RUNTIME
                        .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                            let desc = ImageTextureDesc {
                                url: &key,
                                device: &device,
                                queue: &queue,
                            };
    
                            let r = TextureRes::async_load(desc, result).await;
                            match r {
                                Ok(r) => {
                                    awaits.push((id, key.clone(), r));
                                }
                                Err(e) => {
                                    log::error!("load image fail, {:?}", e);
                                }
                            };
                        })
                        .unwrap();
                }
            }
        });
    }
}

pub struct SysKeyImageCheck<S: std::ops::Deref<Target = Atom>, D: From<Handle<TextureRes>>>(PhantomData<(S, D)>);
#[setup]
impl<S, D> SysKeyImageCheck<S, D> 
where
    S: std::ops::Deref<Target = Atom> + 'static + ThreadSync,
    D: From<Handle<TextureRes>> + 'static + ThreadSync,
{
    #[system]
    pub fn check_await_texture(
        image_await: Res<ImageAwait<S>>,
        mut query: Query<GameObject, &S>,
        mut image_cmd: Commands<GameObject, D>,
    ) {
        // log::debug!("check_await_texture: ");
        // let awaits = std::mem::replace(&mut border_image_await.0, Share::new(SegQueue::new()));
        let mut r = image_await.0.pop();
        while let Some((id, key, texture)) = r {
            r = image_await.0.pop();

            let mut nowkey = match query.get_mut(id.clone()) {
                Some(img) => {
                    // image已经修改，不需要设置texture
                    if **img != key {
                        continue;
                    }
                    log::info!("image_loaded: {:?}", key);
                    image_cmd.insert(id, D::from(texture));
                }
                // 节点已经销毁，或image已经被删除，不需要设置texture
                None => continue,
            };
            log::debug!("Write texture_item $$$");
        }
    }
}