use std::{mem::size_of, sync::Arc};

use pi_async_rt::prelude::AsyncRuntime;
use pi_hal::runtime::RENDER_RUNTIME;
use crossbeam::queue::SegQueue;
use pi_assets::asset::{Asset, Handle};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::PiRenderQueue;
use pi_hash::{XHashMap, XHashSet};
use pi_render::{asset::TAssetKeyU64, renderer::texture::{KeyImageTexture, ResImageTexture}};
use pi_world::single_res::{SingleRes, SingleResMut};
use pi_world_macros::Resource;


pub type KeyTextureFrameAtlas   = u64;
pub type IdxTextureFrame        = u16;
pub type IdxTextureFrameAnim    = u8;

#[derive(Clone)]
pub enum ETextureFrameRenderMode {
    EAtlasRenderBase    = 0,
    EAtlasRenderHSB     = 1,
    EAtlasRenderHS      = 2,
}

#[derive(Clone)]
pub struct TextureFrame {
    pub rotated: bool,
    pub trimmed: bool,
    pub source_size_w: u16,
    pub source_size_h: u16,
    pub sprite_source_size_x: u16,
    pub sprite_source_size_y: u16,
    pub sprite_source_size_w: u16,
    pub sprite_source_size_h: u16,
    pub frame_x: u16,
    pub frame_y: u16,
    pub frame_w: u16,
    pub frame_h: u16,
}
impl TextureFrame {
    pub fn from_data(data: &[u16]) -> Self {
        Self {
            rotated: data[0] > 0,
            trimmed: data[1] > 0,
            source_size_w: data[2],
            source_size_h: data[3],
            sprite_source_size_x: data[4],
            sprite_source_size_y: data[5],
            sprite_source_size_w: data[6],
            sprite_source_size_h: data[7],
            frame_x: data[8],
            frame_y: data[9],
            frame_w: data[10],
            frame_h: data[11],
        }
    }
}

#[derive(Clone)]
pub struct TextureFrameAtlas {
    pub _frames: XHashMap<u64, IdxTextureFrame>,
    pub _animations: XHashMap<u64, IdxTextureFrameAnim>,
    pub frames: Vec<TextureFrame>,
    pub animations: Vec<Vec<IdxTextureFrame>>,
    pub image: String,
    // pub sampler_mode: KeySampler,
    // pub alpha_mode: ModelBlend,
    // pub render_mode: ETextureFrameRenderMode,
    // pub is_invert_y: bool,
    pub scale_x: f32,
    pub scale_y: f32,
    pub width: u16,
    pub height: u16,
}
impl pi_bevy_asset::TAssetCapacity for TextureFrameAtlas {
    const ASSET_TYPE: &'static str = "TextureFrameAtlas";

