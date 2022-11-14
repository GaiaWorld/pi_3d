use pi_slotmap::{DefaultKey};

use crate::{geometry::GBID, object::{ObjectID}, renderers::render_sort::{RenderSortParam}};

use super::pipeline::PipelineKey;

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
    pub pipeline: PipelineKey,
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
    pub pipeline: PipelineKey,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderObjectVertice {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderObjectIndices {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
    pub format: wgpu::IndexFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderObjectInstance {
    pub slot: u32,
    pub gbid: GBID,
    pub start: usize,
    pub end: usize,
    pub count: usize,
}

#[derive(Debug, Default)]
pub struct TempDrawInfoRecord {
    list: Vec<RenderObjectVertice>,
    indices: Option<RenderObjectIndices>,
}
impl TempDrawInfoRecord {
    pub fn record_vertex_and_check_diff_with_last(
        &mut self,
        vertex: &RenderObjectVertice,
    ) -> bool {
        if self.get(vertex.slot as usize) == vertex {
            return false;
        } else {
            self.list[vertex.slot as usize] = *vertex;
            return true;
        }
    }
    pub fn record_indices_and_check_diff_with_last(
        &mut self,
        indices: &RenderObjectIndices,
    ) -> bool {
        let result = match self.indices {
            Some(old) => {
                old != *indices
            },
            None => {
                true
            },
        };

        self.indices = Some(*indices);
        
        result
    }
    fn get(&mut self, slot: usize) -> &RenderObjectVertice {
        let oldlen = self.list.len();
        let mut addcount = 0;
        while oldlen + addcount <= slot {
            self.list.push(RenderObjectVertice {
                slot: slot as u32,
                gbid: GBID::default(),
                start: 0,
                end: 0,
                count: 0,
            });
            addcount += 1;
        }

        self.list.get(slot).unwrap()
    }
}