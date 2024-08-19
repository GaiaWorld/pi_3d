
use std::{ops::Range, sync::Arc};
use pi_scene_shell::prelude::*;
pub use pi_scene_shell::prelude::InstanceCacheBuffer;

use crate::prelude::TmpInstanceSort;

#[derive(Component, Default)]
pub struct InstancedInfoComp(pub Option<InstancedInfo>);

pub struct InstancedInfo {
    // pub state: u32,
    pub bytes_per_instance: u32,
    pub slot: EVertexBufferSlot,
}
impl InstancedInfo {
    pub fn new(bytes_per_instance: u32, slot: EVertexBufferSlot) -> Self {
        Self {
            // state,
            bytes_per_instance,
            slot,
        }
    }
    // pub fn attributes(&self) -> Vec<VertexAttribute> {
    //     InstanceState::attributes(self.state)
    // }
    pub fn slot(&self) -> EVertexBufferSlot {
        self.slot
    }
    pub fn bytes_per_instance(&self) -> u32 {
        self.bytes_per_instance
    }
    // pub fn geo_desc(&self) -> VertexBufferDesc {
    //     VertexBufferDesc::new( KeyVertexBuffer::from(""), VertexBufferDescRange::default(), self.attributes(), true)
    // }
}

#[derive(Resource)]
pub struct ArgInstanceBufferAllocatorSize(pub u32);

#[derive(Resource)]
pub struct InstanceBufferAllocator(PiInstanceBufferAllocator);
impl InstanceBufferAllocator {
    pub fn one_mesh_max_instance_bytes(&self) -> usize {
        self.0.one_mesh_max_instance_bytes()
    }
    pub fn check(&self, buffer: &Buffer) -> bool {
        self.0.check(buffer)
    }
    pub fn new(one_mesh_max_instance_bytes: u32, allocator: &mut VertexBufferAllocator, device: &RenderDevice, queue: &RenderQueue) -> Self {
        Self(PiInstanceBufferAllocator::new(one_mesh_max_instance_bytes, allocator, device, queue))
    }
    pub fn instance_initial_buffer(&self) -> (Arc<NotUpdatableBufferRange>, u32, u32) {
        self.0.instance_initial_buffer()
    }
    /// 默认都是 f32
    pub fn collect(&mut self, data: &[u8], bytes_per_instance: u32, allocator: &mut VertexBufferAllocator, device: &RenderDevice, queue: &RenderQueue) -> Option<(Arc<NotUpdatableBufferRange>, u32, u32)> {
        self.0.collect(data, bytes_per_instance, allocator, device, queue)
    }
    pub fn upload(&mut self, queue: &RenderQueue) {
        self.0.upload(queue)
    }
}

pub struct InstanceDataRef {

}

pub struct DataPool {
    vec: Vec<u8>,
    used: usize,
}
impl DataPool {
    pub fn new(initmax: usize) -> Self {
        Self {
            vec: Vec::with_capacity(initmax),
            used: 0,
        }
    }
    pub fn usedsize(&self) -> usize {
        self.used
    }
    pub fn size(&self) -> usize {
        self.vec.capacity()
    }
    pub fn reset(&mut self) {
        // self.vec.clear();
        self.used = 0;
    }
    pub fn record(&mut self, data: &[u8]) -> Range<usize> {
        let start = self.used;
        let end = self.used + data.len();

        let mutlen = (self.vec.len() - self.used).min(data.len());
        for idx in 0..mutlen{
            self.vec[self.used + idx] = data[idx];
        }

        let pushlen = data.len() - mutlen;
        for idx in 0..pushlen {
            self.vec.push(data[mutlen + idx]);
        }

        self.used += data.len();

        Range { start, end }
    }
    pub fn data(&self, range: &Range<usize>) -> &[u8] {
        &self.vec.as_slice()[range.start..range.end]
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct InstanceDataCommon(DataPool);
impl InstanceDataCommon {
    pub fn new(initmax: usize) -> Self {
        Self(DataPool::new(initmax))
    }
    pub fn size(&self) -> usize {
        self.0.size()
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct CombineDataCommon(DataPool);
impl CombineDataCommon {
    pub fn new(initmax: usize) -> Self {
        Self(DataPool::new(initmax))
    }
    pub fn size(&self) -> usize {
        self.0.size()
    }
}
