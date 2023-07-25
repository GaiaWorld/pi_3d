use std::marker::PhantomData;

use crossbeam::queue::SegQueue;
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult},
};
use pi_async::prelude::AsyncRuntime;
use pi_hal::{runtime::MULTI_MEDIA_RUNTIME, loader::AsyncLoader};
use pi_bevy_asset::ShareAssetMgr;
use pi_hash::{XHashMap, XHashSet};
use pi_render::rhi::{
    asset::{ImageTextureDesc, TextureRes},
};
use pi_share::{Share, ThreadSync};
use crate::prelude::*;

#[derive(Clone, Resource)]
pub struct ImageTextureLoader {
    pub wait: Share<SegQueue<KeyImageTexture>>,
    pub success_imgtex: Share<SegQueue<(KeyImageTexture, Handle<ImageTexture>)>>,
    pub failed: Share<SegQueue<KeyImageTexture>>,
    pub fail: XHashSet<KeyImageTexture>,
    pub success: XHashMap<KeyImageTexture, Handle<ImageTexture>>,
}
impl Default for ImageTextureLoader {
    fn default() -> Self {
        Self {
            wait: Share::new(SegQueue::new()),
            success_imgtex: Share::new(SegQueue::new()),
            failed: Share::new(SegQueue::new()),
            fail: XHashSet::default(),
            success: XHashMap::default(),
        }
    }
}
impl ImageTextureLoader {
    ///
    /// 查询 Image 纹理状态, 
    /// 加载成功 返回资源引用
    /// 加载失败 返回 Err(true)
    /// 加载中 返回 Err(false)
    pub fn query_imgtex(&self, key: &KeyImageTexture, asset: &AssetMgr<ImageTexture>) -> Result<Handle<ImageTexture>, bool> {
        if let Some(res) = asset.get(key) {
            Ok(res)
        } else {
            Err(self.fail.contains(key))
        }
    }
    pub fn query_failed(&self, key: &KeyImageTexture) -> bool {
        self.fail.contains(key)
    }
    pub fn reset_failed(&mut self) {
        self.fail.clear()
    }
    pub fn load_imgtex(&self,
        key: &KeyImageTexture,
        image_assets_mgr: &ShareAssetMgr<ImageTexture>,
        queue: &RenderQueue,
        device: &RenderDevice,
    ) {
        match key {
            KeyImageTexture::Data(url, _) => {
                // log::error!("image_texture_view_load fail, Not Found DateTexture: {:?}", url);
                log::error!("image_texture_view_load fail, Not Found DateTexture:");
                self.failed.push(key.clone());
            },
            KeyImageTexture::File(url, _) => {
                let imageresult = AssetMgr::load(&image_assets_mgr, key);
                match imageresult {
                    LoadResult::Ok(res) => {
                        //
                    },
                    _ => {
                        let (success, failquene, device, queue) = (self.success_imgtex.clone(), self.failed.clone(), (device).clone(), (queue).clone());
                        let key = key.clone();
                        MULTI_MEDIA_RUNTIME
                            .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                                let desc = ImageTexture2DDesc {
                                    url: key.clone(),
                                    device: device,
                                    queue: queue,
                                };
        
                                let result = ImageTexture::async_load(desc, imageresult).await;
                                match result {
                                    Ok(_) => {
                                        //
                                    },
                                    Err(e) => {
                                        // log::error!("load image fail, {:?}", e);
                                        failquene.push(key.clone());
                                        log::error!("load image fail,");
                                    }
                                }

                            })
                            .unwrap();
                    }
                }
            },
        }
    }
}

