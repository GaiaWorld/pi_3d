
use pi_engine_shell::prelude::*;
use pi_assets::asset::Handle;

pub type ShaderEffectValueUniformDesc = MaterialValueBindDesc;

#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyShaderEffect(pub KeyShaderMeta);

#[derive(Debug, Deref, DerefMut, Component)]
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
