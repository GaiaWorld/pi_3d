use std::marker::PhantomData;

use bevy_app::{Update, App, Plugin};
use bevy_ecs::{system::{Resource, ResMut, Res, Query}, schedule::{SystemSet, IntoSystemSetConfig, IntoSystemConfigs}, component::Component, entity::Entity, query::Changed};
use crossbeam::queue::SegQueue;
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult},
};
use pi_async_rt::prelude::AsyncRuntime;
use pi_hal::{runtime::RENDER_RUNTIME, loader::AsyncLoader};
use pi_bevy_asset::ShareAssetMgr;
use pi_hash::XHashMap;
use pi_render::rhi::asset::{ImageTextureDesc, TextureRes};
use pi_share::Share;
use crate::prelude::*;

use super::environment_texture_loader::EnvironmentTextureTools;

pub type IDImageTextureLoad = u64;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum ETextureLoaderMode {
    D2,
    Env,
}

pub struct QueueInfo {
    pub id: IDImageTextureLoad,
    pub key: KeyImageTexture,
    pub mode: ETextureLoaderMode,
}

#[derive(Clone, Resource)]
pub struct ImageTextureLoader {
    pub wait: Share<SegQueue<QueueInfo>>,
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
        self.wait.push(QueueInfo { id, key, mode: ETextureLoaderMode::D2 });
        id
    }
    pub fn create_load_env(&mut self, key: KeyImageTexture) -> IDImageTextureLoad {
        self.query_counter += 1;
        let id = self.query_counter;
        self.wait.push(QueueInfo { id, key, mode: ETextureLoaderMode::Env });
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
    mut state: ResMut<StateTextureLoader>,
) {
    let mut again = vec![];
    let mut item = loader.wait.pop();
    while let Some(info) = item {
        let id = info.id;
        let param = info.key.clone();
        let mode = info.mode;
        item = loader.wait.pop();

        // log::warn!("Image Load {:?}", (param.url));
        let imageresult = AssetMgr::load(&image_assets_mgr, &param);
        match imageresult {
            pi_assets::mgr::LoadResult::Ok(res) => {
                if id > 0 {
                    // log::warn!("Image Load Success {:?}", (id, param.url));
                    loader.success_load.push(id);
                    loader.success.insert(id, res);
                }
            },
            _ => {
                if let Some(err) = loader.fail_reason.get(&param) {
                    // log::warn!("Image Load fail {:?}", (param.url, err));
                    if id > 0 {
                        loader.fails.push(id);
                        let err = err.clone();
                        loader.failrecord.insert(id, err);
                        state.image_fail += 1;
                    }
                } else {
                    match mode {
                        ETextureLoaderMode::D2 => match &param.file {
                            false => loader.fail_imgtex.push((param, EErrorImageLoad::CanntLoadDataTexture)),
                            true => {
                                if id > 0 {
                                    again.push(info);
                                }
                                let (failquene, device, queue) = (loader.fail_imgtex.clone(), (device).clone(), (queue).clone());
                                let param = param.clone();
                                RENDER_RUNTIME.spawn(async move {
                                    let desc = ImageTexture2DDesc { url: param.clone(), device, queue, };
                                    match param.compressed {
                                        true => match ImageTexture::async_load_compressed(desc, imageresult).await {
                                            Ok(_) => {},
                                            Err(_) => failquene.push((param.clone(), EErrorImageLoad::LoadFail)),
                                        },
                                        false => match ImageTexture::async_load_image(desc, imageresult).await {
                                            Ok(_) => {},
                                            Err(_) => failquene.push((param.clone(), EErrorImageLoad::LoadFail)),
                                        },
                                    };
                                })
                                .unwrap();
                            },
                        },
                        ETextureLoaderMode::Env => 
                        {
                            if id > 0 {
                                again.push(info);
                            }
                            if param.file {
                                loader.fail_imgtex.push((param.clone(), EErrorImageLoad::LoadFail));
                                // loader.fails.push(id);
                                // loader.failrecord.insert(id, EErrorImageLoad::LoadFail);
                                // state.image_fail += 1;
                            } else {
                                let (failquene, device, queue) = (loader.fail_imgtex.clone(), (device).clone(), (queue).clone());
                                let param = param.clone();
                                RENDER_RUNTIME.spawn(async move {
                                    let desc = ImageTexture2DDesc { url: param.clone(), device, queue, };
                                    match EnvironmentTextureTools::async_load(desc, imageresult).await {
                                        Ok(_) => {},
                                        Err(_) => failquene.push((param.clone(), EErrorImageLoad::LoadFail)),
                                    }
                                })
                                .unwrap();
                            }
                        },
                    }
                }
            }
        }
    }

    again.drain(..).for_each(|item| { loader.wait.push(item); });
}

