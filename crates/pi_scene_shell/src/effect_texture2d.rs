
use bevy_ecs::prelude::Component;
use pi_assets::asset::Handle;

use pi_render::renderer::texture::*;

use crate::assets::texture::TEXTURE_SLOT_COUNT;

#[derive(Default, Clone, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2DList {
    pub data: [Option<ETextureViewUsage>; TEXTURE_SLOT_COUNT],
    pub loaded: Vec<usize>,
}
impl EffectBindTexture2DList {
    pub fn empty(&mut self) {
        self.data   = [None, None, None, None, None, None, None, None];
        self.loaded = vec![];
    }
    pub fn loaded_textureviewusage(&mut self, slot: usize, tex: ETextureViewUsage) {
        match slot {
            0 => self.data[0] = Some(tex),
            1 => self.data[1] = Some(tex),
            2 => self.data[2] = Some(tex),
            3 => self.data[3] = Some(tex),
            4 => self.data[4] = Some(tex),
            5 => self.data[5] = Some(tex),
            6 => self.data[6] = Some(tex),
            7 => self.data[7] = Some(tex),
            _ => { return; }
        }
        self.loaded.push(slot);
    }
    pub fn loaded_textureview(&mut self, slot: usize, tex: Handle<ImageTextureView>) {
        match slot {
            0 => self.data[0] = Some(ETextureViewUsage::Image(tex)),
            1 => self.data[1] = Some(ETextureViewUsage::Image(tex)),
            2 => self.data[2] = Some(ETextureViewUsage::Image(tex)),
            3 => self.data[3] = Some(ETextureViewUsage::Image(tex)),
            4 => self.data[4] = Some(ETextureViewUsage::Image(tex)),
            5 => self.data[5] = Some(ETextureViewUsage::Image(tex)),
            6 => self.data[6] = Some(ETextureViewUsage::Image(tex)),
            7 => self.data[7] = Some(ETextureViewUsage::Image(tex)),
            _ => { return; }
        }
        self.loaded.push(slot);
    }
}
