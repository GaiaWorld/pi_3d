use std::{sync::Arc, ops::Range};

use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;
use pi_scene_math::{Matrix, Vector3};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageModel {
    CreateMesh,
    _InitMesh,
    CreateInstance,
    _InitInstance,
    AbstructMeshCommand,
    InstanceEffectMesh,
    RenderMatrix,
    InstanceEffectGeometry,
    LightingCollect,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum EScalingMode {
    Hierarchy = 0,
    Local,
    Shape,
}
impl Default for EScalingMode {
    fn default() -> Self {
        Self::Hierarchy
    }
}


#[derive(Component)]
pub struct Mesh;

#[derive(Component)]
/// 相对于 SourceMesh 的 AlphaIndex
pub struct InstanceTransparentIndex(pub i32);

#[derive(Component)]
pub struct AbstructMeshCullingFlag(pub bool);

#[derive(Component)]
pub struct MeshInstanceState {
    pub instances: Vec<CustomVertexAttribute>,
    pub instance_matrix: bool,
    pub use_single_instancebuffer: bool,
}
impl Default for MeshInstanceState {
    fn default() -> Self {
        Self { instances: vec![], instance_matrix: false, use_single_instancebuffer: false }
    }
}

#[derive(Clone, Default, Component)]
pub struct DirtyMeshRef;

pub type MeshRefs = EntityRefInfo<DirtyMeshRef>;

#[derive(Clone, Component, Deref, DerefMut)]
pub struct RenderAlignment(pub ERenderAlignment);
impl Default for RenderAlignment {
    fn default() -> Self {
        Self(ERenderAlignment::Local)
    }
}

#[derive(Clone, Default, Component, Deref, DerefMut)]
pub struct ScalingMode(pub EScalingMode);

#[derive(Clone, Component, Deref, DerefMut)]
pub struct ModelVelocity(pub Vector3);
impl Default for ModelVelocity {
    fn default() -> Self {
        Self(Vector3::new(0., 1., 0.))
    }
}

#[derive(Component, Clone)]
pub struct ModelStatic;

#[derive(Component, Clone)]
pub struct BindModel(pub Arc<ShaderBindModelAboutMatrix>);
impl BindModel {
    pub fn new(
        allocator: &mut BindBufferAllocator,
    ) -> Option<Self> {

        if let Some(bind) = ShaderBindModelAboutMatrix::new(allocator) {
            Some(Self(Arc::new(bind)))
        } else {
            None
        }
    }
}

#[derive(Resource)]
pub struct CommonBindModel(pub BindModel);

#[derive(Component, Clone, Default)]
pub struct RecordIndiceRenderRange(pub IndiceRenderRange);
impl TAnimatableCompRecord<IndiceRenderRange> for RecordIndiceRenderRange {
    fn comp(&self) -> IndiceRenderRange {
        self.0.clone()
    }
}

#[derive(Component, Clone)]
pub struct IndiceRenderRange(pub Option<Range<u32>>);
impl IndiceRenderRange {
    pub fn new(val: Option<(u32, u32)>) -> Self {
        if let Some((start, end)) = val {
            Self(Some(Range { start, end }))
        } else {
            Self(None)
        }
    }
    pub fn apply(&self, geo: &RenderGeometry) -> Option<RenderIndices> {
        if let Some(mut indices) = geo.indices.clone() {
            if let Some(renderrange) = &self.0 {
                let range0 = indices.buffer.range();
                let mut start = renderrange.start as u64 * indices.format.use_bytes();
                let mut end = renderrange.end as u64 * indices.format.use_bytes();

                if let Some(range) = indices.buffer_range.as_ref() {
                    start = u64::min(range.end, range.start + start);
                    end = u64::min(range.end, range.start + end);
                } else {
                    let temp = range0.end - range0.start;
                    start = u64::min(temp, 0 + start);
                    end = u64::min(temp, 0 + end);
                }

                indices.buffer_range = Some(
                    Range { start, end }
                );
            }
            // log::warn!("IndiceRenderRange: {:?} buffer_range: {:?}", self, indices.buffer_range);
            
            Some(indices)
        } else {
            // log::warn!("IndiceRenderRange: {:?} buffer_range: None", self);
            None
        }
    }
}
impl Default for IndiceRenderRange {
    fn default() -> Self {
        Self(None)
    }
}
impl pi_curves::curve::frame::FrameDataValue for IndiceRenderRange {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        if amount < 0.5 {
            self.clone()
        } else {
            rhs.clone()
        }
    }

    fn hermite(value1: &Self, _tangent1: &Self, value2: &Self, _tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, _frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        if amount < 0.5 {
            value1.clone()
        } else {
            value2.clone()
        }
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        if amount < 0.5 {
            self.clone()
        } else {
            rhs.clone()
        }
    }
    fn size() -> usize {
        2 * 4
    }
}
impl TAssetCapacity for IndiceRenderRange {
    const ASSET_TYPE: &'static str = "AnimeCurveIndiceRenderRange";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 500 * 1024 , max: 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for IndiceRenderRange {

}

