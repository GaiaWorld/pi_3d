
use pi_engine_shell::prelude::*;

use crate::pass::EPassTag;
use super::material::*;
use super::shader_effect::*;
use super::uniforms::{uniform::*, texture::*};

pub struct OpsMaterialCreate(pub Entity, pub KeyShaderMeta, pub EPassTag);
impl OpsMaterialCreate {
    pub fn ops(mat: Entity, shader_meta: &str, pass: EPassTag) -> Self {
        Self(mat, Atom::from(shader_meta), pass)
    }
}
pub type ActionListMaterialCreate = ActionList<OpsMaterialCreate>;

#[derive(Debug)]
pub enum OpsMaterialUse {
    Use(Entity, Entity),
    UnUse(Entity, Entity),
}
impl OpsMaterialUse {
    pub fn ops(id_mesh: Entity, id_mat: Entity) -> Self {
        Self::Use(id_mesh, id_mat)
    }
}
pub type ActionListMaterialUse = ActionList<OpsMaterialUse>;

/// 材质属性操作 当没有找到目标材质时,最多等待 MATERIAL_UNIFORM_OPS_WAIT_FRAME 帧, 便抛弃该操作
pub const MATERIAL_UNIFORM_OPS_WAIT_FRAME: u16 = 8;

pub struct OpsUniformMat4(pub(crate) Entity, pub(crate) Atom, pub(crate) [f32;16], pub(crate) u16);
impl OpsUniformMat4 {
    pub fn ops(mat: Entity, uniformname: Atom, value: [f32;16]) -> Self {
        Self(mat, uniformname, value, 0)
    }
}
pub type ActionListUniformMat4 = ActionList<OpsUniformMat4>;

pub struct OpsUniformMat2(pub(crate) Entity, pub(crate) Atom, pub(crate) [f32;4], pub(crate) u16);
impl OpsUniformMat2 {
    pub fn ops(mat: Entity, uniformname: Atom, value: [f32;4]) -> Self {
        Self(mat, uniformname, value, 0)
    }
}
pub type ActionListUniformMat2 = ActionList<OpsUniformMat2>;


pub struct OpsUniformVec4(pub(crate) Entity, pub(crate) Atom, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) u16);
impl OpsUniformVec4 {
    pub fn ops(mat: Entity, uniformname: Atom, x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(mat, uniformname, x, y, z, w, 0)
    }
}
pub type ActionListUniformVec4 = ActionList<OpsUniformVec4>;

pub struct OpsUniformVec2(pub(crate) Entity, pub(crate) Atom, pub(crate) f32, pub(crate) f32, pub(crate) u16);
impl OpsUniformVec2 {
    pub fn ops(mat: Entity, uniformname: Atom, x: f32, y: f32) -> Self {
        Self(mat, uniformname, x, y, 0)
    }
}
pub type ActionListUniformVec2 = ActionList<OpsUniformVec2>;

pub struct OpsUniformFloat(pub(crate) Entity, pub(crate) Atom, pub(crate) f32, pub(crate) u16);
impl OpsUniformFloat {
    pub fn ops(mat: Entity, uniformname: Atom, x: f32) -> Self {
        Self(mat, uniformname, x, 0)
    }
}
pub type ActionListUniformFloat = ActionList<OpsUniformFloat>;

pub struct OpsUniformInt(pub(crate) Entity, pub(crate) Atom, pub(crate) i32, pub(crate) u16);
impl OpsUniformInt {
    pub fn ops(mat: Entity, uniformname: Atom, x: i32) -> Self {
        Self(mat, uniformname, x, 0)
    }
}
pub type ActionListUniformInt = ActionList<OpsUniformInt>;

pub struct OpsUniformUint(pub(crate) Entity, pub(crate) Atom, pub(crate) u32, pub(crate) u16);
impl OpsUniformUint {
    pub fn ops(mat: Entity, uniformname: Atom, x: u32) -> Self {
        Self(mat, uniformname, x, 0)
    }
}
pub type ActionListUniformUint = ActionList<OpsUniformUint>;

pub struct OpsUniformTexture(pub(crate) Entity, pub(crate) UniformTextureWithSamplerParam, pub(crate) u16);
impl OpsUniformTexture {
    pub fn ops(mat: Entity, val: UniformTextureWithSamplerParam) -> Self {
        Self(mat, val, 0)
    }
}
pub type ActionListUniformTexture = ActionList<OpsUniformTexture>;

/// size 1608 + 4 + 4 + 4
pub struct BundleMaterial (
    BindEffect,
    AssetKeyShaderEffect,
    AssetResShaderEffectMeta,
    BindEffectValueDirty,
    EPassTag ,
    DirtyMaterialRefs,
    MaterialRefs,
    UniformTextureWithSamplerParams,
);