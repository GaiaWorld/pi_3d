use std::mem::size_of;

use pi_assets::asset::Asset;
use pi_bevy_asset::ShareAssetMgr;
use pi_hash::XHashMap;
use pi_render::asset::TAssetKeyU64;


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
