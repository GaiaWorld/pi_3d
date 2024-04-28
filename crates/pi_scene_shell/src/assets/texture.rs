
use std::sync::Arc;

// use bevy_ecs::component::Component;
use pi_render::renderer::texture::*;
use wgpu::{TextureView, TextureFormat};

use crate::shader::UniformTextureWithSamplerParam;

pub const TEXTURE_SLOT_COUNT: usize = 8;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ETextureSlot {
    Tex01,
    Tex02,
    Tex03,
    Tex04,
    Tex05,
    Tex06,
    Tex07,
    Tex08,
}

#[derive(Clone, Hash)]
pub struct TextureKeyList (pub [crate::Arc<UniformTextureWithSamplerParam>; TEXTURE_SLOT_COUNT]);
impl Default for TextureKeyList {
    fn default() -> Self {
        Self([
            Arc::new(UniformTextureWithSamplerParam::default()),
            Arc::new(UniformTextureWithSamplerParam::default()),
            Arc::new(UniformTextureWithSamplerParam::default()),
            Arc::new(UniformTextureWithSamplerParam::default()),
            Arc::new(UniformTextureWithSamplerParam::default()),
            Arc::new(UniformTextureWithSamplerParam::default()),
            Arc::new(UniformTextureWithSamplerParam::default()),
            Arc::new(UniformTextureWithSamplerParam::default()),
        ])
    }
}
impl TextureKeyList {
    pub fn query(&self, idx: usize) -> &crate::Arc<UniformTextureWithSamplerParam> {
        self.0.get(idx).unwrap()
    }
    pub fn modify(&mut self, idx: usize, val: crate::Arc<UniformTextureWithSamplerParam>) {
        self.0[idx] = val;
    }
}

// #[derive(Default, Clone, Component)]
// pub struct TextureResList {
//     pub data: [Option<pi_assets::asset::Handle<crate::prelude::TextureRes>>; TEXTURE_SLOT_COUNT],
//     pub loaded: Vec<usize>,
// }
// impl TextureResList {
//     pub fn empty(&mut self) {
//         self.data   = [None, None, None, None, None, None, None, None];
//         self.loaded = vec![];
//     }
//     pub fn loaded(&mut self, slot: usize, tex: pi_assets::asset::Handle<crate::prelude::TextureRes>) {
//         match slot {
//             0 => self.data[0] = Some(tex),
//             1 => self.data[1] = Some(tex),
//             2 => self.data[2] = Some(tex),
//             3 => self.data[3] = Some(tex),
//             4 => self.data[4] = Some(tex),
//             5 => self.data[5] = Some(tex),
//             6 => self.data[6] = Some(tex),
//             7 => self.data[7] = Some(tex),
//             _ => { return; }
//         }
//         self.loaded.push(slot);
//     }
// }


pub trait ITexture {
    fn view(&self) -> &TextureView;
    fn format(&self) -> TextureFormat;
    fn key(&self) -> KeyTextureViewUsage;
}