
use pi_scene_shell::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{cullings::prelude::*, geometry::instance::EInstanceSortMode};

use super::model::*;

pub struct OpsMeshCreation(pub(crate) Entity, pub(crate) Entity, pub(crate) MeshInstanceState);
impl OpsMeshCreation {
    pub fn ops(scene: Entity, entity: Entity, state: MeshInstanceState) -> Self {
        Self(scene, entity, state)
    }
}
pub type ActionListMeshCreate = ActionList<OpsMeshCreation>;

pub struct OpsInstanceMeshCreation(pub(crate) Entity, pub(crate) Entity);
impl OpsInstanceMeshCreation {
    pub fn ops(source: Entity, instance: Entity) -> Self {
        Self(source, instance)
    }
}
pub type ActionListInstanceMeshCreate = ActionList<OpsInstanceMeshCreation>;

pub struct OpsAbstractMeshPose(pub(crate) Entity, pub Matrix);
impl OpsAbstractMeshPose {
    pub fn ops(source: Entity, pose: Matrix) -> Self {
        Self(source, pose)
    }
}
pub type ActionListAbstractMeshPose = ActionList<OpsAbstractMeshPose>;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EInstanceAttr {
    Float(Number),
    Uint(u32),
    Int(i32),
    Vec4([Number; 4]),
    Vec3([Number; 3]),
    Vec2([Number; 2]),
    IVec4([i32; 4]),
    U8x4([u8; 4]),
    U16x4([u16; 4]),
    U16x2([u16; 2]),
}

pub struct OpsInstanceAttr(pub(crate) Entity, pub(crate) EInstanceAttr, pub Atom);
impl OpsInstanceAttr {
    pub fn ops(instance: Entity, val: EInstanceAttr, attr: Atom) -> Self {
        Self(instance, val, attr)
    }
}
pub type ActionListInstanceAttr = ActionList<OpsInstanceAttr>;

pub struct OpsTargetAnimationAttribute(pub(crate) Entity, pub(crate) Atom, pub(crate) Entity, pub(crate) u64);
impl OpsTargetAnimationAttribute {
    pub fn ops(target: Entity, tatype: Atom, group: Entity, curve: u64) -> Self {
        Self(target, tatype, group, curve)
    }
}
pub type ActionListTargetAnimationAttribute = ActionList<OpsTargetAnimationAttribute>;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EMeshStateModify {
    Alignment(ERenderAlignment),
    ScalingMode(EScalingMode),
    CastShadow(bool),
    ReceiveShadow(bool),
    BoundingCullingMode(ECullingStrategy),
    InstanceSortMode(EInstanceSortMode),
}
pub struct OpsMeshStateModify(pub(crate) Entity, pub(crate) EMeshStateModify);
impl OpsMeshStateModify {
    pub fn ops(instance: Entity, val: EMeshStateModify) -> Self {
        Self(instance, val)
    }
}
pub type ActionListMeshStateModify = ActionList<OpsMeshStateModify>;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EMeshValueStateModify {
    BoneOffset(u32),
    IndiceRange(Option<(u32, u32)>),
    VertexRange(Option<(u32, u32)>),
    Velocity(Number, Number, Number),
    MorphInfluence(Number, Number, Number, Number),
}
pub struct OpsAbstructMeshValueStateModify(pub(crate) Entity, pub(crate) EMeshValueStateModify);
impl OpsAbstructMeshValueStateModify {
    pub fn ops(entity: Entity, val: EMeshValueStateModify) -> Self {
        Self(entity, val)
    }
}
pub type ActionListAbstructMeshValueStateModify = ActionList<OpsAbstructMeshValueStateModify>;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EMeshForceLighting {
    ForcePointLighting(bool),
    ForceSpotLighting(bool),
    ForceHemiLighting(bool),
}
pub struct OpsMeshForceLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) EMeshForceLighting);
impl OpsMeshForceLighting {
    pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: EMeshForceLighting) -> Self {
        Self(mesh_or_instance, light, is_add)
    }
}
pub type ActionListMeshForceLighting = ActionList<OpsMeshForceLighting>;