    fn capacity() -> pi_bevy_asset::AssetCapacity {
        pi_bevy_asset::AssetCapacity {
            flag: true,
            min: 1024,
            max: 1,
            timeout: 10,
        }
    }
}
impl pi_assets::asset::Size for TextureFrameAtlas {
    fn size(&self) -> usize {
        self.frames.len() * size_of::<TextureFrame>() + 116
    }
}
impl Asset for TextureFrameAtlas {
    type Key = KeyTextureFrameAtlas;
}
impl TextureFrameAtlas {
    pub fn new(image: String) -> Self {
        Self {
            _frames: XHashMap::default(),
            _animations: XHashMap::default(),
            frames: vec![],
            animations: vec![],
            image: image,
            // sampler_mode: todo!(),
            // alpha_mode: todo!(),
            // render_mode: todo!(),
            // is_invert_y: todo!(),
            scale_x: 1.,
            scale_y: 1.,
            width: 1,
            height: 1,
        }
    }
    pub fn append_animation(&mut self, animname: String, data: Vec<IdxTextureFrame>) -> IdxTextureFrameAnim {
        let idx = self.animations.len();
        self._animations.insert(animname.asset_u64(), idx as IdxTextureFrameAnim);
        self.animations.push(data);
        idx as IdxTextureFrameAnim
    }
    pub fn get_animation(&self, animname: String) -> Option<&Vec<IdxTextureFrame>> {
        if let Some(idx) = self._animations.get(&animname.asset_u64()) {
            return self.animations.get(*idx as usize);
        } else {
            return None;
        }
    }
    pub fn get_animation_by_idx(&self, idx: IdxTextureFrameAnim) -> Option<&Vec<IdxTextureFrame>> {
        return self.animations.get(idx as usize);
    }
    pub fn append_frame(&mut self, frame_name: String, frame: TextureFrame) -> IdxTextureFrame {
        let frame_idx = self.frames.len();
        self._frames.insert(frame_name.asset_u64(), frame_idx as IdxTextureFrame);
        self.frames.push(frame);
        frame_idx as IdxTextureFrame
    }
    pub fn get_frame(&self, frame_name: String) -> Option<&TextureFrame> {
        match self._frames.get(&frame_name.asset_u64()) {
            Some(idx) => {
                self.frames.get(*idx as usize)
            },
            None => None,
        }
    }
    pub fn get_frame_by_idx(&self, idx: IdxTextureFrame) -> Option<&TextureFrame> {
        self.frames.get(idx as usize)
    }
    pub fn get_frame_idx(&self, frame_name: String) -> Option<IdxTextureFrame> {
        match self._frames.get(&frame_name.asset_u64()) {
            Some(idx) => {
                Some(*idx)
            },
            None => None,
        }
    }
    pub fn get_tilloffset(&self, frame_name: String) -> Option<[f32;4]> {
        match self._frames.get(&frame_name.asset_u64()) {
            Some(idx) => {
                match self.frames.get(*idx as usize) {
                    Some(frame) => Some([
                        frame.frame_w as f32 / self.width   as f32,
                        frame.frame_h as f32 / self.height  as f32,
                        frame.frame_x as f32 / self.width   as f32,
                        frame.frame_y as f32 / self.height  as f32
                    ]),
                    None => None,
                }
            },
            None => None,
        }
    }
}

pub type TextureFrameAtlasManager = ShareAssetMgr<TextureFrameAtlas>;


