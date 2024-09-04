
use std::{ops::Range, sync::Arc};
use pi_scene_shell::prelude::*;
pub use pi_scene_shell::prelude::InstanceCacheBuffer;

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
        let mut vec = Vec::with_capacity(initmax);
        for _ in 0..initmax {
            vec.push(0);
        }
        Self {
            vec,
            used: 0,
        }
    }
    pub fn usedsize(&self) -> usize {
        self.used
    }
    pub fn size(&self) -> usize {
        self.vec.capacity()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn reset(&mut self) {
        // self.vec.clear();
        self.used = 0;
    }
    pub fn record(&mut self, data: &[u8]) -> Range<usize> {
        let start = self.used;
        let end = self.used + data.len();

        let mut pushlen = data.len();
        let mut mutlen = 0;
        if self.vec.len() > self.used {
            mutlen = (self.vec.len() - self.used).min(data.len());
            for idx in 0..mutlen{
                self.vec[self.used + idx] = data[idx];
            }
            pushlen = data.len() - mutlen;
        }
    
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
pub struct CombineDataCommon(DataPool);
impl CombineDataCommon {
    pub fn new(initmax: usize) -> Self {
        Self(DataPool::new(initmax))
    }
    pub fn size(&self) -> usize {
        self.0.size()
    }
}

#[derive(Resource)]
pub struct CombineBuffer {
    pub buffers: Option<Arc<NotUpdatableBufferRange>>,
    pub data: DataPool,
    initmax: usize,
    pub maxcombinesize: usize,
}
impl CombineBuffer {
    pub fn new(initmax: usize, allocator: &mut VertexBufferAllocator3D, device: &PiRenderDevice, queue: &PiRenderQueue) -> Self {
        let data = DataPool::new(initmax);
        let buffer = allocator.create_not_updatable_buffer_pre(device, queue, data.vec.as_slice(), None);
        Self {
            buffers: buffer,
            initmax,
            data: DataPool::new(initmax),
            maxcombinesize: initmax,
        }
    }
    pub fn combinecommon(&mut self, requestsize: usize) -> bool {
        if self.data.used < self.initmax {
            let unuselen = self.initmax - self.data.used;
            // if requestsize > unuselen {
            //     let mut temp = Vec::with_capacity(unuselen);
            //     for _ in 0..unuselen {
            //         temp.push(0);
            //     }
            //     self.record(&temp);
            // }
            requestsize <= unuselen
        } else {
           true
        }
    }
    pub fn size(&self) -> usize {
        self.data.size()
    }
    pub fn usedsize(&self) -> usize {
        self.data.used
    }
    pub fn reset(&mut self) {
        // self.vec.clear();
        self.data.reset();
    }
    pub fn record(&mut self, data: &[u8]) -> Range<usize> {
        // if self.data.used < self.initmax {
        //     let unuselen = self.initmax - self.data.used;
        //     if unuselen < data.len() {
        //         let mut temp = Vec::with_capacity(unuselen);
        //         for _ in 0..unuselen {
        //             temp.push(0);
        //         }
        //         self.data.record(&temp);
        //     }
        // }
        self.data.record(data)
    }
    pub fn data(&self, range: &Range<usize>, allocator: &mut VertexBufferAllocator3D, device: &PiRenderDevice, queue: &PiRenderQueue) -> Option<EVertexBufferRange> {
        if range.start < range.end {
            if range.end <= self.initmax {
                if let Some(buffer) = &self.buffers {
                    Some(EVertexBufferRange::NotUpdatable(buffer.clone(), range.start as u32, range.end as u32))
                } else {
                    allocator.create_not_updatable_buffer(device, queue, self.data.data(range), None)
                }
            } else {
                allocator.create_not_updatable_buffer(device, queue, self.data.data(range), None)
            }
        } else {
            None
        }
    }
    pub fn apply(&mut self, queue: &PiRenderQueue) {
        if self.data.used > 0 {
            if let Some(buffer) = &self.buffers {
                let mut range = Range { start: 0, end: self.initmax.min(self.data.used) };
                let size = wgpu::COPY_BUFFER_ALIGNMENT as usize;
                let temp = (range.end / size) * size;
                if temp < range.end { range.end = temp + size; }
                queue.write_buffer(buffer.buffer(), 0, self.data.data(&range));
            }
        }
        self.data.reset();
    }
}
