use pi_ecs::prelude::Id;
use pi_render::rhi::{bind_group::BindGroup, pipeline::RenderPipeline};
use pi_share::Share;
use pi_slotmap::{SlotMap, DefaultKey};

use crate::{geometry::GBID, object::{GameObject, ObjectID}};

#[derive(Debug)]
pub struct RenderObjectID(pub ObjectID);

#[derive(Default)]
pub struct RenderObjectOpaqueList {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub draws: Vec<RenderObjectMeta>,
}
#[derive(Default)]
pub struct RenderObjectTransparentList {
    pub draws: Vec<RenderObjectMeta>,
}

/// wgpu 级别的渲染对象
/// * 在渲染图节点中使用
/// * 记录顶点数据 ID
/// * 记录 BindGroup ID
pub struct RenderObjectMeta {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub pipeline: RenderObjectPipeline,
    pub positions: RenderObjectVertice,
    pub indices: Option<RenderObjectIndices>,
    pub vertices: Vec<RenderObjectVertice>,
    pub instances: Vec<RenderObjectInstance>,
}

#[derive(Debug, Clone)]
pub struct RenderObjectBindGroup {
    pub value: BindGroup,
    pub set: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderObjectPipeline {
    pub id: DefaultKey,
}

pub struct RenderObjectVertice {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
}
pub struct RenderObjectIndices {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
    pub format: wgpu::IndexFormat,
}
pub struct RenderObjectInstance {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
}