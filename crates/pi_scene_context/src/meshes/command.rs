
use std::ops::Range;

use pi_scene_shell::prelude::*;

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

// #[derive(Debug)]
// pub enum OpsMeshShadow {
//     CastShadow(Entity, bool),
//     ReceiveShadow(Entity, bool),
// }
// pub type ActionListMeshShadow = ActionList<OpsMeshShadow>;

pub enum EInstanceAttr {
    Float(Number),
    Uint(u32),
    Int(i32),
    Vec4([Number; 4]),
    Vec3([Number; 3]),
    Vec2([Number; 2]),
}
// impl EInstanceAttr {
//     pub fn bytes(&self) -> Vec<u8> {
//         match self {
//             EInstanceAttr::Float(val) => bytemuck::cast_slice([*val]),
//             EInstanceAttr::Uint(val)  => bytemuck::cast(a),
//             EInstanceAttr::Int(val)   => bytemuck::cast(a),
//             EInstanceAttr::Vec4(val)  => bytemuck::cast(a),
//             EInstanceAttr::Vec3(val)  => bytemuck::cast(a),
//             EInstanceAttr::Vec2(val)  => bytemuck::cast(a),
//         }
//     }
// }

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

pub enum EMeshStateModify {
    Alignment(ERenderAlignment),
    ScalingMode(EScalingMode),
    CastShadow(bool),
    ReceiveShadow(bool),
    BoundingCullingMode(ECullingStrategy),
}
pub struct OpsMeshStateModify(pub(crate) Entity, pub(crate) EMeshStateModify);
impl OpsMeshStateModify {
    pub fn ops(instance: Entity, val: EMeshStateModify) -> Self {
        Self(instance, val)
    }
}
pub type ActionListMeshStateModify = ActionList<OpsMeshStateModify>;

// pub struct OpsBoneOffset(pub(crate) Entity, pub(crate) u32);
// impl OpsBoneOffset {
//     pub fn ops(instance: Entity, val: u32) -> Self {
//         Self(instance, val)
//     }
// }
// pub type ActionListBoneOffset = ActionList<OpsBoneOffset>;

// pub struct OpsMeshRenderAlignment(pub(crate) Entity, pub(crate) RenderAlignment);
// impl OpsMeshRenderAlignment {
//     pub fn ops(entity: Entity, val: ERenderAlignment) -> Self {
//         Self(entity, RenderAlignment(val))
//     }
// }
// pub type ActionListMeshRenderAlignment = ActionList<OpsMeshRenderAlignment>;

// pub struct OpsAbstructMeshScalingMode(pub(crate) Entity, pub(crate) ScalingMode);
// impl OpsAbstructMeshScalingMode {
//     pub fn ops(entity: Entity, val: EScalingMode) -> Self {
//         Self(entity, ScalingMode(val))
//     }
// }
// pub type ActionListAbstructMeshScalingMode = ActionList<OpsAbstructMeshScalingMode>;

// pub struct OpsMeshRenderIndiceRange(pub(crate) Entity, pub(crate) Option<Range<u32>>);
// impl OpsMeshRenderIndiceRange {
//     pub fn ops(entity: Entity, start: Option<u32>, end: Option<u32>) -> Self {
//         if let (Some(start), Some(end)) = (start, end) {
//             Self(entity, Some(Range { start, end }))
//         } else {
//             Self(entity, None)
//         }
//     }
// }
// pub type ActionListMeshRenderIndiceRange = ActionList<OpsMeshRenderIndiceRange>;

pub enum EMeshValueStateModify {
    BoneOffset(u32),
    IndiceRange(Option<(u32, u32)>),
    VertexRange(Option<(u32, u32)>),
    Velocity(Number, Number, Number),
}
pub struct OpsAbstructMeshValueStateModify(pub(crate) Entity, pub(crate) EMeshValueStateModify);
impl OpsAbstructMeshValueStateModify {
    pub fn ops(entity: Entity, val: EMeshValueStateModify) -> Self {
        Self(entity, val)
    }
}
pub type ActionListAbstructMeshValueStateModify = ActionList<OpsAbstructMeshValueStateModify>;

