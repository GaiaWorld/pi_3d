use pi_assets::asset::Handle;
use render_data_container::{VertexBuffer, RenderVertices, RenderIndices};

use crate::{
    object::{ObjectID},
    renderers::render_sort::{RenderSortParam},
    bindgroup::{RenderBindGroupKey}
};

use super::pipeline::{ResRenderPipeline};
pub use super::render_object_list::*;

#[derive(Debug, Clone, Copy)]
pub struct RendererID(pub ObjectID);

pub trait DrawObject {
    fn bind_groups(&self) -> &Vec<RenderObjectBindGroup>;
    fn pipeline(&self) -> &wgpu::RenderPipeline;
    fn indices(&self) -> &Option<RenderIndices>;
    fn vertices(&self) -> &Vec<RenderVertices>;
    fn instances(&self) -> &Vec<RenderVertices>;
}

/// wgpu 级别的渲染对象
/// * 在渲染图节点中使用
/// * 记录顶点数据 ID
/// * 记录 BindGroup ID
#[derive(Debug)]
pub struct RenderObjectMetaOpaque {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub pipeline: Handle<ResRenderPipeline>,
    pub indices: Option<RenderIndices>,
    pub vertices: Vec<RenderVertices>,
    pub instances: Vec<RenderVertices>,
    pub render_sort: RenderSortParam,
    pub view_distance: f32,
}
impl PartialEq for RenderObjectMetaOpaque {
    fn eq(&self, other: &Self) -> bool {
        self.pipeline.key() == other.pipeline.key() && self.render_sort == other.render_sort
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
        self.pipeline.key().partial_cmp(&other.pipeline.key())
    }
}
impl Ord for RenderObjectMetaOpaque {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl DrawObject for RenderObjectMetaOpaque {
    fn bind_groups(&self) -> &Vec<RenderObjectBindGroup> {
        &self.bind_groups
    }

    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline.0
    }

    fn indices(&self) -> &Option<RenderIndices> {
        &self.indices
    }

    fn vertices(&self) -> &Vec<RenderVertices> {
        &self.vertices
    }

    fn instances(&self) -> &Vec<RenderVertices> {
        &self.instances
    }
}


/// wgpu 级别的渲染对象
/// * 在渲染图节点中使用
/// * 记录顶点数据 ID
/// * 记录 BindGroup ID
#[derive(Debug)]
pub struct RenderObjectMetaTransparent {
    pub bind_groups: Vec<RenderObjectBindGroup>,
    pub pipeline: Handle<ResRenderPipeline>,
    pub indices: Option<RenderIndices>,
    pub vertices: Vec<RenderVertices>,
    pub instances: Vec<RenderVertices>,
    pub render_sort: RenderSortParam,
}
impl PartialEq for RenderObjectMetaTransparent {
    fn eq(&self, other: &Self) -> bool {
        self.pipeline.key() == other.pipeline.key() && self.render_sort == other.render_sort
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
        self.pipeline.key().partial_cmp(&other.pipeline.key())
    }
}
impl Ord for RenderObjectMetaTransparent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl DrawObject for RenderObjectMetaTransparent {
    fn bind_groups(&self) -> &Vec<RenderObjectBindGroup> {
        &self.bind_groups
    }

    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline.0
    }

    fn indices(&self) -> &Option<RenderIndices> {
        &self.indices
    }

    fn vertices(&self) -> &Vec<RenderVertices> {
        &self.vertices
    }

    fn instances(&self) -> &Vec<RenderVertices> {
        &self.instances
    }
}


#[derive(Debug, Clone)]
pub struct RenderObjectBindGroup {
    pub bind_group: RenderBindGroupKey,
    pub offsets: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct RenderObjectVertice {
    pub slot: u32,
    pub gbid: Handle<VertexBuffer>,
    pub start: usize,
    pub end: usize,
    pub count: usize,
}
impl PartialEq for RenderObjectVertice {
    fn eq(&self, other: &Self) -> bool {
        self.gbid.key() == other.gbid.key()
    }
}
impl Eq for RenderObjectVertice {
    fn assert_receiver_is_total_eq(&self) {

    }
}

#[derive(Debug, Clone)]
pub struct RenderObjectIndices {
    pub slot: u32,
    pub gbid: Handle<VertexBuffer>,
    pub start: usize,
    pub end: usize,
    pub count: usize,
    pub format: wgpu::IndexFormat,
}
impl PartialEq for RenderObjectIndices {
    fn eq(&self, other: &Self) -> bool {
        self.gbid.key() == other.gbid.key()
    }
}
impl Eq for RenderObjectIndices {
    fn assert_receiver_is_total_eq(&self) {

    }
}

#[derive(Debug, Clone)]
pub struct RenderObjectInstance {
    pub slot: u32,
    pub gbid: Handle<VertexBuffer>,
    pub start: usize,
    pub end: usize,
    pub count: usize,
}
impl PartialEq for RenderObjectInstance {
    fn eq(&self, other: &Self) -> bool {
        self.gbid.key() == other.gbid.key()
    }
}
impl Eq for RenderObjectInstance {
    fn assert_receiver_is_total_eq(&self) {

    }
}

#[derive(Debug, Default)]
pub struct TempDrawInfoRecord {
    list: Vec<Option<RenderVertices>>,
    indices: Option<RenderIndices>,
}
impl TempDrawInfoRecord {
    pub fn record_vertex_and_check_diff_with_last(
        &mut self,
        vertex: &RenderVertices,
    ) -> bool {
        if let Some(save) = self.get(vertex.slot as usize) {
            if save == vertex {
                return false;
            } else {
                self.list[vertex.slot as usize] = Some(vertex.clone());
                return true;
            }
        } else {
            self.list[vertex.slot as usize] = Some(vertex.clone());
            return true;
        }
    }
    pub fn record_indices_and_check_diff_with_last(
        &mut self,
        indices: &RenderIndices,
    ) -> bool {
        let result = match &self.indices {
            Some(old) => {
                old != indices
            },
            None => {
                true
            },
        };

        self.indices = Some(indices.clone());
        
        result
    }
    fn get(&mut self, slot: usize) -> Option<&RenderVertices> {
        let oldlen = self.list.len();
        let mut addcount = 0;
        while oldlen + addcount <= slot {
            self.list.push(None);
            addcount += 1;
        }

        self.list.get(slot).unwrap().as_ref()
    }
}