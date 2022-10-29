use pi_render::rhi::{bind_group::BindGroup, pipeline::RenderPipeline};
use pi_share::Share;
use pi_slotmap::{SlotMap, DefaultKey};
use render_data_container::GeometryBufferPool;

use crate::geometry::GBID;

pub mod pipeline;

#[derive(Debug, Default)]
pub struct SingleRenderBindGroupPool {
    pub map: SlotMap<DefaultKey, BindGroup>,
}

#[derive(Debug, Default)]
pub struct SingleRenderObjectPipelinePool {
    pub map: SlotMap<DefaultKey, RenderPipeline>,
}

#[derive(Debug, Default)]
pub struct SingleGeometryBufferPool {
    list: SlotMap<GBID, render_data_container::GeometryBuffer>,
}
impl GeometryBufferPool<GBID> for SingleGeometryBufferPool {
    fn insert(&mut self, data: render_data_container::GeometryBuffer) -> GBID {
        self.list.insert(data)
    }

    fn remove(&mut self, key: &GBID) -> Option<render_data_container::GeometryBuffer> {
        self.list.remove(*key)
    }

    fn get(&self, key: &GBID) -> Option<&render_data_container::GeometryBuffer> {
        self.list.get(*key)
    }

    fn get_size(&self, key: &GBID) -> usize {
        match self.list.get(*key) {
            Some(geo) => geo.size(),
            None => 0,
        }
    }

    fn get_mut(&mut self, key: &GBID) -> Option<&mut render_data_container::GeometryBuffer> {
        self.list.get_mut(*key)
    }

    fn get_buffer(&self, key: &GBID) -> Option<Share<&wgpu::Buffer>> {
        match self.list.get(*key) {
            Some(geo) => geo.get_buffer(),
            None => None,
        }
    }
}