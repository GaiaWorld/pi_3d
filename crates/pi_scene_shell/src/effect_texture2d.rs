
use crate::ecs::*;

use std::sync::Arc;
use pi_assets::asset::Handle;

use pi_render::renderer::texture::*;

use crate::{assets::texture::TEXTURE_SLOT_COUNT, shader::UniformTextureWithSamplerParam};

#[derive(Clone, Hash, PartialEq, Eq, Component, Default)]
pub struct EffectBindTexture2DList {
    pub data: [Option<(ETextureViewUsage, EKeyTexture)>; TEXTURE_SLOT_COUNT],
    pub loaded: Vec<usize>,
}
impl EffectBindTexture2DList {
    pub fn empty(&mut self) {
        self.data   = [None, None, None, None, None, None, None, None];
        self.loaded = vec![];
    }
    pub fn loaded_textureviewusage(&mut self, slot: usize, tex: ETextureViewUsage, key: EKeyTexture ) {
        match slot {
            0 => self.data[0] = Some((tex, key)),
            1 => self.data[1] = Some((tex, key)),
            2 => self.data[2] = Some((tex, key)),
            3 => self.data[3] = Some((tex, key)),
            4 => self.data[4] = Some((tex, key)),
            5 => self.data[5] = Some((tex, key)),
            6 => self.data[6] = Some((tex, key)),
            7 => self.data[7] = Some((tex, key)),
            _ => { return; }
        }
        self.loaded.push(slot);
    }
    // pub fn loaded_textureview(&mut self, slot: usize, tex: Handle<ImageTextureView>) {
    //     match slot {
    //         0 => self.data[0] = Some(ETextureViewUsage::Image(tex)),
    //         1 => self.data[1] = Some(ETextureViewUsage::Image(tex)),
    //         2 => self.data[2] = Some(ETextureViewUsage::Image(tex)),
    //         3 => self.data[3] = Some(ETextureViewUsage::Image(tex)),
    //         4 => self.data[4] = Some(ETextureViewUsage::Image(tex)),
    //         5 => self.data[5] = Some(ETextureViewUsage::Image(tex)),
    //         6 => self.data[6] = Some(ETextureViewUsage::Image(tex)),
    //         7 => self.data[7] = Some(ETextureViewUsage::Image(tex)),
    //         _ => { return; }
    //     }
    //     self.loaded.push(slot);
    // }
}
