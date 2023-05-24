use std::marker::PhantomData;

use crossbeam::queue::SegQueue;
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult},
};
use pi_async::prelude::AsyncRuntime;
use pi_hal::{runtime::MULTI_MEDIA_RUNTIME, loader::AsyncLoader};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_render::rhi::{
    asset::{ImageTextureDesc, TextureRes},
    device::RenderDevice,
    RenderQueue,
};
use pi_share::{Share, ThreadSync};
use crate::prelude::*;

#[derive(Clone, Resource)]
pub struct ImageAwait<T>(Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage)>>, Share<SegQueue<(ObjectID, EKeyTexture, Handle<ImageTexture>)>>, PhantomData<T>);

impl<T> Default for ImageAwait<T> {
    fn default() -> Self { Self(Share::new(SegQueue::new()), Share::new(SegQueue::new()), PhantomData) }
}

pub struct PluginImageLoad<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(PhantomData<(K, D)>);
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Plugin for PluginImageLoad<K, D> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageAwait::<K>::default());
        app.add_systems(
            (
                check_await_texture::<K, D>,
                image_change::<K, D>,
            ).chain().in_set(ERunStageChap::Initial)
        );
    }
}
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Default for PluginImageLoad<K, D> {
    fn default() -> Self {
        Self(PhantomData::<(K, D)>::default())
    }
}

fn image_change<
    K: std::ops::Deref<Target = EKeyTexture> + Component,
    D: From<ETextureViewUsage> + Component,
>(
    query: Query<(ObjectID, &K), Changed<K>>,
    mut image_cmd: Commands,
    image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    texture_assets_mgr: Res<ShareAssetMgr<ImageTextureView>>,
    textureres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_await: ResMut<ImageAwait<K>>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
) {
    log::debug!("image_change: ");
    query.iter().for_each(|(obj, vkey)| {
        match vkey.deref() {
            EKeyTexture::Tex(key) => {
                let result = AssetMgr::load(&textureres_assets_mgr, &key.asset_u64());
                match result {
                    LoadResult::Ok(r) => {
                        log::warn!("image_loaded: {:?}", key.as_str());
                        image_cmd.entity(obj).insert(
                            D::from(ETextureViewUsage::Tex(r))
                        );
                    }
                    ,
                    _ => {
                        let (view_await, device, queue) = (image_await.0.clone(), (device).clone(), (queue).clone());
                        let (id, key) = (obj, key.clone());
        
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
                                        view_await.push((id, EKeyTexture::Tex(key), ETextureViewUsage::Tex(r)));
                                    }
                                    Err(e) => {
                                        log::error!("load image fail, {:?}", e);
                                    }
                                };
                            })
                            .unwrap();
                    }
                }
            },
            EKeyTexture::Image(key) => {
                if let Some(texture_view) = texture_assets_mgr.get(&key.asset_u64()) {
                    image_cmd.entity(obj).insert( D::from(ETextureViewUsage::Image(texture_view)) );
                } else {
                    let imageresult = AssetMgr::load(&image_assets_mgr, key.url());
                    match imageresult {
                        LoadResult::Ok(texture) => {
                            let texture_view = ImageTextureView::new(key, texture);
                            match texture_assets_mgr.insert(key.asset_u64(), texture_view) {
                                Ok(texture_view) => {
                                    image_cmd.entity(obj).insert( D::from(ETextureViewUsage::Image(texture_view)) );
                                },
                                Err(e) => {
                                    log::error!("image_texture_view_load fail, while insert: {:?}", key.url().as_str());
                                },
                            }
                        },
                        _ => {
                            let (image_wait, device, queue) = (image_await.1.clone(), (device).clone(), (queue).clone());
                            let (id, imagekey) = (obj, key.url().clone());
                            let key = key.clone();
                            MULTI_MEDIA_RUNTIME
                                .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                                    let desc = ImageTexture2DDesc {
                                        url: &imagekey,
                                        device: &device,
                                        queue: &queue,
                                    };
            
                                    let result = ImageTexture::async_load(desc, imageresult).await;
                                    match result {
                                        Ok(texture) => {
                                            image_wait.push((id, EKeyTexture::Image(key.clone()), texture));
                                        },
                                        Err(e) => {
                                            log::error!("load image fail, {:?}", e);
                                        }
                                    }

                                })
                                .unwrap();
                        }
                    }
                }
            },
            EKeyTexture::SRT(key) => {
                
            },
        }
    });
}

pub fn check_await_texture<
    K: std::ops::Deref<Target = EKeyTexture> + Component,
    D: From<ETextureViewUsage> + Component,
