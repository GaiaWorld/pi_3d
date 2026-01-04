use std::sync::Arc;

use pi_scene_shell::prelude::*;

#[derive(Clone, Component, Default)]
pub struct UniformTextureWithSamplerParams(pub XHashMap<Atom, Arc<UniformTextureWithSamplerParam>>);

/// Mesh 不可见(操作隐藏、视口外), 无实例可见 则使用占位资源,将释放实际使用的资源
#[derive(Component, Default)]
pub struct UniformTextureWithSamplerParamsDirty {
    pub isplacehodler: bool
}

pub trait ValueTextureKey: ThreadSync + 'static {
    fn new(param: UniformTextureWithSamplerParam) -> Self;
    fn key(&self) -> &EKeyTexture;
    fn param(&self) -> Arc<UniformTextureWithSamplerParam>;
}

pub trait UniformTexture {
    fn texture(&self) -> &TextureRes;
}
