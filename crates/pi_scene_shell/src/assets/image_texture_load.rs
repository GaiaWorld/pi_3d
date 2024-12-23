use crate::ecs::*;

// use bevy_app::{Update, App, Plugin};
// use bevy_ecs::{component::Component, entity::{self, Entity}, query::Changed, schedule::{IntoSystemConfigs, IntoSystemSetConfig, SystemSet}, system::{Query, Res, ResMut, Resource}};

use std::{marker::PhantomData, ops::Deref, sync::Arc};
use crossbeam::queue::SegQueue;
use ktx::KtxInfo;
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult, Receiver},
};
use pi_async_rt::prelude::AsyncRuntime;
use pi_hal::{image::DynamicImage, loader::AsyncLoader, runtime::RENDER_RUNTIME};
use pi_bevy_asset::ShareAssetMgr;
use pi_hash::XHashMap;
use pi_render::rhi::asset::{ImageTextureDesc, TextureRes};
use pi_share::Share;
use crate::prelude::*;

use super::{environment_texture_loader::EnvironmentTextureTools, texture::*};

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

#[derive(Clone, Copy)]
pub enum ETextureLoaderMode {
    D2,
    Env,
}

pub struct QueueInfo {
    pub id: IDImageTextureLoad,
    pub key: KeyImageTextureFrame,
    pub mode: ETextureLoaderMode,
}

