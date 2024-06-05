
use std::sync::Arc;

use bevy_ecs::component::Component;
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

#[derive(Clone, Hash, Component)]
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


pub trait ITexture {
    fn view(&self) -> &TextureView;
    fn format(&self) -> TextureFormat;
    fn key(&self) -> KeyTextureViewUsage;
}