pub fn sys_image_texture_loaded(
    mut loader: ResMut<ImageTextureLoader>,
    mut state: ResMut<StateTextureLoader>,
) {
    let mut item = loader.fail_imgtex .pop();
    while let Some((param, error)) = item {
        item = loader.fail_imgtex.pop();
        loader.fail_reason.insert(param, error);
        state.image_fail += 1;
    }
}

#[derive(Resource)]
pub struct ImageTextureViewLoader<K> {
    pub wait: Share<SegQueue<(ObjectID, KeyImageTextureView, IDImageTextureLoad)>>,
    pub success: Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage)>>,
    pub fail: Share<SegQueue<(ObjectID, EKeyTexture)>>,
    pub _p: PhantomData<K>
}
impl<K> Default for ImageTextureViewLoader<K> {
    fn default() -> Self {
        Self { wait: Share::new(SegQueue::new()), success: Share::new(SegQueue::new()), fail: Share::new(SegQueue::new()), _p: PhantomData::default() }
    }
}

pub fn sys_image_texture_view_load_launch<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(
    // mut commands: Commands,
    mut items: Query<(Entity, &K, &mut D), Changed<K>>,
    loader: Res<ImageTextureViewLoader<K>>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureView>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_loader: ResMut<ImageTextureLoader>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
    mut state: ResMut<StateTextureLoader>,
) {
    items.iter_mut().for_each(|(entity, param, mut cmd)| {
        state.texview_count += 1;
        let param = param.deref();
        match param {
            EKeyTexture::Tex(url) => {
                let key_u64 = url.asset_u64();
                let result = AssetMgr::load(&texres_assets_mgr, &key_u64);
                match result {
                    LoadResult::Ok(texture_view) => {
                        // log::error!("Texture While Launch: {:?}", url);
                        *cmd = D::from(ETextureViewUsage::Tex(texture_view));
                        state.texview_success += 1;
                    },
                    _ => {
                        let (success, fail, device, queue) = (loader.success.clone(), loader.fail.clone(), (device).clone(), (queue).clone());
                        let key = param.clone();
                        let url = url.clone();
                        RENDER_RUNTIME
                            .spawn(async move {
                                let desc = ImageTextureDesc { url: &url, device: &device, queue: &queue, };
                                match TextureRes::async_load(desc, result).await {
                                    Ok(res) => {
                                        success.push((entity, key, ETextureViewUsage::Tex(res)));
                                    }
                                    Err(_e) => {
                                        // log::error!("load image fail, {:?}", e);
                                        // log::debug!("load image fail");
                                        fail.push((entity, key));
                                    }
                                };
                            })
                            .unwrap();
                    },
                }
            },
            EKeyTexture::Image(key) => {
                // log::warn!("Texture Load {:?}", (key.url()));
                let key_u64 = key.asset_u64();
                let result = imgtex_assets_mgr.get(&key_u64);
                match result {
                    Some(view) => {
                        // log::error!("Texture While Launch: {:?}", key_u64);
                        // log::warn!("Texture Success 0 {:?}", (key.url()));
                        *cmd = D::from(ETextureViewUsage::Image(view));
                        state.texview_success += 1;
                    },
                    _ => {
                        // let imgkey = key.url();
                        let id = image_loader.create_load(key.url().clone());
                        loader.wait.push((entity, key.clone(), id));
                    },
                }
            },
            EKeyTexture::SRT(_key) => {
                // TODO
                state.texview_fail += 1;
            },
        }
    });
}