pub fn sys_image_texture_load_launch(
    loader: Res<ImageTextureLoader>,
    image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
) {
    let mut item = loader.wait.pop();
    while let Some(param) = item {
        item = loader.wait.pop();

        if let Some(res) = image_assets_mgr.get(&param) {
            loader.success_imgtex.push((param, res));
        } else {
            match &param {
                KeyImageTexture::Data(url, srgb) => {
                    // log::error!("image_texture_view_load fail, Not Found DateTexture: {:?}", url);
                    log::error!("image_texture_view_load fail, Not Found DateTexture:");
                    loader.failed.push(param.clone());
                },
                KeyImageTexture::File(url, srgb) => {
                    let imageresult = AssetMgr::load(&image_assets_mgr, &param);
                    match imageresult {
                        LoadResult::Ok(res) => {
                            loader.success_imgtex.push((param, res));
                        },
                        _ => {
                            let (success, failquene, device, queue) = (loader.success_imgtex.clone(), loader.failed.clone(), (device).clone(), (queue).clone());
                            let param = param.clone();
                            MULTI_MEDIA_RUNTIME
                                .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                                    let desc = ImageTexture2DDesc {
                                        url: param.clone(),
                                        device: device,
                                        queue: queue,
                                    };
            
                                    let result = ImageTexture::async_load(desc, imageresult).await;
                                    match result {
                                        Ok(res) => {
                                            success.push((param, res));
                                        },
                                        Err(e) => {
                                            // log::error!("load image fail, {:?}", e);
                                            failquene.push(param.clone());
                                            log::error!("load image fail,");
                                        }
                                    }
    
                                })
                                .unwrap();
                        }
                    }
                },
            }
        }
    }
}

pub fn sys_image_texture_loaded(
    mut loader: ResMut<ImageTextureLoader>,
) {
    let mut item = loader.success_imgtex.pop();
    while let Some((param, res)) = item {
        item = loader.success_imgtex.pop();
        loader.success.insert(param, res);
    }

    let mut item = loader.failed.pop();
    while let Some(param) = item {
        item = loader.failed.pop();
        loader.fail.insert(param);
    }
}

#[derive(Resource)]
pub struct ImageTextureViewLoader<K: std::ops::Deref<Target = EKeyTexture> + Component> {
    pub wait: Share<SegQueue<(ObjectID, KeyImageTextureView)>>,
    pub success: Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage)>>,
    pub fail: Share<SegQueue<(ObjectID, EKeyTexture)>>,
    pub _p: PhantomData<K>
}
impl<K: std::ops::Deref<Target = EKeyTexture> + Component> Default for ImageTextureViewLoader<K> {
    fn default() -> Self {
        Self { wait: Share::new(SegQueue::new()), success: Share::new(SegQueue::new()), fail: Share::new(SegQueue::new()), _p: PhantomData::default() }
    }
}

pub fn sys_image_texture_view_load_launch<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(
    mut commands: Commands,
    items: Query<(Entity, &K), Changed<K>>,
    loader: Res<ImageTextureViewLoader<K>>,
    image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureView>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    image_loader: Res<ImageTextureLoader>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
) {
    items.iter().for_each(|(entity, param)| {
        let param = param.deref();
        match param {
            EKeyTexture::Tex(url) => {
                let key_u64 = url.asset_u64();
                if let Some(texture_view) = texres_assets_mgr.get(&key_u64) {
                    if let Some(mut cmd) = commands.get_entity(entity) {
                        cmd.insert( D::from(ETextureViewUsage::Tex(texture_view)) );
                    }
                } else {
                    let result = AssetMgr::load(&texres_assets_mgr, &key_u64);
                    match result {
                        LoadResult::Ok(texture_view) => {
                            if let Some(mut cmd) = commands.get_entity(entity) {
                                cmd.insert( D::from(ETextureViewUsage::Tex(texture_view)) );
                            }
                        },
                        _ => {
                            let (success, fail, device, queue) = (loader.success.clone(), loader.fail.clone(), (device).clone(), (queue).clone());
                            let key = param.clone();
                            let url = url.clone();
                            MULTI_MEDIA_RUNTIME
                                .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                                    let desc = ImageTextureDesc { url: &url, device: &device, queue: &queue, };
                                    match TextureRes::async_load(desc, result).await {
                                        Ok(res) => {
                                            success.push((entity, key, ETextureViewUsage::Tex(res)));
                                        }
                                        Err(e) => {
                                            // log::error!("load image fail, {:?}", e);
                                            log::error!("load image fail");
                                            fail.push((entity, key));
                                        }
                                    };
                                })
                                .unwrap();
                        },
                    }
                }
            },
            EKeyTexture::Image(key) => {
                let key_u64 = key.asset_u64();
                if let Some(view) = imgtex_assets_mgr.get(&key_u64) {
                    if let Some(mut cmd) = commands.get_entity(entity) {
                        cmd.insert( D::from(ETextureViewUsage::Image(view)) );
                    }
                } else {
                    let imgkey = key.url();
                    if let Some(image) = image_assets_mgr.get(imgkey) {
                        let texture_view = ImageTextureView::new(key, image);
                        if let Ok(view) = imgtex_assets_mgr.insert(key_u64, texture_view) {
                            if let Some(mut cmd) = commands.get_entity(entity) {
                                cmd.insert( D::from(ETextureViewUsage::Image(view)) );
                            }
                        } else {
                            loader.fail.push((entity, param.clone()));
                        }
                    } else {
                        image_loader.wait.push(key.url().clone());
                        loader.wait.push((entity, key.clone()));
                    }
                }
            },
            EKeyTexture::SRT(key) => {
                // TODO
            },
        }
    });
}

