use std::{ops::Range, sync::Arc, u32};

use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;
use pi_scene_math::{Matrix, Vector3};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageModel {
    MeshCreate,
    _InitMesh,
    InstanceCreate,
    _InitInstance,
    AbstructMeshCommand,
    InstanceEffectMesh,
    RenderMatrix,
    InstanceEffectGeometry,
    LightingCollect,
    MeshDispose,
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

/// 标识实体类型 Mesh , 与 IntancedMesh 有区别, 两者都是 AbstructMesh
#[derive(Component, Default)]
pub struct Mesh;

/// 在 AbstructMesh 实体上 可能设置的 PoseMatrix
/// 用于 调整目标渲染姿态 , 与节点树世界矩阵 有区别
#[derive(Component)]
pub struct RenderPoseMatrix(pub Matrix);
impl Default for RenderPoseMatrix {
    fn default() -> Self {
        Self(Matrix::identity())
    }
}

// #[derive(Component, Default)]
// /// InstancedMesh 的 AlphaIndex
// pub struct InstanceTransparentIndex(pub i32);

/// 用于标识 AbstructMesh 是否通过视口剔除
#[derive(Component, Default)]
pub struct AbstructMeshCullingFlag(pub bool);

/// 用于表达 Mesh 的实例化数据描述
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

/// 用于控制 Mesh 的渲染状态对齐模式控制(ERenderAlignment)
#[derive(Clone, Component, Deref, DerefMut)]
pub struct RenderAlignment(pub ERenderAlignment);
impl Default for RenderAlignment {
    fn default() -> Self {
        Self(ERenderAlignment::Local)
    }
}

/// 用于控制 Mesh 的渲染状态缩放模式控制(EScalingMode)
#[derive(Clone, Component, Default, Deref, DerefMut)]
pub struct ScalingMode(pub EScalingMode);

/// 用于记录 AbstructMesh 的全局速度向量
#[derive(Clone, Component, Deref, DerefMut)]
pub struct ModelVelocity(pub Vector3);
impl Default for ModelVelocity {
    fn default() -> Self {
        Self(Vector3::new(0., 1., 0.))
    }
}

/// 用于标识 Mesh 是否为静态目标
#[derive(Component, Clone)]
pub struct ModelStatic(pub bool);
impl ModelStatic {
    pub fn default() -> Self {
        Self(false)
    }
}

/// 用于记录 Mesh 的模型相关Uniform数据
/// 包含 Mesh 的渲染矩阵, 骨骼绑定, 关联灯光等等信息
#[derive(Component, Default, Clone)]
pub struct BindModel(pub Option<ShaderBindModelAboutMatrix>);
impl BindModel {
    pub fn new(
        allocator: &mut BindBufferAllocator,
    ) -> Self {
        Self(ShaderBindModelAboutMatrix::new(allocator))
    }
}

/// 用于记录 Mesh 的模型 材质数据Index
#[derive(Component, Default, Clone)]
pub struct BindModelMatIdx(pub Option<ShaderBindModelMatIdx>);
impl BindModelMatIdx {
    pub fn new(
        allocator: &mut BindBufferAllocator,
    ) -> Self {
        Self(ShaderBindModelMatIdx::new(allocator))
    }
}

#[derive(Component, Clone)]
pub struct ModelMatIdxs(pub [u16; PassTag::PASS_COUNT]);
impl Default for ModelMatIdxs {
    fn default() -> Self {
        Self([0, 0, 0, 0, 0, 0, 0, 0])
    }
}

/// 通用的一个BindModel,可用于多个粒子系统共用,增加合批机会
/// 实例化渲染中 BindModel 上的矩形数据并不会使用
#[derive(Resource)]
pub struct CommonBindModel(pub BindModel, pub BindModelMatIdx);
impl MemSize for CommonBindModel {
    fn memsize(&self) -> usize {
        1024
    }
}

/// 用于记录 Mesh 的可渲染顶点范围(当使用 Indices 时)
#[derive(Component, Clone)]
pub struct IndiceRenderRange(Range<u32>);
impl IndiceRenderRange {
    pub fn new(val: Option<(u32, u32)>) -> Self {
        if let Some((start, end)) = val {
            Self(Range { start, end })
        } else {
            Self(Range { start: u32::MAX, end: 0 })
        }
    }
    pub fn apply(&self, geo: &RenderGeometry) -> Option<RenderIndices> {
        if let Some(mut indices) = geo.indices.clone() {
            let renderrange = &self.0;
            if renderrange.start < u32::MAX {
                let range0 = indices.buffer.range();
                let mut start = renderrange.start as u64 * indices.format.use_bytes();
                let mut end = renderrange.end as u64 * indices.format.use_bytes();

                if let Some(range) = indices.buffer_range.as_ref() {
                    start = u64::min(range.end as u64, range.start as u64 + start);
                    end = u64::min(range.end as u64, range.start as u64 + end);
                } else {
                    let temp = range0.end - range0.start;
                    start = u64::min(temp, 0 + start);
                    end = u64::min(temp, 0 + end);
                }

                indices.buffer_range = Some(
                    Range { start: start as u32, end: end as u32 }
                );
            }
            // log::warn!("IndiceRenderRange: {:?} buffer_range: {:?}", self, indices.buffer_range);
            
            Some(indices)
        } else {
            // log::warn!("IndiceRenderRange: {:?} buffer_range: None", self);
            None
        }
    }
    pub fn is_some(&self) -> bool {
        self.0.start < u32::MAX
    }
}
impl Default for IndiceRenderRange {
    fn default() -> Self {
        Self(Range { start: u32::MAX, end: 0 })
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
        AssetCapacity { flag: false, min: 500 * 1024 , max: 1, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for IndiceRenderRange {

}

/// 用于记录 Mesh 的可渲染顶点范围
#[derive(Component, Clone)]
pub struct VertexRenderRange(u32, u32);
impl VertexRenderRange {
    pub fn new(val: Option<(u32, u32)>) -> Self {
        if let Some((start, end)) = val {
            Self(start, end)
        } else {
            Self(u32::MAX, 0)
        }
    }
    pub fn apply(&self, geo: &RenderGeometry) -> Range<u32> {
        let start = self.0;
        let count = self.1;
        if start < u32::MAX {
            let range0 = geo.vertex_range();
            let start = (start + range0.start).min(range0.end);
            let end = (count + start).min(range0.end);

            Range { start, end }
        } else {
            geo.vertex_range()
        }
    }
    pub fn is_some(&self) -> bool {
        self.0 < u32::MAX
    }
}
impl Default for VertexRenderRange {
    fn default() -> Self {
        Self(u32::MAX, 0)
    }
}

/// 用于标识 AbstructMesh 的渲染矩阵需要重新计算
#[derive(Component, Default)]
pub struct FlagRenderWorldMatrix;

/// 用于记录 AbstructMesh 的渲染矩阵
#[derive(Clone, Component, Default)]
pub struct RenderWorldMatrix(pub Matrix);

/// 用于记录 Mesh 的实例的排序后实例数据
#[derive(Clone, Component, Default)]
pub struct InstancedMeshTransparentSortCollection {
    pub ranges: Vec<(i32, Range<u32>, (Number, Number, Number))>,
    pub data: Vec<u8>,
    pub count: u32,
    pub sizeperinstance: u16,
    pub use_single_instancebuffer: bool,
}
impl InstancedMeshTransparentSortCollection {
    pub fn reset(&mut self) {
        self.ranges.clear();
        self.data.clear();
        self.sizeperinstance = 0;
    }
}

/// 用于记录 Mesh 受哪些 Light 的强制影响
#[derive(Component, Default)]
pub struct ModelForceLightings {
    pub point: Vec<Entity>,
    pub spot: Vec<Entity>,
    pub hemi: Vec<Entity>,
}

/// 用于记录 Mesh 关联灯光的 Uniform 数据
#[derive(Component, Default)]
pub struct ModelLightingIndexs {
    pub bind: Option<BindModelLightIndexs>,
    pub count: u32,
}
impl ModelLightingIndexs {
    pub fn new(allocator: &mut BindBufferAllocator, lightlimit: &LightLimitInfo) -> Self {
        let data = BindModelLightIndexs::new(allocator, lightlimit.max_direct_light_count, lightlimit.max_point_light_count, lightlimit.max_spot_light_count, lightlimit.max_hemi_light_count);
        Self { bind: data, count: 0 }
    }
}