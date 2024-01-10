
use pi_scene_shell::prelude::*;

use crate::pass::PassTag;

pub struct OpsMaterialCreate(pub Entity, pub KeyShaderMeta);
impl OpsMaterialCreate {
    pub fn ops(mat: Entity, shader_meta: &str) -> Self {
        Self(mat, Atom::from(shader_meta))
    }
}
pub type ActionListMaterialCreate = ActionList<OpsMaterialCreate>;

#[derive(Debug)]
pub enum OpsMaterialUse {
    Use(Entity, Entity, PassTag),
    UnUse(Entity, Entity),
}
impl OpsMaterialUse {
    pub fn ops(id_mesh: Entity, id_mat: Entity, pass: PassTag) -> Self {
        Self::Use(id_mesh, id_mat, pass)
    }
}
pub type ActionListMaterialUse = ActionList<OpsMaterialUse>;

/// 材质属性操作 当没有找到目标材质时,最多等待 MATERIAL_UNIFORM_OPS_WAIT_FRAME 帧, 便抛弃该操作
pub const MATERIAL_UNIFORM_OPS_WAIT_FRAME: u16 = 8;

pub struct OpsUniformMat4(pub(crate) Entity, pub(crate) Atom, pub(crate) [f32;16]);
impl OpsUniformMat4 {
    pub fn ops(mat: Entity, uniformname: Atom, value: [f32;16]) -> Self {
        Self(mat, uniformname, value)
    }
}
pub type ActionListUniformMat4 = ActionList<OpsUniformMat4>;

// pub struct OpsUniformMat2(pub(crate) Entity, pub(crate) Atom, pub(crate) [f32;4]);
// impl OpsUniformMat2 {
//     pub fn ops(mat: Entity, uniformname: Atom, value: [f32;4]) -> Self {
//         Self(mat, uniformname, value)
//     }
// }
// pub type ActionListUniformMat2 = ActionList<OpsUniformMat2>;


pub struct OpsUniformVec4(pub(crate) Entity, pub(crate) Atom, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsUniformVec4 {
    pub fn ops(mat: Entity, uniformname: Atom, x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(mat, uniformname, x, y, z, w)
    }
}
pub type ActionListUniformVec4 = ActionList<OpsUniformVec4>;

pub struct OpsUniformVec3(pub(crate) Entity, pub(crate) Atom, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsUniformVec3 {
    pub fn ops(mat: Entity, uniformname: Atom, x: f32, y: f32, z: f32) -> Self {
        Self(mat, uniformname, x, y, z)
    }
}
pub type ActionListUniformVec3 = ActionList<OpsUniformVec3>;

pub struct OpsUniformVec2(pub(crate) Entity, pub(crate) Atom, pub(crate) f32, pub(crate) f32);
impl OpsUniformVec2 {
    pub fn ops(mat: Entity, uniformname: Atom, x: f32, y: f32) -> Self {
        Self(mat, uniformname, x, y)
    }
}
pub type ActionListUniformVec2 = ActionList<OpsUniformVec2>;

pub struct OpsUniformFloat(pub(crate) Entity, pub(crate) Atom, pub(crate) f32);
impl OpsUniformFloat {
    pub fn ops(mat: Entity, uniformname: Atom, x: f32) -> Self {
        Self(mat, uniformname, x)
    }
}
pub type ActionListUniformFloat = ActionList<OpsUniformFloat>;

// pub struct OpsUniformInt(pub(crate) Entity, pub(crate) Atom, pub(crate) i32);
// impl OpsUniformInt {
//     pub fn ops(mat: Entity, uniformname: Atom, x: i32) -> Self {
//         Self(mat, uniformname, x)
//     }
// }
// pub type ActionListUniformInt = ActionList<OpsUniformInt>;

pub struct OpsUniformUint(pub(crate) Entity, pub(crate) Atom, pub(crate) u32);
impl OpsUniformUint {
    pub fn ops(mat: Entity, uniformname: Atom, x: u32) -> Self {
        Self(mat, uniformname, x)
    }
}
pub type ActionListUniformUint = ActionList<OpsUniformUint>;

pub struct OpsUniformTexture(pub(crate) Entity, pub(crate) UniformTextureWithSamplerParam);
impl OpsUniformTexture {
    pub fn ops(mat: Entity, val: UniformTextureWithSamplerParam) -> Self {
        Self(mat, val)
    }
}
pub type ActionListUniformTexture = ActionList<OpsUniformTexture>;

pub struct OpsUniformTextureFromRenderTarget(pub(crate) Entity, pub(crate) UniformTextureWithSamplerParam, pub(crate) KeyRenderTarget, pub(crate) Atom);
impl OpsUniformTextureFromRenderTarget {
    pub fn ops(mat: Entity, val: UniformTextureWithSamplerParam, keytarget: KeyRenderTarget, tilloffslot: Atom) -> Self {
        Self(mat, val, keytarget, tilloffslot)
    }
}
pub type ActionListUniformTextureFromRenderTarget = ActionList<OpsUniformTextureFromRenderTarget>;

pub type EAnimeUniform = Atom;
pub struct OpsTargetAnimationUniform(pub(crate) Entity, pub(crate) EAnimeUniform, pub(crate) Entity, pub(crate) u64);
impl OpsTargetAnimationUniform {
    pub fn ops(target: Entity, tatype: EAnimeUniform, group: Entity, curve: u64) -> Self {
        Self(target, tatype, group, curve)
    }
}
pub type ActionListTargetAnimationUniform = ActionList<OpsTargetAnimationUniform>;