pub fn sys_image_texture_view_loaded_check<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(
    mut commands: Commands,
    loader: Res<ImageTextureViewLoader<K>>,
    image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureView>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    image_loader: Res<ImageTextureLoader>,
) {
    let mut item = loader.wait.pop();
    let mut waitagain = vec![];
    while let Some((entity, key)) = item {
        item = loader.wait.pop();

        let key_u64 = key.asset_u64();
        let imgkey = key.url();
        if let Some(image) = image_assets_mgr.get(imgkey) {
            let texture_view = ImageTextureView::new(&key, image);
            if let Ok(view) = imgtex_assets_mgr.insert(key_u64, texture_view) {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert( D::from(ETextureViewUsage::Image(view)) );
                }
            } else {
                loader.fail.push((entity, EKeyTexture::Image(key)));
            }
        } else if image_loader.fail.contains(&key.url()) {
            loader.fail.push((entity, EKeyTexture::Image(key)));
        } else {
            waitagain.push((entity, key));
        }
    }
    waitagain.drain(..).for_each(|item| { loader.wait.push(item) });

    let mut item = loader.success.pop();
    while let Some((entity, key, view)) = item {
        item = loader.success.pop();
        if let Some(mut cmd) = commands.get_entity(entity) {
            cmd.insert( D::from(view) );
        }
    }

    let whitekey = KeyTexture::from(DefaultTexture::WHITE_2D);
    let white = EKeyTexture::Tex(whitekey.clone());
    let key_u64 = whitekey.asset_u64();
    let view = texres_assets_mgr.get(&key_u64).unwrap();
    let mut item = loader.fail.pop();
    while let Some((entity, key)) = item {
        item = loader.fail.pop();
        if let Some(mut cmd) = commands.get_entity(entity) {
            cmd.insert( D::from(ETextureViewUsage::Tex(view.clone())) );
        }
    }
}

pub struct PluginImageTextureViewLoad<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(PhantomData<(K, D)>);
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Plugin for PluginImageTextureViewLoad<K, D> {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<ImageTextureLoader>() == false {
            app.insert_resource(ImageTextureLoader::default());
            app.add_systems(
                (
                    sys_image_texture_load_launch,
                    sys_image_texture_loaded
                ).chain().in_set(ERunStageChap::Initial)
            );
        }
        app.insert_resource(ImageTextureViewLoader::<K>::default());
        app.add_systems(
            (
                sys_image_texture_view_load_launch::<K, D>,
                sys_image_texture_view_loaded_check::<K, D>,
            ).chain().in_set(ERunStageChap::Initial)
        );
    }
}
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Default for PluginImageTextureViewLoad<K, D> {
    fn default() -> Self {
        Self(PhantomData::<(K, D)>::default())
    }
}
