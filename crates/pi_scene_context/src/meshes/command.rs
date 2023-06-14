
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Matrix, Number, Vector3};

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
    materials::prelude::*, prelude::{RenderAlignment, ScalingMode, ModelVelocity, BundleTransformNode, EScalingMode},
};

use super::{
    model::{RenderWorldMatrix, RenderWorldMatrixInv, RenderMatrixDirty},
    abstract_mesh::AbstructMesh,
    Mesh,
    lighting::{MeshCastShadow, MeshReceiveShadow}
};

pub struct OpsMeshCreation(pub(crate) Entity, pub(crate) Entity, pub(crate) String, pub u8);
impl OpsMeshCreation {
    pub fn ops(scene: Entity, entity: Entity, name: String) -> Self {
        Self(scene, entity, name, 0)
    }
}
pub type ActionListMeshCreate = ActionList<OpsMeshCreation>;

pub struct OpsInstanceMeshCreation(pub(crate) Entity, pub(crate) Entity, pub(crate) String, pub u8);
impl OpsInstanceMeshCreation {
    pub fn ops(source: Entity, instance: Entity, name: String) -> Self {
        Self(source, instance, name, 0)
    }
}
pub type ActionListInstanceMeshCreate = ActionList<OpsInstanceMeshCreation>;

#[derive(Debug)]
pub enum OpsMeshShadow {
    CastShadow(Entity, bool),
    ReceiveShadow(Entity, bool),
}
pub type ActionListMeshShadow = ActionList<OpsMeshShadow>;

pub struct OpsInstanceColor(pub(crate) Entity, pub(crate) Vector4, pub u8);
impl OpsInstanceColor {
    pub fn ops(instance: Entity, r: Number, g: Number, b: Number, a: Number) -> Self {
        Self(instance, Vector4::new(r, g, b, a), 0)
    }
}
pub type ActionListInstanceColor = ActionList<OpsInstanceColor>;

pub struct OpsInstanceTillOff(pub(crate) Entity, pub(crate) Vector4, pub u8);
impl OpsInstanceTillOff {
    pub fn ops(instance: Entity, uscale: Number, vscale: Number, uoffset: Number, voffset: Number) -> Self {
        Self(instance, Vector4::new(uscale, vscale, uoffset, voffset), 0)
    }
}
pub type ActionListInstanceTillOff = ActionList<OpsInstanceTillOff>;

pub struct OpsMeshRenderAlignment(pub(crate) Entity, pub(crate) RenderAlignment, pub u8);
impl OpsMeshRenderAlignment {
    pub fn ops(entity: Entity, val: ERenderAlignment) -> Self {
        Self(entity, RenderAlignment(val), 0)
    }
}
pub type ActionListMeshRenderAlignment = ActionList<OpsMeshRenderAlignment>;

pub struct OpsAbstructMeshScalingMode(pub(crate) Entity, pub(crate) ScalingMode, pub u8);
impl OpsAbstructMeshScalingMode {
    pub fn ops(entity: Entity, val: EScalingMode) -> Self {
        Self(entity, ScalingMode(val), 0)
    }
}
pub type ActionListAbstructMeshScalingMode = ActionList<OpsAbstructMeshScalingMode>;

pub struct OpsAbstructMeshVelocity(pub(crate) Entity, pub(crate) ModelVelocity, pub u8);
impl OpsAbstructMeshVelocity {
    pub fn ops(entity: Entity, x: Number, y: Number, z: Number) -> Self {
        Self(entity, ModelVelocity(Vector3::new(x, y, z)), 0)
    }
}
pub type ActionListAbstructMeshVelocity = ActionList<OpsAbstructMeshVelocity>;


pub struct BundleMesh(
    BundleTransformNode,
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
    CCullMode,
    FrontFace,
    CPolygonMode,
    Topology,
    CUnClipDepth,
    DepthWrite,
    DepthCompare,
    DepthBias,
    StencilFront,
    StencilBack,
    StencilRead,
    StencilWrite,
    ModelBlend,
    BindSkinValue,
    RenderAlignment,
    ScalingMode,
    ModelVelocity,
);

pub struct BundleInstanceMesh(
    BundleTransformNode,
    AbstructMesh,
    InstanceSourceID,
    InstanceColor,
    InstanceTillOff,
    RenderMatrixDirty,
    RenderWorldMatrix,
    RenderWorldMatrixInv,
    ScalingMode,
    ModelVelocity,
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