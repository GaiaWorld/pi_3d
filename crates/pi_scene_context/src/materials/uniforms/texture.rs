use std::sync::Arc;

use pi_scene_shell::prelude::*;

#[derive(Clone, Component, Default)]
pub struct UniformTextureWithSamplerParams(pub XHashMap<Atom, Arc<UniformTextureWithSamplerParam>>);

#[derive(Component, Default)]
pub struct UniformTextureWithSamplerParamsDirty;

pub trait ValueTextureKey: ThreadSync + 'static {
    fn new(param: UniformTextureWithSamplerParam) -> Self;
    fn key(&self) -> &EKeyTexture;
    fn param(&self) -> Arc<UniformTextureWithSamplerParam>;
}

pub trait UniformTexture {
    fn texture(&self) -> &TextureRes;
}