#[derive(Component, Clone)]
pub struct VertexRenderRange(pub Option<(u32, u32)>);
impl VertexRenderRange {
    pub fn apply(&self, geo: &RenderGeometry) -> Range<u32> {
        if let Some((start, count)) = &self.0 {
            let range0 = geo.vertex_range();
            let start = (*start + range0.start).min(range0.end);
            let end = (*count + start).min(range0.end);

            Range { start, end }
        } else {
            geo.vertex_range()
        }
    }
}
impl Default for VertexRenderRange {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Component)]
pub struct RenderMatrixDirty(pub bool);
impl TInstanceFlag for RenderMatrixDirty {
    fn dirty(&self) -> bool {
        self.0
    }

    fn reset(&mut self) {
        self.0 = false
    }
}

#[derive(Clone, Component)]
pub struct RenderWorldMatrix(pub Matrix);
impl RenderWorldMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}
impl TInstanceData for RenderWorldMatrix {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsWorldRow1
    }

    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];

        list.iter().for_each(|v| {
            bytemuck::cast_slice(v.0.as_slice()).iter().for_each(|v| {
                result.push(*v);
            })
        });

        result
    }

    // fn size() -> usize {
    //     16
    // }

    // fn bytes_size() -> usize {
    //     16 * 4
    // }

    // fn local_offset(&self) -> usize {
    //     0
    // }
}

#[derive(Clone, Component)]
pub struct RenderWorldMatrixInv(pub Matrix);
impl RenderWorldMatrixInv {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}

#[derive(Clone, Component)]
pub struct InstancedMeshTransparentSortCollection(pub Vec<(i32, Range<u32>)>);

// #[derive(Component, Default)]
// pub struct ModelSpotLightingDirty;

// #[derive(Component, Default)]
// pub struct ModelPointLightingDirty;

// #[derive(Component, Default)]
// pub struct ModelHemiLightingDirty;

// #[derive(Component)]
// pub struct ModelLightingInfosDirty;

#[derive(Component, Default)]
pub struct ModelForcePointLightings(pub Vec<Entity>);


#[derive(Component, Default)]
pub struct ModelForceSpotLightings(pub Vec<Entity>);


#[derive(Component, Default)]
pub struct ModelForceHemiLightings(pub Vec<Entity>);

#[derive(Component)]
pub struct ModelLightingIndexs {
    pub bind: Option<Arc<BindModelLightIndexs>>,
    pub count: u32,
}
impl ModelLightingIndexs {
    pub fn new(allocator: &mut BindBufferAllocator, lightlimit: &LightLimitInfo) -> Self {
        let data = if let Some(data) = BindModelLightIndexs::new(allocator, lightlimit.max_direct_light_count, lightlimit.max_point_light_count, lightlimit.max_spot_light_count, lightlimit.max_hemi_light_count) {
            Some(Arc::new(data))
        } else {
            None
        };
        Self { bind: data, count: 0 }
    }
}