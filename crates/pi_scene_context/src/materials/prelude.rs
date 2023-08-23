
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
    pub float: ResMut<'w, ActionListUniformFloat>,
    pub int: ResMut<'w, ActionListUniformInt>,
    pub uint: ResMut<'w, ActionListUniformUint>,
    pub vec2: ResMut<'w, ActionListUniformVec2>,
    pub vec4: ResMut<'w, ActionListUniformVec4>,
    pub mat2: ResMut<'w, ActionListUniformMat2>,
    pub mat4: ResMut<'w, ActionListUniformMat4>,
    pub texture: ResMut<'w, ActionListUniformTexture>,
    pub metas: Res<'w, ShareAssetMgr<ShaderEffectMeta>>,
    // pub metas_wait: ResMut<'w, AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>,
}