#[derive(Resource, Default)]
pub struct TextureCombineCmds {
    pub loaded_quene: Arc<SegQueue<(Atom, Atom, Arc<Vec<u8>>)>>,
    pub loaded_quene2: Arc<SegQueue<(Atom, Atom, pi_hal::image::DynamicImage)>>,
    pub failed_quene: Arc<SegQueue<(Atom, Atom, u16, u32)>>,
    pub cmds: XHashMap<Atom, XHashMap<Atom, (u32, u16, bool, u32, u32, u32, u32)>>,
    pub records: XHashMap<Atom, XHashMap<Atom, (u32, u16, bool, u32, u32, u32, u32)>>,
    pub textures: XHashMap<Atom, Handle<ResImageTexture>>,
    pub check_loaded: XHashMap<u32, i32>,
    pub success: XHashSet<u32>,
    pub faileds: XHashSet<u32>,
}
impl TextureCombineCmds {
    pub fn request(&mut self, requestid: u32, keytex: KeyImageTexture, atlas: XHashMap<Atom, (u32, u16, bool, u32, u32, u32, u32)>, assets: &ShareAssetMgr<ResImageTexture>) {

        if let Some(texture) = assets.get(&keytex) {
            let key = keytex.url;
            self.check_loaded.insert(requestid, atlas.len() as i32);
            self.cmds.insert(key.clone(), atlas);
            self.textures.insert(key, texture);
        } else {
            self.faileds.insert(requestid);
        }
    }
    pub fn remove(&mut self, requestid: u32, key: Atom) {
        self.check_loaded.remove(&requestid);
        self.cmds.remove(&key);
        self.records.remove(&key);
        self.textures.remove(&key);
    }
    pub fn successed(&mut self) -> std::collections::hash_set::Drain<u32> {
        self.success.drain()
    }
    pub fn failed(&mut self) -> std::collections::hash_set::Drain<u32> {
        self.faileds.drain()
    }
}
pub fn sys_texture_combine(
    mut cmds: SingleResMut<TextureCombineCmds>,
    queue: SingleRes<PiRenderQueue>,
) {
    let mut requests = vec![];
    cmds.cmds.drain().for_each(|(key, cmd)| {
        requests.push((key, cmd));
    });
    requests.drain(..).for_each(|(key, cmd)| {
        cmd.iter().for_each(|(file, (requestid, idx, iscompress, _, _, _, _))| {
            let idx = *idx;
            let requestid = *requestid;
            let iscompress = *iscompress;
            let key = key.clone();
            let file = file.clone();
            let loaded = cmds.loaded_quene.clone();
            let failed = cmds.failed_quene.clone();
            let loaded2 = cmds.loaded_quene2.clone();
            RENDER_RUNTIME
            .spawn(async move {
                if iscompress {
                    match pi_hal::file::load_from_url(&file).await {
                        Ok(res) => {
                            loaded.push((key, file, res));
                        }
                        Err(_e) => {
                            failed.push((key, file, idx, requestid));
                        }
                    }
                } else {
                    match pi_hal::image::load_from_url(&file).await {
                        Ok(res) => {
                            loaded2.push((key, file, res));
                        }
                        Err(_e) => {
                            failed.push((key, file, idx, requestid));
                        }
                    }
                }
            })
            .unwrap();
        });
        cmds.records.insert(key, cmd);
    });

    let mut success = vec![];
    while let Some((key, file, data)) = cmds.loaded_quene.pop() {
        if let Some(atlas) = cmds.records.get(&key) {
            if let Some((requestid, _, _, xoffset, yoffset, width, height)) = atlas.get(&file) {
                let dataoffset = 0;
                let depth_or_array_layers = 1;
                let aspect = None;
                if let Some(tex) = cmds.textures.get(&key) {
                    let ktx = ktx::Ktx::new(data.as_slice());
                    for data in ktx.textures() {
                        tex.update(&queue, *xoffset, *yoffset, *width, *height, depth_or_array_layers, aspect, data, dataoffset);
                    }
                    let requestid = *requestid;
                    if let Some(check) = cmds.check_loaded.get_mut(&requestid) {
                        *check -= 1;
                        if *check <= 0 {
                            success.push(requestid);
                        }
                    }
                }
            }
        }
    }
    
    while let Some((key, file, data)) = cmds.loaded_quene2.pop() {
        if let Some(atlas) = cmds.records.get(&key) {
            if let Some((requestid, _, _, xoffset, yoffset, width, height)) = atlas.get(&file) {
                let dataoffset = 0;
                let depth_or_array_layers = 1;
                let aspect = None;
                if let Some(tex) = cmds.textures.get(&key) {
                    match &data {
                        pi_hal::image::DynamicImage::ImageLuma8(image_buffer) => {
                            tex.update(&queue, *xoffset, *yoffset, *width, *height, depth_or_array_layers, aspect, &image_buffer.as_raw(), dataoffset);
                        },
                        pi_hal::image::DynamicImage::ImageLumaA8(image_buffer) => {
                            tex.update(&queue, *xoffset, *yoffset, *width, *height, depth_or_array_layers, aspect, &image_buffer.as_raw(), dataoffset);
                        },
                        pi_hal::image::DynamicImage::ImageRgb8(image_buffer) => {
                            tex.update(&queue, *xoffset, *yoffset, *width, *height, depth_or_array_layers, aspect, &data.to_rgba8(), dataoffset);
                        },
                        pi_hal::image::DynamicImage::ImageRgba8(image_buffer) => {
                            tex.update(&queue, *xoffset, *yoffset, *width, *height, depth_or_array_layers, aspect, &image_buffer.as_raw(), dataoffset);
                        },
                        pi_hal::image::DynamicImage::ImageLuma16(image_buffer) => todo!(),
                        pi_hal::image::DynamicImage::ImageLumaA16(image_buffer) => todo!(),
                        pi_hal::image::DynamicImage::ImageRgb16(image_buffer) => todo!(),
                        pi_hal::image::DynamicImage::ImageRgba16(image_buffer) => todo!(),
                        pi_hal::image::DynamicImage::ImageRgb32F(image_buffer) => todo!(),
                        pi_hal::image::DynamicImage::ImageRgba32F(image_buffer) => todo!(),
                        _ => todo!(),
                    }
                    let requestid = *requestid;
                    if let Some(check) = cmds.check_loaded.get_mut(&requestid) {
                        *check -= 1;
                        if *check <= 0 {
                            success.push(requestid);
                        }
                    }
                }
            }
        }
    }
    success.drain(..).for_each(|requestid| {
        cmds.success.insert(requestid);
        cmds.check_loaded.remove(&requestid);
    });
    while let Some((key, file, idx, requestid)) = cmds.failed_quene.pop() {
        cmds.faileds.insert(requestid);
        cmds.check_loaded.remove(&requestid);
    }
}