pub fn sys_image_texture_view_loaded_check<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(
    // entities: Query<Entity>,
    mut items: Query<(&K, &mut D)>,
    // mut commands: Commands,
    loader: Res<ImageTextureViewLoader<K>>,
    // image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureView>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_loader: ResMut<ImageTextureLoader>,
    mut state: ResMut<StateTextureLoader>,
) {
    let mut item = loader.wait.pop();
    let mut waitagain = vec![];
    while let Some((entity, key, id)) = item {
        item = loader.wait.pop();

        let key_u64 = key.asset_u64();
        // let imgkey = key.url();
        if let Some(image) = image_loader.query_success(id) {
            let result = AssetMgr::load(&imgtex_assets_mgr, &key_u64);
            // log::warn!("Texture Image Success {:?}", (key.url()));
            let (success, fail) = (loader.success.clone(), loader.fail.clone());
            let viewkey = key.clone();
            let texkey = EKeyTexture::Image(key);
            RENDER_RUNTIME.spawn(async move {
                // log::error!("Texture Load Task {:?}", (texkey));
                match ImageTextureView::async_load(image, viewkey, result).await {
                    Ok(res) => {
                        // log::warn!("Texture Load Success {:?}", (texkey));
                        success.push((entity, texkey, ETextureViewUsage::Image(res)));
                    }
                    Err(_e) => {
                        // log::error!("Texture Load Fail {:?}", (texkey));
                        fail.push((entity, texkey));
                    }
                };
            }).unwrap();
        } else if let Some(_fail) = image_loader.query_failed_reason(id) {
            // log::warn!("Texture Fail {:?}", (key.url(), fail));
            loader.fail.push((entity, EKeyTexture::Image(key)));
            state.texview_fail += 1;
        } else {
            // log::warn!("Texture Load Again {:?}", (id, key.url()));
            waitagain.push((entity, key, id));
        }
    }
    waitagain.drain(..).for_each(|item| { loader.wait.push(item) });
    state.texview_waiting = loader.wait.len() as u32;

    let mut item = loader.success.pop();
    while let Some((entity, _key, view)) = item {
        item = loader.success.pop();
        // log::error!("Texture Success {:?}", _key);
        if let Ok((_, mut item)) = items.get_mut(entity) {
            // log::error!("Texture Success Component");
            // log::error!("Texture From Success Queue: {:?}", view.asset_u64());
            *item = D::from(view);
            state.texview_success += 1;
        }
    }

    let whitekey = KeyTexture::from(DefaultTexture::WHITE_2D);
    // let white = EKeyTexture::Tex(whitekey.clone());
    let key_u64 = whitekey.asset_u64();
    let view = texres_assets_mgr.get(&key_u64).unwrap();
    let mut item = loader.fail.pop();
    while let Some((entity, _key)) = item {
        item = loader.fail.pop();
        if let Ok((_, mut item)) = items.get_mut(entity) {
            // log::error!("Texture From Fail Queue:");
            *item = D::from(ETextureViewUsage::Tex(view.clone()));
            state.texview_success += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageTextureLoad {
    TextureRequest,
    TextureLoading,
    TextureLoaded,
}

#[derive(Resource, Default)]
pub struct StateTextureLoader {
    pub image_count: u32,
    pub image_success: u32,
    pub image_fail: u32,
    pub image_waiting: u32,
    pub texview_count: u32,
    pub texview_success: u32,
    pub texview_fail: u32,
    pub texview_waiting: u32,
}

pub struct PluginImageTextureViewLoad<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(PhantomData<(K, D)>);
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Plugin for PluginImageTextureViewLoad<K, D> {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<ImageTextureLoader>() == false {
            app.insert_resource(ImageTextureLoader::default());
            app.insert_resource(StateTextureLoader::default());

            app.configure_set(Update, StageTextureLoad::TextureRequest);
            app.configure_set(Update, StageTextureLoad::TextureLoading.after(StageTextureLoad::TextureRequest));
            app.configure_set(Update, StageTextureLoad::TextureLoaded.after(StageTextureLoad::TextureLoading).before(ERunStageChap::Uniform));
            app.add_systems(
				Update,
                (
                    sys_image_texture_load_launch,
                    sys_image_texture_loaded
                ).chain().in_set(StageTextureLoad::TextureLoading)
            );
        }
        app.insert_resource(ImageTextureViewLoader::<K>::default());
        // app.add_systems(
		// 	Update,
        //     (
        //         sys_image_texture_view_load_launch::<K, D>,
        //     ).chain().in_set(StageTextureLoad::TextureRequest)
        // );
        // app.add_systems(
		// 	Update,
        //     (
        //         sys_image_texture_view_loaded_check::<K, D>,
        //     ).in_set(StageTextureLoad::TextureLoaded)
        // );
    }
}
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Default for PluginImageTextureViewLoad<K, D> {
    fn default() -> Self {
        Self(PhantomData::<(K, D)>::default())
    }
}
