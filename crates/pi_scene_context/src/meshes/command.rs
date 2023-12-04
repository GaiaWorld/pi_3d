
use std::ops::Range;

use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Number, Vector3};

use crate::{
    geometry::{
        prelude::*,
        instance::{instance_boneoffset::*, instance_float::InstanceFloatType}
    },
    pass::*,
    renderers::prelude::*,
    layer_mask::prelude::*,
    skeleton::prelude::*,
    materials::prelude::*,
    cullings::prelude::*,
    transforms::prelude::*,
};

use super::{
    model::*,
    lighting::*,
    abstract_mesh::AbstructMesh,
    lighting::{MeshCastShadow, MeshReceiveShadow}
};

pub struct OpsMeshCreation(pub(crate) Entity, pub(crate) Entity, pub(crate) MeshInstanceState);
impl OpsMeshCreation {
    pub fn ops(scene: Entity, entity: Entity, state: MeshInstanceState) -> Self {
        Self(scene, entity, state)
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

pub struct OpsInstanceFloat(pub(crate) Entity, pub(crate) f32, pub InstanceFloatType);
impl OpsInstanceFloat {
    pub fn ops(instance: Entity, val: Number, float_type: InstanceFloatType) -> Self {
        Self(instance, val, float_type)
    }
}
pub type ActionListInstanceFloat = ActionList<OpsInstanceFloat>;


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
            Self(entity, Some(Range { start, end }), 0)
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

pub struct OpsMeshRenderVertexRange(pub(crate) Entity, pub(crate) Option<(u32, u32)>, pub u8);
impl OpsMeshRenderVertexRange {
    pub fn ops(entity: Entity, start: Option<u32>, count: Option<u32>) -> Self {
        if let (Some(start), Some(count)) = (start, count) {
            Self(entity, Some((start, count)), 0)
        } else {
            Self(entity, None, 0)
        }
    }
}
pub type ActionListMeshRenderVertexRange = ActionList<OpsMeshRenderVertexRange>;

pub struct OpsMeshForcePointLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool, pub(crate) u8);
impl OpsMeshForcePointLighting {
    pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
        Self(mesh_or_instance, light, is_add, 0)
    }
}
pub type ActionListMeshForcePointLighting = ActionList<OpsMeshForcePointLighting>;


pub struct OpsMeshForceSpotLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool, pub(crate) u8);
impl OpsMeshForceSpotLighting {
    pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
        Self(mesh_or_instance, light, is_add, 0)
    }
}
pub type ActionListMeshForceSpotLighting = ActionList<OpsMeshForceSpotLighting>;


pub struct OpsMeshForceHemiLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool, pub(crate) u8);
impl OpsMeshForceHemiLighting {
    pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
        Self(mesh_or_instance, light, is_add, 0)
    }
}
pub type ActionListMeshForceHemiLighting = ActionList<OpsMeshForceHemiLighting>;


pub struct BundleMesh(
    BundleTransformNode,
    AbstructMesh,
    Mesh,
    GeometryID,
    RenderGeometryEable,
    RenderWorldMatrix,
    // RenderWorldMatrixInv,
    RenderMatrixDirty,
    MeshCastShadow,
    MeshReceiveShadow,
    PassDirtyBindEffectValue,
    FlagPassDirtyBindEffectValue,
    PassDirtyBindEffectTextures,
    FlagPassDirtyBindEffectTextures,
    LayerMask,
    AbstructMeshCullingFlag,
    TransparentSortParam,
    CCullMode,
    CFrontFace,
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
    ModelVelocity,
    RenderAlignment,
    ScalingMode,
    IndiceRenderRange,
    RecordIndiceRenderRange,
    VertexRenderRange,
    GeometryBounding,
    GeometryCullingMode,
    InstancedMeshTransparentSortCollection,
    MeshInstanceState,
    InstanceBoneoffset,
    // InstanceSourceRefs,
    DirtyInstanceSourceRefs,
    InstanceWorldMatrixDirty,
    InstanceColorDirty,
    InstanceTillOffDirty,
    InstanceBoneOffsetDirty,
    InstanceVec4ADirty,
    InstanceVec4BDirty,
    InstanceVec4CDirty,
    InstanceVec4DDirty,
    PassID01,
    PassID02,
    PassID03,
    PassID04,
    PassID05,
    PassID06,
    PassID07,
    PassID08,
    MeshLightingMode,
    ModelLightingIndexs,
    ModelForcePointLightings,
    ModelForceSpotLightings,
    ModelForceHemiLightings,
    ModelPointLightingDirty,
    ModelSpotLightingDirty,
    ModelHemiLightingDirty,
);

pub struct BundleInstanceMesh(
    BundleTransformNode,
    AbstructMesh,
    AbstructMeshCullingFlag,
    InstanceTransparentIndex,
    InstanceMesh,
    InstanceRGB,
    InstanceAlpha,
    InstanceColor,
    InstanceTillOff,
    InstanceBoneoffset,
    RecordInstanceBoneoffset,
    InstanceVec4A,
    InstanceVec4B,
    InstanceVec4C,
    InstanceVec4D,
    RenderMatrixDirty,
    RenderWorldMatrix,
    RenderWorldMatrixInv,
    ModelVelocity,
    ScalingMode,
    GeometryBounding,
    GeometryCullingMode,
);

pub struct BundlePass(
    PassModelID,
    PassSceneID,
    PassSceneForSet3,
    PassViewerID,
    PassMaterialID,
    PassGeometryID,
    PassRendererID,
    PassPipelineStateDirty,
    PassDrawDirty,
    PassTag,
);

pub struct BundlePassActived(
    BundlePass,
    PassBindEffectValue,
    PassBindEffectTextures,
    PassBindGroupScene,
    PassBindGroupTextureSamplers,
    PassBindGroups,
    PassEffectReady,
    PassShader,
    PassPipeline,
    PassDraw,
);
