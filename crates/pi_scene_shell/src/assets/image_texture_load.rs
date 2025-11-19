use crate::ecs::*;

use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use crossbeam::queue::SegQueue;
use pi_assets::{
    mgr::{AssetMgr, LoadResult},
};
use pi_async_rt::prelude::AsyncRuntime;
use pi_bevy_render_plugin::asimage_url::{RenderTarget, load_from_asimage_url};
pub use pi_bevy_render_plugin::{ResStateTextureLoader, ResTextureCombineAtlas2DMgr};
use pi_hal::{loader::AsyncLoader, runtime::RENDER_RUNTIME};
use pi_bevy_asset::ShareAssetMgr;
use pi_render::{renderer::texture_loader::{environment_texture_loader::EnvironmentTextureTools, loader::ImageTextureLoader}, rhi::asset::{ImageTextureDesc, TextureRes}};
use pi_share::Share;
use crate::prelude::*;

use super::{texture::*};


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

pub type IDImageTextureLoad = u64;
#[derive(Resource, Default)]
pub struct ResImageTextureLoader {
    pub loader: ImageTextureLoader<IDImageTextureLoad>,
    pub query_counter: IDImageTextureLoad,
    pub wait: Share<SegQueue<QueueInfo>>,
    pub success: XHashMap<IDImageTextureLoad, Handle<ImageTextureFrame>>,
    pub fail_reason: XHashMap<KeyImageTextureFrame, EError>,
    pub failrecord: XHashMap<IDImageTextureLoad, EError>,
}
impl DerefMut for ResImageTextureLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.loader
    }
}
impl Deref for ResImageTextureLoader {
    type Target = ImageTextureLoader<u64>;

    fn deref(&self) -> &Self::Target {
        &self.loader
    }
    
}
impl ResImageTextureLoader {
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
    pub fn query_failed_reason(&mut self, id: IDImageTextureLoad) -> Option<EError> {
        if let Some(key) = self.failrecord.remove(&id) {
            Some(key)
        } else {
            None
        }
    }
    pub fn query_success(&mut self, id: IDImageTextureLoad) -> Option<Handle<ImageTextureFrame>> {
        self.success.remove(&id)
    }
}

impl MemSize for ResImageTextureLoader {
    fn memsize(&self) -> usize {
        self.wait.len() * 32 + 256
        + self.success.len() * 8 + 256
        + self.fail_reason.len() * 8 + 256
        + self.loading_image.len() * 32 + 256
        + self.loading_data.len() * 32 + 256
        + self.fail_reason.capacity() * 16
        + self.success.capacity() * 16
        + self.failrecord.len() * 16
        + 16
    }
}

pub fn sys_image_texture_load_launch(
    mut loader: ResMut<ResImageTextureLoader>,
    image_assets_mgr: Res<ShareAssetMgr<ImageTextureFrame>>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
    mut state: ResMut<ResStateTextureLoader>,
    mut combinemgr: ResMut<ResTextureCombineAtlas2DMgr>,
) {
    while let Some(item) = loader.wait.pop() {
        match item.mode {
            ETextureLoaderMode::D2 => {
                if let Some(tex) = loader.loader.async_load(
                    item.id,
                    item.key, wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    &device, &queue, &image_assets_mgr
                ) {
                    loader.success.insert(item.id, tex);
                }
            },
            ETextureLoaderMode::Env => {
                let imageresult = AssetMgr::load(&image_assets_mgr, &item.key);
                match imageresult {
                    pi_assets::mgr::LoadResult::Ok(res) => {
                        loader.success.insert(item.id, res);
                    },
                    _ => {
                        if item.key.file {
                            loader.fail_reason.insert(item.key.clone(), ErrorRecord::ERROR_TEXTURE_LOAD_FAIL);
                            loader.failrecord.insert(item.id, ErrorRecord::ERROR_TEXTURE_LOAD_FAIL);
                        } else {
                            let (success, failquene, device, queue) = (loader.loader.success.clone(), loader.failquene.clone(), (device).clone(), (queue).clone());
                            let param = item.key.clone();
                            let id = item.id;
                            RENDER_RUNTIME.spawn(async move {
                                match EnvironmentTextureTools::async_load(param.clone(), device, queue, imageresult).await {
                                    Ok(data) => { success.push((id, param, data)) },
                                    Err(_) => failquene.push((id, param, ErrorRecord::ERROR_TEXTURE_LOAD_FAIL)),
                                }
                            })
                            .unwrap();
                        }
                    }
                }
            },
        }
    }
    loader.loader.check_combine(&device, &queue, &mut combinemgr);
    while let Some(item) = loader.loader.success.pop() {
        loader.success.insert(item.0, item.2);
    }
    while let Some(item) = loader.loader.failquene.pop() {
        loader.fail_reason.insert(item.1, item.2);
        loader.failrecord.insert(item.0, item.2);
    }
}