// pub struct OpsAbstructMeshVelocity(pub(crate) Entity, pub(crate) ModelVelocity);
// impl OpsAbstructMeshVelocity {
//     pub fn ops(entity: Entity, x: Number, y: Number, z: Number) -> Self {
//         Self(entity, ModelVelocity(Vector3::new(x, y, z)))
//     }
// }
// pub type ActionListAbstructMeshVelocity = ActionList<OpsAbstructMeshVelocity>;

// pub struct OpsMeshRenderVertexRange(pub(crate) Entity, pub(crate) Option<(u32, u32)>);
// impl OpsMeshRenderVertexRange {
//     pub fn ops(entity: Entity, start: Option<u32>, count: Option<u32>) -> Self {
//         if let (Some(start), Some(count)) = (start, count) {
//             Self(entity, Some((start, count)))
//         } else {
//             Self(entity, None)
//         }
//     }
// }
// pub type ActionListMeshRenderVertexRange = ActionList<OpsMeshRenderVertexRange>;

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

// pub struct OpsMeshForcePointLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
// impl OpsMeshForcePointLighting {
//     pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
//         Self(mesh_or_instance, light, is_add)
//     }
// }
// pub type ActionListMeshForcePointLighting = ActionList<OpsMeshForcePointLighting>;


// pub struct OpsMeshForceSpotLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
// impl OpsMeshForceSpotLighting {
//     pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
//         Self(mesh_or_instance, light, is_add)
//     }
// }
// pub type ActionListMeshForceSpotLighting = ActionList<OpsMeshForceSpotLighting>;


// pub struct OpsMeshForceHemiLighting(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
// impl OpsMeshForceHemiLighting {
//     pub fn ops(mesh_or_instance: Entity, light: Entity, is_add: bool) -> Self {
//         Self(mesh_or_instance, light, is_add)
//     }
// }
// pub type ActionListMeshForceHemiLighting = ActionList<OpsMeshForceHemiLighting>;

pub type BundleMeshGeometry = (
    GeometryID,
    RenderGeometryEable,
    MeshInstanceState,
    DirtyInstanceSourceRefs,
);

pub type BundleMeshRenderState = (
    // CCullMode,
    // CFrontFace,
    // CPolygonMode,
    // Topology,
    // CUnClipDepth,

    // DepthWrite,
    // DepthCompare,
    // DepthBias,
    // StencilFront,
    // StencilBack,
    // StencilRead,
    // StencilWrite,

    // ModelBlend,
);

pub type BundleMeshMaterial = (
    PassIDs,
);

pub type BundleMeshFunctionRenderModules = (
    BundleMeshLighting,
    ModelVelocity,
    BindSkinValue,
    RenderAlignment,
    ScalingMode,
);

pub type BundleMeshLighting = (
    MeshLightingMode,
    ModelLightingIndexs,
    ModelForcePointLightings,
    ModelForceSpotLightings,
    ModelForceHemiLightings,
    // ModelPointLightingDirty,
    // ModelSpotLightingDirty,
    // ModelHemiLightingDirty,
);

pub type BundleMesh = (
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

    // CCullMode,
    // CFrontFace,
    // CPolygonMode,
    // Topology,
    // CUnClipDepth,

    // DepthWrite,
    // DepthCompare,
    // DepthBias,
    // StencilFront,
    // StencilBack,
    // StencilRead,
    // StencilWrite,

    // ModelBlend,
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
    PassIDs,
    MeshLightingMode,
    ModelLightingIndexs,
    ModelForcePointLightings,
    ModelForceSpotLightings,
    ModelForceHemiLightings,
    // ModelPointLightingDirty,
    // ModelSpotLightingDirty,
    // ModelHemiLightingDirty,
);

pub type BundleInstanceMesh = (
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

pub type BundlePass = (
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

pub type BundlePassActived = (
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
