
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Matrix, Number};

use crate::{
    geometry::{
        prelude::*,
        command_sys::*
    },
    pass::*,
    renderers::{
        prelude::*,
    },
    layer_mask::prelude::*,
    skeleton::prelude::*,
    materials::prelude::*,
};

use super::{
    model::{RenderWorldMatrix, RenderWorldMatrixInv, RenderMatrixDirty},
    abstract_mesh::AbstructMesh,
    Mesh,
    lighting::{MeshCastShadow, MeshReceiveShadow}
};

pub struct OpsMeshCreation(pub(crate) Entity, pub(crate) Entity, pub(crate) String);
impl OpsMeshCreation {
    pub fn ops(scene: Entity, entity: Entity, name: String) -> Self {
        Self(scene, entity, name)
    }
}
pub type ActionListMeshCreate = ActionList<OpsMeshCreation>;

pub struct OpsInstanceMeshCreation(pub(crate) Entity, pub(crate) Entity, pub(crate) String);
impl OpsInstanceMeshCreation {
    pub fn ops(source: Entity, instance: Entity, name: String) -> Self {
        Self(source, instance, name)
    }
}
pub type ActionListInstanceMeshCreate = ActionList<OpsInstanceMeshCreation>;

#[derive(Debug)]
pub enum OpsMeshShadow {
    CastShadow(Entity, bool),
    ReceiveShadow(Entity, bool),
}
pub type ActionListMeshShadow = ActionList<OpsMeshShadow>;

pub struct OpsInstanceColor(pub(crate) Entity, pub(crate) Vector4);
impl OpsInstanceColor {
    pub fn ops(instance: Entity, r: Number, g: Number, b: Number, a: Number) -> Self {
        Self(instance, Vector4::new(r, g, b, a))
    }
}
pub type ActionListInstanceColor = ActionList<OpsInstanceColor>;

pub struct OpsInstanceTillOff(pub(crate) Entity, pub(crate) Vector4);
impl OpsInstanceTillOff {
    pub fn ops(instance: Entity, uscale: Number, vscale: Number, uoffset: Number, voffset: Number) -> Self {
        Self(instance, Vector4::new(uscale, vscale, uoffset, voffset))
    }
}
pub type ActionListInstanceTillOff = ActionList<OpsInstanceTillOff>;

pub struct BundleMesh(
    AbstructMesh,
    Mesh,
    RenderWorldMatrix,
    RenderWorldMatrixInv,
    RenderMatrixDirty,
    MeshCastShadow,
    MeshReceiveShadow,
    PassDirtyBindEffectValue,
    FlagPassDirtyBindEffectValue,
    PassDirtyBindEffectTextures,
    FlagPassDirtyBindEffectTextures,
    LayerMask,
    Opaque,
    TransparentSortParam,
    ECullMode,
    FrontFace,
    PolygonMode,
    DepthWrite,
    DepthCompare,
    DepthBias,
    StencilFront,
    StencilBack,
    StencilRead,
    StencilWrite,
    ModelBlend,
    ModelBlend,
    BindSkinValue,
);

pub struct BundleInstanceMesh(
    AbstructMesh,
    InstanceSourceID,
    InstanceColor,
    InstanceTillOff,
    RenderMatrixDirty,
    RenderWorldMatrix,
    RenderWorldMatrixInv,
);

pub struct BundlePass(
    PassSource,
    PassBindEffectValue,
    PassBindEffectTextures,
    PassBindGroupScene,
    PassBindGroupTextureSamplers,
    PassBindGroups,
    PassReady,
    PassShader,
    PassPipeline,
    PassDraw,
    MaterialID,
);