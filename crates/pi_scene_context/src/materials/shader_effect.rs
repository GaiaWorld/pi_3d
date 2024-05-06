
use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;

pub type ShaderEffectValueUniformDesc = MaterialValueBindDesc;

#[derive(Debug, Deref, DerefMut, Clone, Hash, )]
pub struct AssetKeyShaderEffect(pub KeyShaderMeta);

#[derive(Deref, DerefMut, )]
pub struct AssetResShaderEffectMeta(pub Handle<ShaderEffectMeta>);
impl From<Handle<ShaderEffectMeta>> for AssetResShaderEffectMeta {
    fn from(value: Handle<ShaderEffectMeta>) -> Self {
        Self(value)
    }
}
impl AssetResShaderEffectMeta {
    pub fn query_tex_slot(&self, name: &UniformPropertyName) -> Option<usize> {
        match self.0.textures.binary_search_by(|a| { a.slotname.cmp(name) }) {
            Ok(index) => Some(index),
            Err(_) => None,
        }
    }
}
