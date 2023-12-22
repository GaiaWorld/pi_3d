
use pi_engine_shell::prelude::*;

pub use super::material::*;
pub use super::shader_effect::*;
pub use super::command::*;
pub use super::uniforms::{
    uniform::*,
    float::*,
    // int::*,
    uint::*,
    vec2::*,
    vec4::*,
    // mat2::*,
    mat4::*,
    texture::*,
    // sys_uniform::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageMaterial {
    MaterialCommand,
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
    // pub int: ResMut<'w, ActionListUniformInt>,
    pub uint: ResMut<'w, ActionListUniformUint>,
    pub vec2: ResMut<'w, ActionListUniformVec2>,
    pub vec3: ResMut<'w, ActionListUniformVec3>,
    pub vec4: ResMut<'w, ActionListUniformVec4>,
    // pub mat2: ResMut<'w, ActionListUniformMat2>,
    pub mat4: ResMut<'w, ActionListUniformMat4>,
    pub texture: ResMut<'w, ActionListUniformTexture>,
    pub texturefromtarget: ResMut<'w, ActionListUniformTextureFromRenderTarget>,
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
            if texs.textures.len() == meta.textures.len() {
                state.count_ready += 1;
            }
        } else if meta.textures.len() == 0 {
            state.count_ready += 1;
        }
    });
}

///
pub struct BundleMaterial (
    BindEffect,
    UniformAnimated,
    TargetAnimatorableIsRunning,
    AssetKeyShaderEffect,
    // MaterialRefs,
    BindEffectReset,
    // UniformTextureWithSamplerParams,
    UniformTextureWithSamplerParamsDirty,
    FlagAnimationStartResetComp,
    DirtyMaterialRefs,
    TextureSlot01,
    TextureSlot02,
    TextureSlot03,
    TextureSlot04,
    TextureSlot05,
    TextureSlot06,
    TextureSlot07,
    TextureSlot08,
    EffectBindTexture2D01Comp,
    EffectBindTexture2D02Comp,
    EffectBindTexture2D03Comp,
    EffectBindTexture2D04Comp,
    EffectBindTexture2D05Comp,
    EffectBindTexture2D06Comp,
    EffectBindTexture2D07Comp,
    EffectBindTexture2D08Comp,
    EffectBindSampler2D01Comp,
    EffectBindSampler2D02Comp,
    EffectBindSampler2D03Comp,
    EffectBindSampler2D04Comp,
    EffectBindSampler2D05Comp,
    EffectBindSampler2D06Comp,
    EffectBindSampler2D07Comp,
    EffectBindSampler2D08Comp,
    EffectBindSampler2D08Comp,
    EffectTextureSamplersComp,
);