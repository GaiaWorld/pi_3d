
use std::sync::Arc;

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
