use pi_ecs::prelude::Id;
use pi_render::rhi::{bind_group::BindGroup, pipeline::RenderPipeline};
use pi_share::Share;
use pi_slotmap::{SlotMap, DefaultKey};

use crate::{geometry::GBID, object::{GameObject, ObjectID}, flags::{ERenderMode, RenderSortParam}};

#[derive(Debug, Clone, Copy)]
pub struct RenderObjectID(pub ObjectID);

#[derive(Default)]
pub struct RenderObjectOpaqueList {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub draws: Vec<RenderObjectMetaOpaque>,
}
#[derive(Default)]
pub struct RenderObjectTransparentList {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub draws: Vec<RenderObjectMetaTransparent>,
}

/// wgpu 级别的渲染对象
/// * 在渲染图节点中使用
/// * 记录顶点数据 ID
/// * 记录 BindGroup ID
#[derive(Debug)]
pub struct RenderObjectMetaOpaque {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub pipeline: RenderObjectPipeline,
    pub positions: RenderObjectVertice,
    pub indices: Option<RenderObjectIndices>,
    pub vertices: Vec<RenderObjectVertice>,
    pub instances: Vec<RenderObjectInstance>,
    pub render_sort: RenderSortParam,
    pub view_distance: f32,
}
impl PartialEq for RenderObjectMetaOpaque {
    fn eq(&self, other: &Self) -> bool {
        self.pipeline == other.pipeline && self.render_sort == other.render_sort
    }
}
impl Eq for RenderObjectMetaOpaque {
    fn assert_receiver_is_total_eq(&self) {
        
    }
}
impl PartialOrd for RenderObjectMetaOpaque {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // match self.render_sort.partial_cmp(&other.render_sort) {
        //     Some(core::cmp::Ordering::Equal) => {}
        //     ord => return ord,
        // }
        match other.view_distance.partial_cmp(&self.view_distance) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.pipeline.partial_cmp(&other.pipeline)
    }
}
impl Ord for RenderObjectMetaOpaque {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


/// wgpu 级别的渲染对象
/// * 在渲染图节点中使用
/// * 记录顶点数据 ID
/// * 记录 BindGroup ID
#[derive(Debug)]
pub struct RenderObjectMetaTransparent {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub pipeline: RenderObjectPipeline,
    pub positions: RenderObjectVertice,
    pub indices: Option<RenderObjectIndices>,
    pub vertices: Vec<RenderObjectVertice>,
    pub instances: Vec<RenderObjectInstance>,
    pub render_sort: RenderSortParam,
}
impl PartialEq for RenderObjectMetaTransparent {
    fn eq(&self, other: &Self) -> bool {
        self.pipeline == other.pipeline && self.render_sort == other.render_sort
    }
}
impl Eq for RenderObjectMetaTransparent {
    fn assert_receiver_is_total_eq(&self) {
        
    }
}
impl PartialOrd for RenderObjectMetaTransparent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.render_sort.partial_cmp(&other.render_sort) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.pipeline.partial_cmp(&other.pipeline)
    }
}
impl Ord for RenderObjectMetaTransparent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


#[derive(Debug, Clone)]
pub struct RenderObjectBindGroup {
    pub bind_group: ObjectID,
    pub offsets: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderObjectPipeline {
    pub id: DefaultKey,
}

#[derive(Debug)]
pub struct RenderObjectVertice {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
}

#[derive(Debug)]
pub struct RenderObjectIndices {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
    pub format: wgpu::IndexFormat,
}

#[derive(Debug)]
pub struct RenderObjectInstance {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
}