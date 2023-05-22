
use pi_engine_shell::prelude::*;

pub use super::material::*;
pub use super::shader_effect::*;
pub use super::command::*;
pub use super::uniforms::{
    uniform::*,
    float::*,
    int::*,
    uint::*,
    vec2::*,
    vec4::*,
    mat2::*,
    mat4::*,
    texture::*,
    texture_uniform::*,
    value_uniform::*,
    sys_uniform::*,
};


#[derive(SystemParam)]
pub struct ActionSetMaterial<'w> {
    pub usemat: ResMut<'w, ActionListMaterialUse>,
    pub create: ResMut<'w, ActionListMaterialCreate>,
    pub uniform: ResMut<'w, ActionListUniformByName>,
    pub metas: Res<'w, ShareAssetMgr<ShaderEffectMeta>>,
    pub metas_wait: ResMut<'w, AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>,
}