>(
    image_await: Res<ImageAwait<K>>,
    texture_assets_mgr: Res<ShareAssetMgr<ImageTextureView>>,
    mut query: Query<&K>,
    mut image_cmd: Commands,
) {
    // log::debug!("check_await_texture: ");
    // let awaits = std::mem::replace(&mut border_image_await.0, Share::new(SegQueue::new()));

    let mut res = image_await.1.pop();
    while let Some((id, key, texture)) = res {
        res = image_await.1.pop();

        match query.get_mut(id.clone()) {
            Ok(img) => {
                // image已经修改，不需要设置texture
                if **img != key {
                    continue;
                }
                let texture = match &key {
                    EKeyTexture::Tex(_) => {
                        break;
                    },
                    EKeyTexture::Image(key) => {
                        if let Some(texture_view) = texture_assets_mgr.get(&key.asset_u64()) {
                            log::warn!("image_loaded: {:?}", key.url().as_str());
                            texture_view
                        } else {
                            let texture_view = ImageTextureView::new(&key, texture);
                            if let Ok(texture_view) = texture_assets_mgr.insert(key.asset_u64(), texture_view) {
                                log::warn!("image_loaded: {:?}", key.url().as_str());
                                texture_view
                            } else {
                                log::error!("image_texture_view_load fail 2, while insert: {:?}", key.url().as_str());
                                break;
                            }
                        }
                    },
                    EKeyTexture::SRT(_) => {
                        break;
                    },
                };
                image_cmd.entity(id).insert(D::from(ETextureViewUsage::Image(texture)));
            }
            // 节点已经销毁，或image已经被删除，不需要设置texture
            _ => continue,
        };
        log::debug!("Write texture_item $$$");
    }

    let mut r = image_await.0.pop();
    while let Some((id, key, texture)) = r {
        r = image_await.0.pop();

        match query.get_mut(id.clone()) {
            Ok(img) => {
                // image已经修改，不需要设置texture
                if **img != key {
                    continue;
                }
                match key {
                    EKeyTexture::Tex(key) => log::warn!("image_loaded: {:?}", key.as_str()),
                    EKeyTexture::Image(key) => log::warn!("image_loaded: {:?}", key.url().as_str()),
                    EKeyTexture::SRT(key) => log::warn!("image_loaded: {:?}", key.to_string()),
                }
                image_cmd.entity(id).insert(D::from(texture));
            }
            // 节点已经销毁，或image已经被删除，不需要设置texture
            _ => continue,
        };
        log::debug!("Write texture_item $$$");
    }
}


// #[derive(Clone, Resource)]
// pub struct ImageTextureViewAwait<T>(Share<SegQueue<(ObjectID, EKeyTexture, Handle<ImageTextureView>)>>, PhantomData<T>);

// impl<T> Default for ImageTextureViewAwait<T> {
//     fn default() -> Self { Self(Share::new(SegQueue::new()), PhantomData) }
// }

// pub struct PluginImageTextureViewLoad<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<Handle<ImageTexture>> + Component>(PhantomData<(K, D)>);
// impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<Handle<ImageTexture>> + Component> Plugin for PluginImageLoad<K, D> {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(ImageAwait::<K>::default());
//         app.add_systems(
//             (
//                 image_change::<K, D>,
//                 check_await_texture::<K, D>
//             ).in_set(ERunStageChap::Initial)
//         );
//     }
// }
// impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<Handle<ImageTexture>> + Component> Default for PluginImageLoad<K, D> {
//     fn default() -> Self {
//         Self(PhantomData::<(K, D)>::default())
//     }
// }

// fn image_change<
//     K: std::ops::Deref<Target = EKeyTexture> + Component,
//     D: From<Handle<ImageTexture>> + Component,
// >(
//     query: Query<(ObjectID, &K), Changed<K>>,
//     mut image_cmd: Commands,
//     texture_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
//     mut image_await: ResMut<ImageAwait<K>>,
//     queue: Res<PiRenderQueue>,
//     device: Res<PiRenderDevice>,
// ) {
//     log::debug!("image_change: ");
//     query.iter().for_each(|(obj, key)| {
//         let result = AssetMgr::load(&texture_assets_mgr, key);
//         match result {
//             LoadResult::Ok(r) => {
//                 log::warn!("image_loaded: {:?}", key.as_str());
//                 image_cmd.entity(obj).insert(
//                     D::from(r)
//                 );
//             }
//             ,
//             _ => {
//                 let (image_await, device, queue) = (image_await.0.clone(), (device).clone(), (queue).clone());
//                 let (id, key) = (obj, (*key).clone());

//                 MULTI_MEDIA_RUNTIME
//                     .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
//                         let desc = ImageTexture2DDesc {
//                             url: &key,
//                             device: &device,
//                             queue: &queue,
//                         };

//                         let r = ImageTexture::async_load(desc, result).await;
//                         match r {
//                             Ok(r) => {
//                                 image_await.push((id, key.clone(), r));
//                             }
//                             Err(e) => {
//                                 log::error!("load image fail, {:?}", e);
//                             }
//                         };
//                     })
//                     .unwrap();
//             }
//         }
//     });
// }

// pub fn check_await_texture<
//     K: std::ops::Deref<Target = EKeyTexture> + Component,
//     D: From<Handle<ImageTexture>> + Component,
// >(
//     image_await: Res<ImageAwait<K>>,
//     mut query: Query<&K>,
//     mut image_cmd: Commands,
// ) {
//     // log::debug!("check_await_texture: ");
//     // let awaits = std::mem::replace(&mut border_image_await.0, Share::new(SegQueue::new()));
//     let mut r = image_await.0.pop();
//     while let Some((id, key, texture)) = r {
//         r = image_await.0.pop();

//         match query.get_mut(id.clone()) {
//             Ok(img) => {
//                 // image已经修改，不需要设置texture
//                 if **img != key {
//                     continue;
//                 }
//                 log::warn!("image_loaded: {:?}", key.as_str());
//                 image_cmd.entity(id).insert(D::from(texture));
//             }
//             // 节点已经销毁，或image已经被删除，不需要设置texture
//             _ => continue,
//         };
//         log::debug!("Write texture_item $$$");
//     }
// }
