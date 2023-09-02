
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageMaterial {
    MaterialUse,
    MaterialUseApply,
    MaterialCommand,
    MaterialCommandApply,
    MaterialReady,
}

#[derive(Resource, Default)]
pub struct StateMaterial {
    pub count: u32,
    pub count_ready: u32,
    pub count_tex0: u32,
    pub count_tex0_ready: u32,
    pub count_tex1: u32,
    pub count_tex1_ready: u32,
    pub count_tex2: u32,
    pub count_tex2_ready: u32,
    pub count_tex3: u32,
    pub count_tex3_ready: u32,
    pub count_tex4: u32,
    pub count_tex4_ready: u32,
    pub count_tex5: u32,
    pub count_tex5_ready: u32,
    pub count_tex6: u32,
    pub count_tex6_ready: u32,
}

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

pub type StateMaterialQuery = QueryState<(&'static AssetResShaderEffectMeta, &'static EffectTextureSamplersComp)>;

pub fn sys_state_material(
    mut state: ResMut<StateMaterial>,
    materials: Query<(&AssetResShaderEffectMeta, &EffectTextureSamplersComp)>,
) {
    state.count = 0;
    state.count_ready = 0;

    materials.iter().for_each(|(meta, texs)| {
        state.count += 1;
        if let Some(texs) = &texs.0 {
            if texs.binding_count == meta.textures.len() as u32 * 2 {
                state.count_ready += 1;
            }
        } else if meta.textures.len() == 0 {
            state.count_ready += 1;
        }
    });
}
