
use std::ops::Range;

use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Number, Vector3};

use crate::{
    geometry::{
        prelude::*,
        instance::instance_boneoffset::*
    },
    pass::*,
    renderers::prelude::*,
    layer_mask::prelude::*,
    skeleton::prelude::*,
    materials::prelude::*, prelude::{RenderAlignment, ScalingMode, ModelVelocity, BundleTransformNode, EScalingMode, IndiceRenderRange},
};

use super::{
    model::{RenderWorldMatrix, RenderWorldMatrixInv, RenderMatrixDirty},
    abstract_mesh::AbstructMesh,
    Mesh,
    lighting::{MeshCastShadow, MeshReceiveShadow}
};

pub struct OpsMeshCreation(pub(crate) Entity, pub(crate) Entity);
impl OpsMeshCreation {
    pub fn ops(scene: Entity, entity: Entity) -> Self {
        Self(scene, entity)
    }
}
pub type ActionListMeshCreate = ActionList<OpsMeshCreation>;

pub struct OpsInstanceMeshCreation(pub(crate) Entity, pub(crate) Entity, pub u8);
impl OpsInstanceMeshCreation {
    pub fn ops(source: Entity, instance: Entity) -> Self {
        Self(source, instance, 0)
    }
}
pub type ActionListInstanceMeshCreate = ActionList<OpsInstanceMeshCreation>;

#[derive(Debug)]
pub enum OpsMeshShadow {
    CastShadow(Entity, bool),
    ReceiveShadow(Entity, bool),
}
pub type ActionListMeshShadow = ActionList<OpsMeshShadow>;

pub struct OpsInstanceColorAlpha(pub(crate) Entity, pub(crate) Number, pub(crate) Number, pub(crate) Number, pub(crate) Number, pub u8);
impl OpsInstanceColorAlpha {
    pub fn ops(instance: Entity, r: Number, g: Number, b: Number, a: Number) -> Self {
        Self(instance, r, g, b, a, 0)
    }
}
pub type ActionListInstanceColorAlpha = ActionList<OpsInstanceColorAlpha>;

pub struct OpsInstanceColor(pub(crate) Entity, pub(crate) Number, pub(crate) Number, pub(crate) Number, pub u8);
impl OpsInstanceColor {
    pub fn ops(instance: Entity, r: Number, g: Number, b: Number) -> Self {
        Self(instance, r, g, b, 0)
    }
}
pub type ActionListInstanceColor = ActionList<OpsInstanceColor>;

pub struct OpsInstanceAlpha(pub(crate) Entity, pub(crate) Number, pub u8);
impl OpsInstanceAlpha {
    pub fn ops(instance: Entity, alpha: Number) -> Self {
        Self(instance, alpha, 0)
    }
}
pub type ActionListInstanceAlpha = ActionList<OpsInstanceAlpha>;

pub struct OpsInstanceTillOff(pub(crate) Entity, pub(crate) Vector4, pub u8);
impl OpsInstanceTillOff {
    pub fn ops(instance: Entity, uscale: Number, vscale: Number, uoffset: Number, voffset: Number) -> Self {
        Self(instance, Vector4::new(uscale, vscale, uoffset, voffset), 0)
    }
}
pub type ActionListInstanceTillOff = ActionList<OpsInstanceTillOff>;

pub struct OpsBoneOffset(pub(crate) Entity, pub(crate) u32, pub u8);
impl OpsBoneOffset {
    pub fn ops(instance: Entity, val: u32) -> Self {
        Self(instance, val, 0)
    }
}
pub type ActionListBoneOffset = ActionList<OpsBoneOffset>;

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

pub struct OpsMeshRenderIndiceRange(pub(crate) Entity, pub(crate) Option<Range<u32>>, pub u8);
impl OpsMeshRenderIndiceRange {
    pub fn ops(entity: Entity, start: Option<u32>, end: Option<u32>) -> Self {
        if let (Some(start), Some(end)) = (start, end) {
            Self(entity, Some(Range{ start, end }), 0)
        } else {
            Self(entity, None, 0)
        }
    }
}
pub type ActionListMeshRenderIndiceRange = ActionList<OpsMeshRenderIndiceRange>;

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
    IndiceRenderRange,
    PassID01,
    PassID02,
    PassID03,
    PassID04,
    PassID05,
    PassID06,
    PassID07,
    PassID08,
    InstanceBoneoffset,
    InstanceSourceRefs,
    InstanceColorDirty,
    InstanceTillOffDirty,
    InstanceWorldMatrixDirty,
    InstanceBoneOffsetDirty,
);

pub struct BundleInstanceMesh(
    BundleTransformNode,
    AbstructMesh,
    InstanceMesh,
    InstanceRGB,
    InstanceAlpha,
    InstanceColor,
    InstanceTillOff,
    InstanceBoneoffset,
    RenderMatrixDirty,
    RenderWorldMatrix,
    RenderWorldMatrixInv,
    ScalingMode,
    ModelVelocity,
);

pub struct BundlePass(
    ModelPass,
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