
use pi_engine_shell::prelude::*;
use pi_assets::asset::Handle;


// #[derive(Clone, Debug)]
// pub struct UniformPropertyMat4(pub UniformPropertyName, pub Matrix);
// impl TUnifromShaderProperty for UniformPropertyMat4 {
//     fn tag(&self) -> &UniformPropertyName {
//         &self.0
//     }
// }
// #[derive(Clone, Debug)]
// pub struct UniformPropertyMat2(pub UniformPropertyName, pub Matrix2);
// impl TUnifromShaderProperty for UniformPropertyMat2 {
//     fn tag(&self) -> &UniformPropertyName {
//         &self.0
//     }
// }
// #[derive(Clone, Debug)]
// pub struct UniformPropertyVec4(pub UniformPropertyName, pub Vector4);
// impl TUnifromShaderProperty for UniformPropertyVec4 {
//     fn tag(&self) -> &UniformPropertyName {
//         &self.0
//     }
// }
// #[derive(Clone, Debug)]
// pub struct UniformPropertyVec2(pub UniformPropertyName, pub Vector2);
// impl TUnifromShaderProperty for UniformPropertyVec2 {
//     fn tag(&self) -> &UniformPropertyName {
//         &self.0
//     }
// }
// #[derive(Clone, Debug)]
// pub struct UniformPropertyFloat(pub UniformPropertyName, pub Number);
// impl TUnifromShaderProperty for UniformPropertyFloat {
//     fn tag(&self) -> &UniformPropertyName {
//         &self.0
//     }
// }
// #[derive(Clone, Debug)]
// pub struct UniformPropertyInt(pub UniformPropertyName, pub i32);
// impl TUnifromShaderProperty for UniformPropertyInt {
//     fn tag(&self) -> &UniformPropertyName {
//         &self.0
//     }
// }
// #[derive(Clone, Debug)]
// pub struct UniformPropertyUint(pub UniformPropertyName, pub u32);
// impl TUnifromShaderProperty for UniformPropertyUint {
//     fn tag(&self) -> &UniformPropertyName {
//         &self.0
//     }
// }

// #[derive(Deref, DerefMut, Component)]
// pub struct ShaderEffectMeta(pub ShaderMeta);

// #[derive(Deref, DerefMut, Component)]
// pub struct ShaderEffectValueUniformDesc(pub MaterialValueBindDesc);
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
