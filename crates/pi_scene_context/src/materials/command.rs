
use pi_scene_shell::prelude::*;
use serde::{Deserialize, Serialize};

use crate::pass::PassTag;

pub struct OpsMaterialCreate(pub Entity, pub KeyShaderMeta, pub bool);
impl OpsMaterialCreate {
    pub fn ops_with_matarray(mat: Entity, shader_meta: &str) -> Self {
        Self(mat, Atom::from(shader_meta), true)
    }
    pub fn ops(mat: Entity, shader_meta: &str, withmatarray: bool) -> Self {
        Self(mat, Atom::from(shader_meta), withmatarray)
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

pub enum OpsUniformValB{
    Mat4(Entity, Atom, [f32;16]),
    Texture(Entity, UniformTextureWithSamplerParam),
    TextureFromRenderTarget(Entity, UniformTextureWithSamplerParam, KeyRenderTarget, Atom),
    TargetAnimation(Entity, EAnimeUniform, Entity, u64),
    TextureFromRenderInput(Entity, UniformTextureWithSamplerParam, Entity, Atom),
}
impl OpsUniformValB {
    pub fn mat4(mat: Entity, uniformname: Atom, value: [f32;16]) -> Self {
        Self::Mat4(mat, uniformname, value)
    }
    pub fn texture(mat: Entity, val: UniformTextureWithSamplerParam) -> Self {
        Self::Texture(mat, val)
    }
    pub fn texture_from_target(mat: Entity, val: UniformTextureWithSamplerParam, keytarget: KeyRenderTarget, tilloffslot: Atom) -> Self {
        Self::TextureFromRenderTarget(mat, val, keytarget, tilloffslot)
    }
    pub fn texture_from_renderer(mat: Entity, val: UniformTextureWithSamplerParam, keytarget: Entity, tilloffslot: Atom) -> Self {
        Self::TextureFromRenderInput(mat, val, keytarget, tilloffslot)
    }
    pub fn targetanim(target: Entity, tatype: EAnimeUniform, group: Entity, curve: u64) -> Self {
        Self::TargetAnimation(target, tatype, group, curve)
    }
}
pub type ActionListUniformValB = ActionList<OpsUniformValB>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EUniformVal {
    Vec4 (Atom, f32, f32, f32, f32),
    Vec3 (Atom, f32, f32, f32),
    Vec2 (Atom, f32, f32),
    Float(Atom, f32),
    Uint (Atom, u32),
}
pub struct OpsUniformVal(pub(crate) Entity, pub(crate) EUniformVal);
impl OpsUniformVal {
    pub fn ops(mat: Entity, val: EUniformVal) -> Self {
        Self(mat, val)
    }
    pub fn vec4(mat: Entity, uniformname: Atom, x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(mat, EUniformVal::Vec4(uniformname, x, y, z, w))
    }
    pub fn vec3(mat: Entity, uniformname: Atom, x: f32, y: f32, z: f32) -> Self {
        Self(mat, EUniformVal::Vec3( uniformname, x, y, z))
    }
    pub fn vec2(mat: Entity, uniformname: Atom, x: f32, y: f32) -> Self {
        Self(mat, EUniformVal::Vec2( uniformname, x, y))
    }
    pub fn float(mat: Entity, uniformname: Atom, x: f32) -> Self {
        Self(mat, EUniformVal::Float( uniformname, x))
    }
    pub fn uint(mat: Entity, uniformname: Atom, x: u32) -> Self {
        Self(mat, EUniformVal::Uint( uniformname, x))
    }
}
pub type ActionListUniformVal = ActionList<OpsUniformVal>;

pub type EAnimeUniform = Atom;