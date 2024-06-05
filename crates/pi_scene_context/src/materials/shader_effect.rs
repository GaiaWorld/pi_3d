
use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;

pub type ShaderEffectValueUniformDesc = MaterialValueBindDesc;

#[derive(Debug, Deref, DerefMut, Clone, Hash, Component, Default)]
pub struct AssetKeyShaderEffect(pub KeyShaderMeta);

#[derive(Deref, DerefMut, Component, Default)]
pub struct AssetResShaderEffectMeta(pub Option<Handle<ShaderEffectMeta>>);
impl From<Handle<ShaderEffectMeta>> for AssetResShaderEffectMeta {
    fn from(value: Handle<ShaderEffectMeta>) -> Self {
        Self(Some(value))
    }
}
impl AssetResShaderEffectMeta {
    pub fn query_tex_slot(&self, name: &UniformPropertyName) -> Option<usize> {
        match self.0.as_ref().unwrap().textures.binary_search_by(|a| { a.slotname.cmp(name) }) {
            Ok(index) => Some(index),
            Err(_) => None,
        }
    }
}