pub fn sys_image_texture_loaded(
    // mut loader: ResMut<ResImageTextureLoader>,
    // mut state: ResMut<ResStateTextureLoader>,
    // mut combinemgr: ResMut<ResTextureCombineAtlas2DMgr>,
    // device: Res<PiRenderDevice>,
    // queue: Res<PiRenderQueue>,
) {
    // image_texture_loaded(&mut loader, &mut state, &mut combinemgr, &queue, &device);
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
    mut image_loader: ResMut<ResImageTextureLoader>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
    mut state: ResMut<ResStateTextureLoader>,
    targets: Res<CustomRenderTargets>,
    asimage: Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)>
    // mut combinemgr: ResMut<ResTextureCombineAtlas2DMgr>,
) {
    items.iter_mut().for_each(|(entity, param, mut cmd)| {
        state.texview_count += 1;
        let param = param.deref();
        match _sys_image_texture_view_load_launch2(
            entity, 0, param, &imgtex_assets_mgr, &texres_assets_mgr, &mut image_loader,
            &queue, &device, &mut state, &loader.wait, &loader.success, &loader.fail, &targets, &asimage
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
    mut image_loader: ResMut<ResImageTextureLoader>,
    mut state: ResMut<ResStateTextureLoader>,
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
    image_loader: &mut ResImageTextureLoader,
    state: &mut ResStateTextureLoader,
) {
    let mut item = wait.pop();
    let mut waitagain = vec![];
    let mut idcounter = 0;
    while let Some((entity, key, id, _)) = item {
        idcounter = idcounter + 1;
        if idcounter >= 1024 {
            log::error!("sys_image_texture_loaded");
        }
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

impl MemSize for ResStateTextureLoader {
    fn memsize(&self) -> usize {
        8 * 4
    }
}

pub struct PluginImageTextureViewLoad<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(PhantomData<(K, D)>);
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Plugin for PluginImageTextureViewLoad<K, D> {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<ResImageTextureLoader>() == false {
            app.insert_resource(ResImageTextureLoader::default());
            app.insert_resource(ResStateTextureLoader::default());

            app.configure_set(StageD3, StageTextureLoad::TextureRequest  .in_set(ERunStageChap::Modify));
            app.configure_set(StageD3, StageTextureLoad::TextureLoading  .in_set(ERunStageChap::Modify).after(StageTextureLoad::TextureRequest));
            app.configure_set(StageD3, StageTextureLoad::TextureLoaded   .in_set(ERunStageChap::Modify).after(StageTextureLoad::TextureLoading));

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
                app.add_systems(StageD3, sys_image_texture_load_launch                                                    .in_set(StageTextureLoad::TextureLoading));
                app.add_systems(StageD3, sys_image_texture_loaded     .after(sys_image_texture_load_launch)       .in_set(StageTextureLoad::TextureLoading));
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
    mut image_loader: ResMut<ResImageTextureLoader>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
    mut state: ResMut<ResStateTextureLoader>,
    targets: Res<CustomRenderTargets>,
    asimage: Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)>
) {
    items.iter_mut().for_each(|(entity, param, mut cmd)| {
        state.texview_count += 1;
        cmd.empty();
        let mut idx = 0;
        param.0.iter().for_each(|key| {
            match _sys_image_texture_view_load_launch2(
                entity, idx, &key.deref().url, &imgtex_assets_mgr, &texres_assets_mgr, &mut image_loader,
                &queue, &device, &mut state, &loader.wait, &loader.success, &loader.fail, &targets, &asimage
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
    image_loader: &mut ResImageTextureLoader,
    queue: &RenderQueue,
    device: &RenderDevice,
    state: &mut ResStateTextureLoader,
    wait: &Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    success: &Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    fail: &Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
    targets: &CustomRenderTargets,
    asimage: &Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)>,
) -> Option<ETextureViewUsage> {
    match param {
        EKeyTexture::Tex(url) => {
            if url.starts_with("asimage:://") {
                let key = param.clone();
                match load_from_asimage_url(url, asimage) {
                    Ok(rt) => match rt {
                        Some(rt) => {
                            state.texview_success += 1;
                            Some(ETextureViewUsage::from(&rt.0))
                        },
                        None => {
                            fail.push((entity, key, slot)); None
                        },
                    },
                    Err(_) => {
                        fail.push((entity, key, slot));
                        None
                    }
                }
            } else {
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
                // log::error!(">>> Use SRT {:?}", (target.rt.target_index(), _key));
                Some(ETextureViewUsage::from(&target.rt))
            } else {
                // log::error!("EKeyTexture::SRT Fail");
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
    mut image_loader: ResMut<ResImageTextureLoader>,
    mut state: ResMut<ResStateTextureLoader>,
    // mut combinemgr: ResMut<ResTextureCombineAtlas2DMgr>,
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
    image_loader: &mut ResImageTextureLoader,
    state: &mut ResStateTextureLoader,
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