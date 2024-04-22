use std::sync::Arc;

use derive_deref::{DerefMut, Deref};
use pi_scene_shell::prelude::*;

use crate::materials::value::{UniformBind, SlotActiveRequire};

#[derive(Clone, Copy)]
pub enum ETextureSlot {
    Slot0,
    Slot1,
    Slot2,
    Slot3,
}

#[derive(Clone, Default, Component)]
pub struct UniformTextureWithSamplerParams(pub XHashMap<Atom, Arc<UniformTextureWithSamplerParam>>);

#[derive(Component)]
pub struct UniformTextureWithSamplerParamsDirty;

pub trait ValueTextureKey: ThreadSync + 'static {
    fn new(param: UniformTextureWithSamplerParam) -> Self;
    fn key(&self) -> &EKeyTexture;
    fn param(&self) -> Arc<UniformTextureWithSamplerParam>;
}

pub trait UniformTexture {
    fn texture(&self) -> &TextureRes;
}
