
use std::ops::Range;

use pi_engine_shell::prelude::*;
use pi_scene_math::*;

use crate::{
    geometry::prelude::*,
    pass::*,
    renderers::prelude::*,
    layer_mask::prelude::*,
    skeleton::prelude::*,
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

pub struct OpsInstanceFloat(pub(crate) Entity, pub(crate) Number, pub Atom);
impl OpsInstanceFloat {
    pub fn ops(instance: Entity, val: Number, attr: Atom) -> Self {
        Self(instance, val, attr)
    }
}
pub type ActionListInstanceFloat = ActionList<OpsInstanceFloat>;

pub struct OpsInstanceVec4(pub(crate) Entity, pub(crate) [Number; 4], pub Atom);
impl OpsInstanceVec4 {
    pub fn ops(instance: Entity, x: Number, y: Number, z: Number, w: Number, attr: Atom) -> Self {
        Self(instance, [x, y, z, w], attr)
    }
}
pub type ActionListInstanceVec4 = ActionList<OpsInstanceVec4>;

pub struct OpsInstanceVec3(pub(crate) Entity, pub(crate) [Number; 3], pub Atom);
impl OpsInstanceVec3 {
    pub fn ops(instance: Entity, x: Number, y: Number, z: Number, attr: Atom) -> Self {
        Self(instance, [x, y, z], attr)
    }
}
pub type ActionListInstanceVec3 = ActionList<OpsInstanceVec3>;

pub struct OpsInstanceVec2(pub(crate) Entity, pub(crate) [Number; 2], pub Atom);
impl OpsInstanceVec2 {
    pub fn ops(instance: Entity, x: Number, y: Number, attr: Atom) -> Self {
        Self(instance, [x, y], attr)
    }
}
pub type ActionListInstanceVec2 = ActionList<OpsInstanceVec2>;

pub struct OpsInstanceUint(pub(crate) Entity, pub(crate) u32, pub Atom);
impl OpsInstanceUint {
    pub fn ops(instance: Entity, x: u32, attr: Atom) -> Self {
        Self(instance, x, attr)
    }
}
pub type ActionListInstanceUint = ActionList<OpsInstanceUint>;

pub struct OpsInstanceSint(pub(crate) Entity, pub(crate) i32, pub Atom);
impl OpsInstanceSint {
    pub fn ops(instance: Entity, x: i32, attr: Atom) -> Self {
        Self(instance, x, attr)
    }
}
pub type ActionListInstanceSint = ActionList<OpsInstanceSint>;

pub struct OpsTargetAnimationAttribute(pub(crate) Entity, pub(crate) Atom, pub(crate) Entity, pub(crate) u64);
impl OpsTargetAnimationAttribute {
    pub fn ops(target: Entity, tatype: Atom, group: Entity, curve: u64) -> Self {
        Self(target, tatype, group, curve)
    }
}
pub type ActionListTargetAnimationAttribute = ActionList<OpsTargetAnimationAttribute>;

pub struct OpsBoneOffset(pub(crate) Entity, pub(crate) u32);
impl OpsBoneOffset {
    pub fn ops(instance: Entity, val: u32) -> Self {
        Self(instance, val)
    }
}
pub type ActionListBoneOffset = ActionList<OpsBoneOffset>;

pub struct OpsMeshRenderAlignment(pub(crate) Entity, pub(crate) RenderAlignment);
impl OpsMeshRenderAlignment {
    pub fn ops(entity: Entity, val: ERenderAlignment) -> Self {
        Self(entity, RenderAlignment(val))
    }
}
pub type ActionListMeshRenderAlignment = ActionList<OpsMeshRenderAlignment>;

pub struct OpsAbstructMeshScalingMode(pub(crate) Entity, pub(crate) ScalingMode);
impl OpsAbstructMeshScalingMode {
    pub fn ops(entity: Entity, val: EScalingMode) -> Self {
        Self(entity, ScalingMode(val))
    }
}
pub type ActionListAbstructMeshScalingMode = ActionList<OpsAbstructMeshScalingMode>;

pub struct OpsMeshRenderIndiceRange(pub(crate) Entity, pub(crate) Option<Range<u32>>);
impl OpsMeshRenderIndiceRange {
    pub fn ops(entity: Entity, start: Option<u32>, end: Option<u32>) -> Self {
        if let (Some(start), Some(end)) = (start, end) {
            Self(entity, Some(Range { start, end }))
        } else {
            Self(entity, None)
        }
    }
}
pub type ActionListMeshRenderIndiceRange = ActionList<OpsMeshRenderIndiceRange>;

pub struct OpsAbstructMeshVelocity(pub(crate) Entity, pub(crate) ModelVelocity);
impl OpsAbstructMeshVelocity {
    pub fn ops(entity: Entity, x: Number, y: Number, z: Number) -> Self {
        Self(entity, ModelVelocity(Vector3::new(x, y, z)))
    }
}
pub type ActionListAbstructMeshVelocity = ActionList<OpsAbstructMeshVelocity>;

pub struct OpsMeshRenderVertexRange(pub(crate) Entity, pub(crate) Option<(u32, u32)>);
impl OpsMeshRenderVertexRange {
    pub fn ops(entity: Entity, start: Option<u32>, count: Option<u32>) -> Self {
        if let (Some(start), Some(count)) = (start, count) {
            Self(entity, Some((start, count)))
        } else {
            Self(entity, None)
        }
    }
}
pub type ActionListMeshRenderVertexRange = ActionList<OpsMeshRenderVertexRange>;

pub struct OpsMeshForcePointLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
impl OpsMeshForcePointLighting {
    pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
        Self(mesh_or_instance, light, is_add)
    }
}
pub type ActionListMeshForcePointLighting = ActionList<OpsMeshForcePointLighting>;


pub struct OpsMeshForceSpotLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
impl OpsMeshForceSpotLighting {
    pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
        Self(mesh_or_instance, light, is_add)
    }
}
pub type ActionListMeshForceSpotLighting = ActionList<OpsMeshForceSpotLighting>;


pub struct OpsMeshForceHemiLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
impl OpsMeshForceHemiLighting {
    pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
        Self(mesh_or_instance, light, is_add)
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
    // InstanceSourceRefs,
    DirtyInstanceSourceRefs,
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