#[derive(Clone, Resource)]
pub struct ImageTextureLoader {
    pub wait: Share<SegQueue<QueueInfo>>,
    pub success_load: Share<SegQueue<IDImageTextureLoad>>,
    pub fails: Share<SegQueue<IDImageTextureLoad>>,
    pub loading: XHashSet<KeyImageTextureFrame>,
    pub loading_image: Share<SegQueue<(KeyImageTextureFrame, DynamicImage, Receiver<ImageTextureFrame, GarbageEmpty>)>>,
    pub loading_data: Share<SegQueue<(KeyImageTextureFrame, Arc<Vec<u8>>, Receiver<ImageTextureFrame, GarbageEmpty>)>>,
    pub fail_reason: XHashMap<KeyImageTextureFrame, EErrorImageLoad>,
    pub fail_imgtex: Share<SegQueue<(KeyImageTextureFrame, EErrorImageLoad)>>,
    pub success: XHashMap<IDImageTextureLoad, Handle<ImageTextureFrame>>,
    pub failrecord: XHashMap<IDImageTextureLoad, EErrorImageLoad>,
    pub query_counter: IDImageTextureLoad,
}
impl MemSize for ImageTextureLoader {
    fn memsize(&self) -> usize {
        self.wait.len() * 32 + 256
        + self.success_load.len() * 8 + 256
        + self.fails.len() * 8 + 256
        + self.loading.capacity() * 8
        + self.loading_image.len() * 32 + 256
        + self.loading_data.len() * 32 + 256
        + self.fail_reason.capacity() * 16
        + self.fail_imgtex.len() * 16 + 256
        + self.success.capacity() * 16
        + self.failrecord.len() * 16
        + 16
    }
}
impl Default for ImageTextureLoader {
    fn default() -> Self {
        Self {
            wait: Share::new(SegQueue::new()),
            success_load: Share::new(SegQueue::new()),
            loading: XHashSet::default(),
            loading_image: Share::new(SegQueue::new()),
            loading_data: Share::new(SegQueue::new()),
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
    pub fn size(&self) -> usize {
        self.wait.len() * 32
        + self.success_load.len() * 8
        + self.fails.len() * 8
        + self.fail_reason.len() * 20
        + self.fail_imgtex.len() * 20
        + self.success.len() * 8
        + self.failrecord.len() * 1
    }
    pub fn create_load(&mut self, key: KeyImageTextureFrame) -> IDImageTextureLoad {
        self.query_counter += 1;
        let id = self.query_counter;
        self.wait.push(QueueInfo { id, key, mode: ETextureLoaderMode::D2 });
        id
    }
    pub fn create_load_env(&mut self, key: KeyImageTextureFrame) -> IDImageTextureLoad {
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
    pub fn query_imgtex(&self, key: &KeyImageTextureFrame, asset: &AssetMgr<ImageTextureFrame>) -> Result<Handle<ImageTextureFrame>, bool> {
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
    pub fn query_success(&mut self, id: IDImageTextureLoad) -> Option<Handle<ImageTextureFrame>> {
        self.success.remove(&id)
    }
}

pub fn sys_image_texture_load_launch(
    mut loader: ResMut<ImageTextureLoader>,
    image_assets_mgr: Res<ShareAssetMgr<ImageTextureFrame>>,
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

        if let Some(res) = image_assets_mgr.get(&param) {
            if id > 0 {
                loader.success_load.push(id);
                loader.success.insert(id, res);
            }
            continue;
        }
        let imageresult = AssetMgr::load(&image_assets_mgr, &param);
        match imageresult {
            pi_assets::mgr::LoadResult::Ok(res) => {
                if id > 0 {
                    loader.success_load.push(id);
                    loader.success.insert(id, res);
                }
            },
            _ => {
                if let Some(err) = loader.fail_reason.get(&param) {
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
                                let (loading_img, loading_data) = (loader.loading_image.clone(), loader.loading_data.clone());
                                let param = param.clone();

                                if param.cancombine {
                                    if loader.loading.contains(&param) == false {
                                        match imageresult {
                                            LoadResult::Ok(_r) => {},
                                            LoadResult::Wait(f) => {
                                                RENDER_RUNTIME.spawn(async move {
                                                    match f.await {
                                                        Ok(_result) => {},
                                                        Err(_err) => failquene.push((param.clone(), EErrorImageLoad::CacheFail))
                                                    }
                                                })
                                                .unwrap();
                                            },
                                            LoadResult::Receiver(recv) => {
                                                loader.loading.insert(param.clone());
                                                RENDER_RUNTIME.spawn(async move {
                                                    if param.compressed {
                                                        match pi_hal::file::load_from_url(&param.url).await {
                                                            Ok(data) => {
                                                                loading_data.push((param, data, recv));
                                                            },
                                                            Err(_) => failquene.push((param.clone(), EErrorImageLoad::LoadFail)),
                                                        }
                                                    } else {
                                                        match pi_hal::image::load_from_url(&param.url).await {
                                                            Ok(img) => {
                                                                loading_img.push((param, img, recv));
                                                            },
                                                            Err(_) => failquene.push((param.clone(), EErrorImageLoad::LoadFail)),
                                                        }
                                                    }
                                                })
                                                .unwrap();
                                            }
                                        }
                                    }
                                } else {
                                    RENDER_RUNTIME.spawn(async move {
                                        
                                        match imageresult {
                                            LoadResult::Ok(r) => {},
                                            LoadResult::Wait(f) => match f.await {
                                                Ok(_result) => {},
                                                Err(_err) => failquene.push((param.clone(), EErrorImageLoad::CacheFail))
                                            },
                                            LoadResult::Receiver(recv) => {
    
                                                let haldesc = pi_hal::texture::ImageTextureDesc {
                                                    url: param.url.clone(),
                                                    srgb: false,
                                                    useage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                                                };
                                                match pi_hal::image_texture_load::load_from_url(&haldesc, &device, &queue).await {
                                                    Ok(data) => {
                                                        match recv.receive(param.clone(), Ok(ImageTextureFrame::new(data))).await {
                                                            Ok(_result) => {},
                                                            Err(_) => failquene.push((param.clone(), EErrorImageLoad::CacheFail))
                                                        }
                                                    },
                                                    Err(_) => {
                                                        failquene.push((param.clone(), EErrorImageLoad::LoadFail));
                                                    },
                                                };
                                            }
                                        }
                                    })
                                    .unwrap();
                                }
                            },
                        },
                        ETextureLoaderMode::Env => 
                        {
                            if id > 0 {
                                again.push(info);
                            }
                            if param.file {
                                loader.fail_imgtex.push((param.clone(), EErrorImageLoad::LoadFail));
                            } else {
                                let (failquene, device, queue) = (loader.fail_imgtex.clone(), (device).clone(), (queue).clone());
                                let param = param.clone();
                                RENDER_RUNTIME.spawn(async move {
                                    match EnvironmentTextureTools::async_load(param.clone(), device, queue, imageresult).await {
                                        Ok(_) => {},
                                        Err(_) => {
                                            failquene.push((param.clone(), EErrorImageLoad::LoadFail))
                                        },
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
    mut combinemgr: ResMut<TextureCombineAtlas2DMgr>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    while let Some((keyimage, data, receiver)) = loader.loading_image.pop() {
        loader.loading.remove(&keyimage);
        let failquene = loader.fail_imgtex.clone();
        if let Some(texture) = combinemgr.combine_image(&keyimage, &data, &device, &queue) {
            RENDER_RUNTIME.spawn(async move {
                match receiver.receive(keyimage.clone(), Ok(texture)).await {
                    Ok(_) => {},
                    Err(_) => {
                        failquene.push((keyimage, EErrorImageLoad::LoadFail));
                    },
                }
            })
            .unwrap();
        } else if let Some(texture) = ImageTextureFrame::create_image(&device, &queue, &keyimage.url, wgpu::TextureViewDimension::D2, data) {
            RENDER_RUNTIME.spawn(async move {
                match receiver.receive(keyimage.clone(), Ok(ImageTextureFrame::new(texture))).await {
                    Ok(_) => {},
                    Err(_) => {
                        failquene.push((keyimage, EErrorImageLoad::LoadFail));
                    },
                }
            })
            .unwrap();
        } else {
            failquene.push((keyimage, EErrorImageLoad::LoadFail));
        }
    }
    while let Some((keyimage, data, receiver)) = loader.loading_data.pop() {
        loader.loading.remove(&keyimage);
        let failquene = loader.fail_imgtex.clone();
        if let Some(texture) = combinemgr.combine_ktx(&keyimage, &data, &device, &queue) {
            RENDER_RUNTIME.spawn(async move {
                match receiver.receive(keyimage.clone(), Ok(texture)).await {
                    Ok(_) => {},
                    Err(_) => {
                        failquene.push((keyimage, EErrorImageLoad::LoadFail));
                    },
                }
            })
            .unwrap();
        } else {
            let ktx = ktx::Ktx::new(data.as_slice());
            if let Some(format) = compressed_texture_format(ktx.gl_internal_format()) {
                if let Some(texture) = ImageTextureFrame::create_ktx(&device, &queue, &keyimage.url, wgpu::TextureViewDimension::D2, format, &ktx) {
                    RENDER_RUNTIME.spawn(async move {
                        match receiver.receive(keyimage.clone(), Ok(ImageTextureFrame::new(texture))).await {
                            Ok(_) => {},
                            Err(_) => {
                                failquene.push((keyimage, EErrorImageLoad::LoadFail));
                            },
                        }
                    })
                    .unwrap();
                } else {
                    loader.fail_imgtex.push((keyimage, EErrorImageLoad::LoadFail));
                }
            } else {
                loader.fail_imgtex.push((keyimage, EErrorImageLoad::LoadFail));
            }
        }
    }
    let mut item = loader.fail_imgtex .pop();
    while let Some((param, error)) = item {
        item = loader.fail_imgtex.pop();
        loader.fail_reason.insert(param, error);
        state.image_fail += 1;
    }
}

#[derive(Resource)]
pub struct ImageTextureViewLoader<K> {
    pub wait: Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    pub success: Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    pub fail: Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
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
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureViewFrame>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_loader: ResMut<ImageTextureLoader>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
    mut state: ResMut<StateTextureLoader>,
    targets: Res<CustomRenderTargets>,
    mut combinemgr: ResMut<TextureCombineAtlas2DMgr>,
) {
    items.iter_mut().for_each(|(entity, param, mut cmd)| {
        state.texview_count += 1;
        let param = param.deref();
        match _sys_image_texture_view_load_launch2(
            entity, 0, param, &imgtex_assets_mgr, &texres_assets_mgr, &mut image_loader,
            &queue, &device, &mut state, &loader.wait, &loader.success, &loader.fail, &targets
        ) {
            Some(data) => { 
                *cmd = D::from(data); 
            },
            None => {}
        }
    });
}

pub fn sys_image_texture_view_loaded_check<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(
    // entities: Query<Entity>,
    mut items: Query<(&K, &mut D)>,
    // mut commands: Commands,
    loader: Res<ImageTextureViewLoader<K>>,
    // image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureViewFrame>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_loader: ResMut<ImageTextureLoader>,
    mut state: ResMut<StateTextureLoader>,
) {
    _sys_image_texture_view_loaded_check(
        &loader.wait, &loader.success, &loader.fail,
        &imgtex_assets_mgr, &mut image_loader, &mut state
    );

    let mut item = loader.success.pop();
    while let Some((entity, _key, view, _)) = item {
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
    while let Some((entity, _key, _)) = item {
        item = loader.fail.pop();
        if let Ok((_, mut item)) = items.get_mut(entity) {
            // log::error!("Texture From Fail Queue:");
            *item = D::from(ETextureViewUsage::Tex(view.clone()));
            state.texview_success += 1;
        }
    }
}

fn _sys_image_texture_view_loaded_check(
    wait: &Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    success: &Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    fail: &Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
    imgtex_assets_mgr: &ShareAssetMgr<ImageTextureViewFrame>,
    image_loader: &mut ImageTextureLoader,
    state: &mut StateTextureLoader,
) {
    let mut item = wait.pop();
    let mut waitagain = vec![];
    while let Some((entity, key, id, _)) = item {
        item = wait.pop();

        let key_u64 = key.asset_u64();
        // let imgkey = key.url();
        if let Some(image) = image_loader.query_success(id) {
            let result = AssetMgr::load(&imgtex_assets_mgr, &key_u64);
            // log::warn!("Texture Image Success {:?}", (key.url()));
            let (success, fail) = (success.clone(), fail.clone());
            let viewkey = key.clone();
            let texkey = EKeyTexture::ImageFrame(key);
            RENDER_RUNTIME.spawn(async move {
                // log::error!("Texture Load Task {:?}", (texkey));
                match ImageTextureViewFrame::async_load(image, viewkey, result).await {
                    Ok(res) => {
                        // log::warn!("Texture Load Success {:?}", (texkey));
                        success.push((entity, texkey, ETextureViewUsage::ImageFrame(res), 0));
                    }
                    Err(_e) => {
                        // log::error!("Texture Load Fail {:?}", (texkey));
                        fail.push((entity, texkey, 0));
                    }
                };
            }).unwrap();
        } else if let Some(_fail) = image_loader.query_failed_reason(id) {
            // log::warn!("Texture Fail {:?}", (key.url(), fail));
            fail.push((entity, EKeyTexture::ImageFrame(key), 0));
            state.texview_fail += 1;
        } else {
            // log::warn!("Texture Load Again {:?}", (id, key.url()));
            waitagain.push((entity, key, id, 0));
        }
    }
    waitagain.drain(..).for_each(|item| { wait.push(item) });
    state.texview_waiting = wait.len() as u32;
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
impl MemSize for StateTextureLoader {
    fn memsize(&self) -> usize {
        8 * 4
    }
}

pub struct PluginImageTextureViewLoad<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(PhantomData<(K, D)>);
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Plugin for PluginImageTextureViewLoad<K, D> {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<ImageTextureLoader>() == false {
            app.insert_resource(ImageTextureLoader::default());
            app.insert_resource(StateTextureLoader::default());

            app.configure_set(Update, StageTextureLoad::TextureRequest  .in_set(ERunStageChap::Modify));
            app.configure_set(Update, StageTextureLoad::TextureLoading  .in_set(ERunStageChap::Modify).after(StageTextureLoad::TextureRequest));
            app.configure_set(Update, StageTextureLoad::TextureLoaded   .in_set(ERunStageChap::Modify).after(StageTextureLoad::TextureLoading));

#[cfg(feature="use_pi_ecs")]
{
    app.add_systems(
    	Update,
        (
            sys_image_texture_load_launch,
            sys_image_texture_loaded
        ).chain().in_set(StageTextureLoad::TextureLoading)
    );
}
            
#[cfg(not(feature="use_pi_ecs"))]
            {
                app.add_systems(Update, sys_image_texture_load_launch                                                    .in_set(StageTextureLoad::TextureLoading));
                app.add_systems(Update, sys_image_texture_loaded     .after(sys_image_texture_load_launch)       .in_set(StageTextureLoad::TextureLoading));
            }
        }
        app.insert_resource(ImageTextureViewLoader::<K>::default());
    }
}
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Default for PluginImageTextureViewLoad<K, D> {
    fn default() -> Self {
        Self(PhantomData::<(K, D)>::default())
    }
}


#[derive(Resource, Default)]
pub struct ImageTextureViewLoader2 {
    pub wait: Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    pub success: Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    pub fail: Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
}
impl MemSize for ImageTextureViewLoader2 {
    fn memsize(&self) -> usize {
        self.wait.len() * 64 + 256
        + self.success.len() * 64 + 256
        + self.fail.len() * 64 + 256
    }
}

pub fn sys_image_texture_view_load_launch2(
    // mut commands: Commands,
    mut items: Query<(Entity, &TextureKeyList, &mut EffectBindTexture2DList), Changed<TextureKeyList>>,
    loader: Res<ImageTextureViewLoader2>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureViewFrame>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_loader: ResMut<ImageTextureLoader>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
    mut state: ResMut<StateTextureLoader>,
    targets: Res<CustomRenderTargets>,
) {
    items.iter_mut().for_each(|(entity, param, mut cmd)| {
        state.texview_count += 1;
        cmd.empty();
        let mut idx = 0;
        param.0.iter().for_each(|key| {
            match _sys_image_texture_view_load_launch2(
                entity, idx, &key.deref().url, &imgtex_assets_mgr, &texres_assets_mgr, &mut image_loader,
                &queue, &device, &mut state, &loader.wait, &loader.success, &loader.fail, &targets
            ) {
                Some(data) => {
                    cmd.loaded_textureviewusage(idx, data, key.deref().url.clone());
                },
                None => {}
            }

            idx += 1;
        });
    });
}

fn _sys_image_texture_view_load_launch2(
    entity: Entity,
    slot: usize,
    param: &EKeyTexture,
    imgtex_assets_mgr: &ShareAssetMgr<ImageTextureViewFrame>,
    texres_assets_mgr: &ShareAssetMgr<TextureRes>,
    image_loader: &mut ImageTextureLoader,
    queue: &RenderQueue,
    device: &RenderDevice,
    state: &mut StateTextureLoader,
    wait: &Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    success: &Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    fail: &Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
    targets: &CustomRenderTargets,
) -> Option<ETextureViewUsage> {
    match param {
        EKeyTexture::Tex(url) => {
            let key_u64 = url.asset_u64();
            let result = AssetMgr::load(&texres_assets_mgr, &key_u64);
            match result {
                LoadResult::Ok(texture_view) => {
                    state.texview_success += 1;
                    Some(ETextureViewUsage::Tex(texture_view))
                },
                _ => {
                    let (success, fail, device, queue) = (success.clone(), fail.clone(), (device).clone(), (queue).clone());
                    let key = param.clone();
                    let url = url.clone();
                    RENDER_RUNTIME
                        .spawn(async move {
                            let desc = ImageTextureDesc { url: &url, device: &device, queue: &queue, };
                            match TextureRes::async_load(desc, result).await {
                                Ok(res) => {
                                    success.push((entity, key, ETextureViewUsage::Tex(res), slot));
                                }
                                Err(_e) => {
                                    fail.push((entity, key, slot));
                                }
                            };
                        })
                        .unwrap();
                    
                    None
                },
            }
        },
        EKeyTexture::Image(_key) => {
            fail.push((entity, param.clone(), slot));
            None
            // todo!()
            // // log::warn!("Texture Load {:?}", (key.url()));
            // let key_u64 = key.asset_u64();
            // let result = imgtex_assets_mgr.get(&key_u64);
            // match result {
            //     Some(view) => {
            //         // log::error!("Texture While Launch: {:?}", key_u64);
            //         // log::warn!("Texture Success 0 {:?}", (key.url()));
            //         // *cmd = D::from(ETextureViewUsage::Image(view));
            //         state.texview_success += 1;
            //         Some(ETextureViewUsage::Image(view))
            //     },
            //     _ => {
            //         // let imgkey = key.url();
            //         let id = image_loader.create_load(key.url().clone());
            //         wait.push((entity, key.clone(), id, slot));
            //         None
            //     },
            // }
        },
        EKeyTexture::SRT(_key) => {
            // TODO
            if let Some(target) = targets.get(*_key) {
                state.texview_success += 1;
                Some(ETextureViewUsage::SRT(target.rt.clone()))
            } else {
                state.texview_fail += 1;
                None
            }
        },
        EKeyTexture::ImageFrame(key) => {
            let key_u64 = key.asset_u64();
            let result = imgtex_assets_mgr.get(&key_u64);
            match result {
                Some(view) => {
                    state.texview_success += 1;
                    Some(ETextureViewUsage::ImageFrame(view))
                },
                _ => {
                    let id = image_loader.create_load(key.url().clone());
                    wait.push((entity, key.clone(), id, slot));
                    None
                },
            }
        },
    }
}

pub fn sys_image_texture_view_loaded_check2(
    mut items: Query<(&TextureKeyList, &mut EffectBindTexture2DList)>,
    loader: Res<ImageTextureViewLoader2>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureViewFrame>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_loader: ResMut<ImageTextureLoader>,
    mut state: ResMut<StateTextureLoader>,
    mut combinemgr: ResMut<TextureCombineAtlas2DMgr>,
) {
    _sys_image_texture_view_loaded_check2(
        &loader.wait, &loader.success, &loader.fail,
        &imgtex_assets_mgr, &mut image_loader, &mut state
    );

    let mut item = loader.success.pop();
    while let Some((entity, _key, view, slot)) = item {
        item = loader.success.pop();
        if let Ok((_, mut item)) = items.get_mut(entity) {
            item.loaded_textureviewusage(slot, view, _key);
            state.texview_success += 1;
        }
    }

    let whitekey = KeyTexture::from(DefaultTexture::WHITE_2D);
    // let white = EKeyTexture::Tex(whitekey.clone());
    let key_u64 = whitekey.asset_u64();
    let view = texres_assets_mgr.get(&key_u64).unwrap();
    let mut item = loader.fail.pop();
    while let Some((entity, _key, slot)) = item {
        item = loader.fail.pop();
        if let Ok((_, mut item)) = items.get_mut(entity) {
            // log::error!("Texture From Fail Queue:");
            item.loaded_textureviewusage(slot, ETextureViewUsage::Tex(view.clone()), _key.clone());
            state.texview_success += 1;
        }
    }
}

fn _sys_image_texture_view_loaded_check2(
    wait: &Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    success: &Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    fail: &Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
    imgtex_assets_mgr: &ShareAssetMgr<ImageTextureViewFrame>,
    image_loader: &mut ImageTextureLoader,
    state: &mut StateTextureLoader,
) {
    let mut item = wait.pop();
    let mut waitagain = vec![];
    while let Some((entity, key, id, slot)) = item {
        item = wait.pop();

        let key_u64 = key.asset_u64();
        // let imgkey = key.url();
        if let Some(image) = image_loader.query_success(id) {
            let result = AssetMgr::load(&imgtex_assets_mgr, &key_u64);
            // log::warn!("Texture Image Success {:?}", (key.url()));
            let (success, fail) = (success.clone(), fail.clone());
            let viewkey = key.clone();
            let texkey = EKeyTexture::ImageFrame(key);
            RENDER_RUNTIME.spawn(async move {
                // log::error!("Texture Load Task {:?}", (texkey));
                match ImageTextureViewFrame::async_load(image, viewkey, result).await {
                    Ok(res) => {
                        // log::warn!("Texture Load Success {:?}", (texkey));
                        success.push((entity, texkey, ETextureViewUsage::ImageFrame(res), slot));
                    }
                    Err(_e) => {
                        // log::error!("Texture Load Fail {:?}", (texkey));
                        fail.push((entity, texkey, slot));
                    }
                };
            }).unwrap();
        } else if let Some(_fail) = image_loader.query_failed_reason(id) {
            // log::warn!("Texture Fail {:?}", (key.url(), fail));
            fail.push((entity, EKeyTexture::ImageFrame(key), slot));
            state.texview_fail += 1;
        } else {
            // log::warn!("Texture Load Again {:?}", (id, key.url()));
            waitagain.push((entity, key, id, slot));
        }
    }
    waitagain.drain(..).for_each(|item| { wait.push(item) });
    state.texview_waiting = wait.len() as u32;
}