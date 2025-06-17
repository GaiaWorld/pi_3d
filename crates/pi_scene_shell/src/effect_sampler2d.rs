
use crate::ecs::*;

use derive_deref::Deref;

use pi_render::renderer::sampler::*;

use crate::assets::texture::TEXTURE_SLOT_COUNT;

#[derive(Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2DList(pub u32);
impl Default for EffectBindSampler2DList {
    fn default() -> Self {
        Self(0)
    }
}
impl EffectBindSampler2DList {
    pub fn custom_address(&mut self, slot: usize, val: bool) {
        let v = 1u32 << slot;
        if val {
            self.0 = self.0 | v;
        } else {
            self.0 = self.0 - (self.0 & v);
        }
    }
    pub fn is_custom_address(&self, slot: usize) -> bool {
        let v = 1u32 << slot;
        (self.0 & v) == v
    }
}
