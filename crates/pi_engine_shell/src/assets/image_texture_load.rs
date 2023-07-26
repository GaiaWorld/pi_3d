use std::marker::PhantomData;

use crossbeam::queue::SegQueue;
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult},
};
use pi_async_rt::prelude::AsyncRuntime;
use pi_hal::{runtime::MULTI_MEDIA_RUNTIME, loader::AsyncLoader};
use pi_bevy_asset::ShareAssetMgr;
use pi_hash::{XHashMap, XHashSet};
use pi_render::rhi::{
    asset::{ImageTextureDesc, TextureRes},
};
use pi_share::{Share, ThreadSync};
use crate::prelude::*;

pub type IDImageTextureLoad = u64;

#[derive(Clone, Copy)]
pub enum EErrorImageLoad {
    LoadFail,
    CacheFail,
    CanntLoadDataTexture,
}
impl ToString for EErrorImageLoad {
    fn to_string(&self) -> String {
        match self {
            Self::LoadFail => String::from("LoadFail, "),
            Self::CacheFail => String::from("CacheFail, "),
            Self::CanntLoadDataTexture => String::from("CanntLoadDataTexture, "),
        }
    }
} 

#[derive(Clone, Resource)]
pub struct ImageTextureLoader {
    pub wait: Share<SegQueue<(IDImageTextureLoad, KeyImageTexture)>>,
    pub success_load: Share<SegQueue<IDImageTextureLoad>>,
    pub fails: Share<SegQueue<IDImageTextureLoad>>,
    pub fail_reason: XHashMap<KeyImageTexture, EErrorImageLoad>,
    pub fail_imgtex: Share<SegQueue<(KeyImageTexture, EErrorImageLoad)>>,
    pub success: XHashMap<IDImageTextureLoad, Handle<ImageTexture>>,
    pub failrecord: XHashMap<IDImageTextureLoad, EErrorImageLoad>,
    pub query_counter: IDImageTextureLoad,
}
impl Default for ImageTextureLoader {
    fn default() -> Self {
        Self {
            wait: Share::new(SegQueue::new()),
            success_load: Share::new(SegQueue::new()),
            fails: Share::new(SegQueue::new()),
            fail_reason: XHashMap::default(),
            fail_imgtex: Share::new(SegQueue::new()),
            success: XHashMap::default(),
            failrecord: XHashMap::default(),
            query_counter: 0,
        }
    }
}
impl ImageTextureLoader {
    pub fn create_load(&mut self, key: KeyImageTexture) -> IDImageTextureLoad {
        self.query_counter += 1;
        let id = self.query_counter;

        self.wait.push((id, key));

        id
    }
    ///
    /// 查询 Image 纹理状态, 
    /// 加载成功 返回资源引用
    /// 加载失败 返回 Err(true)
    /// 加载中 返回 Err(false)
    pub fn query_imgtex(&self, key: &KeyImageTexture, asset: &AssetMgr<ImageTexture>) -> Result<Handle<ImageTexture>, bool> {
        if let Some(res) = asset.get(key) {
            Ok(res)
        } else {
            Err(self.fail_reason.contains_key(key))
        }
    }
    pub fn query_failed_reason(&mut self, id: IDImageTextureLoad) -> Option<String> {
        if let Some(key) = self.failrecord.remove(&id) {
            Some(key.to_string())
        } else {
            None
        }
    }
    pub fn query_success(&mut self, id: IDImageTextureLoad) -> Option<Handle<ImageTexture>> {
        self.success.remove(&id)
    }
}

pub fn sys_image_texture_load_launch(
    mut loader: ResMut<ImageTextureLoader>,
    image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
) {
    let mut again = vec![];
    let mut item = loader.wait.pop();
    while let Some((id, param)) = item {
        item = loader.wait.pop();

        if let Some(res) = image_assets_mgr.get(&param) {
            if id > 0 {
                loader.success_load.push(id);
                loader.success.insert(id, res);
            }
        } else if let Some(err) = loader.fail_reason.get(&param) {
            if id > 0 {
                loader.fails.push(id);
                let err = err.clone();
                loader.failrecord.insert(id, err);
            }
        } else {
            if id > 0 {
                again.push((id, param.clone()));
            }
            match &param {
                KeyImageTexture::Data(url, srgb) => {
                    // log::error!("image_texture_view_load fail, Not Found DateTexture: {:?}", url);
                    // log::error!("image_texture_view_load fail, Not Found DateTexture:");
                    loader.fail_imgtex.push((param, EErrorImageLoad::CanntLoadDataTexture));
                },
                KeyImageTexture::File(url, srgb) => {
                    let imageresult = AssetMgr::load(&image_assets_mgr, &param);
                    match imageresult {
                        LoadResult::Ok(res) => {
                            // loader.success_imgtex.push((param, res));
                        },
                        _ => {
                            let (failquene, device, queue) = (loader.fail_imgtex.clone(), (device).clone(), (queue).clone());
                            let param = param.clone();
                            MULTI_MEDIA_RUNTIME
                                .spawn(async move {
                                    let desc = ImageTexture2DDesc {
                                        url: param.clone(),
                                        device: device,
                                        queue: queue,
                                    };
            
                                    let result = ImageTexture::async_load(desc, imageresult).await;
                                    match result {
                                        Ok(res) => {
                                            // success.push((param, res));
                                        },
                                        Err(e) => {
                                            // log::error!("load image fail, {:?}", e);
                                            failquene.push((param.clone(), EErrorImageLoad::LoadFail));
                                            // log::error!("load image fail,");
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

    again.drain(..).for_each(|item| { loader.wait.push(item); });
}

pub fn sys_image_texture_loaded(
    mut loader: ResMut<ImageTextureLoader>,
) {
    let mut item = loader.fail_imgtex .pop();
    while let Some((param, error)) = item {
        item = loader.fail_imgtex.pop();
        loader.fail_reason.insert(param, error);
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
                                .spawn(async move {
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
                        image_loader.wait.push((0, key.url().clone()));
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
        } else if image_loader.fail_reason.contains_key(&key.url()) {
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
