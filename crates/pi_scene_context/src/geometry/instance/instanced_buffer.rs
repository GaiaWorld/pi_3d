
use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_render::rhi::buffer::Buffer;

#[derive(Component)]
pub struct InstancedInfo {
    pub state: u32,
    pub bytes_per_instance: u32,
    pub slot: EVertexBufferSlot,
}
impl InstancedInfo {
    pub fn new(state: u32, slot: EVertexBufferSlot) -> Self {
        Self {
            state,
            bytes_per_instance: InstanceState::bytes_per_instance(state),
            slot,
        }
    }
    pub fn attributes(&self) -> Vec<VertexAttribute> {
        InstanceState::attributes(self.state)
    }
    pub fn slot(&self) -> EVertexBufferSlot {
        self.slot
    }
    pub fn bytes_per_instance(&self) -> u32 {
        self.bytes_per_instance
    }
    pub fn geo_desc(&self) -> VertexBufferDesc {
        VertexBufferDesc {
            key: KeyVertexBuffer::from(""),
            range: None,
            attrs: self.attributes(),
            step_mode: wgpu::VertexStepMode::Instance,
            instance: true,
        }
    }
}

pub struct InstanceCacheBuffer {
    vertices: Vec<u8>,
    buffer: (Arc<NotUpdatableBufferRange>, u32, u32),
}

#[derive(Resource)]
pub struct InstanceBufferAllocator {
    list: Vec<InstanceCacheBuffer>,
    used_index: usize,
    /// 单个 Mesh 的实例化最多使用多少字节数据
    /// 当 运行时超过该数据时 对数据进行截取
    one_mesh_max_instance_bytes: u32,
}
impl InstanceBufferAllocator {
    pub fn check(&self, buffer: &Buffer) -> bool {
        let mut result = true;
        self.list.iter().for_each(|item| {
            if result {
                result = buffer.id() == item.buffer.0.buffer().id();
            }
        });
        return result;
    }
    pub fn new(one_mesh_max_instance_bytes: u32, allocator: &mut VertexBufferAllocator, device: &RenderDevice, queue: &RenderQueue) -> Self {
        let mut data = Vec::with_capacity(one_mesh_max_instance_bytes as usize);
        for _ in 0..one_mesh_max_instance_bytes {
            data.push(0);
        }
        let buffer = allocator.create_not_updatable_buffer_pre(device, queue, &data, None).unwrap();

        let first = InstanceCacheBuffer {
            vertices: vec![],
            buffer: (buffer, 0, one_mesh_max_instance_bytes),
        };
        Self {
            list: vec![first],
            used_index: 0,
            one_mesh_max_instance_bytes,
        }
    }
    pub fn instance_initial_buffer(&self) -> (Arc<NotUpdatableBufferRange>, u32, u32) {
        (self.list[0].buffer.0.clone(), 0, 0)
    }
    /// 默认都是 f32
    pub fn collect(&mut self, data: &[u8], bytes_per_instance: u32, allocator: &mut VertexBufferAllocator, device: &RenderDevice, queue: &RenderQueue) -> Option<(Arc<NotUpdatableBufferRange>, u32, u32)> {
        let max_count = self.one_mesh_max_instance_bytes / bytes_per_instance;
        let byte_size = data.len().min((max_count * bytes_per_instance) as usize);
        let bytes = &data[0..byte_size];

        if let Some(buffer) = self.list.get(self.used_index) {
            if buffer.vertices.len() + bytes.len() > buffer.buffer.2 as usize {
                self.used_index += 1;
            }
        };
        if let Some(buffer) = self.list.get_mut(self.used_index)  {
            let start = buffer.vertices.len();
            bytes.iter().for_each(|v| { buffer.vertices.push(*v) });
            return Some((buffer.buffer.0.clone(), start as u32, buffer.vertices.len() as u32));
        } else {
            let mut data = Vec::with_capacity(self.one_mesh_max_instance_bytes as usize);
            bytes.iter().for_each(|v| { data.push(*v) });
            let vertices = data.clone();

            for _ in byte_size..self.one_mesh_max_instance_bytes as usize {
                data.push(0);
            }
            if let Some(buffer) = allocator.create_not_updatable_buffer_pre(device, queue, &data, None) {
                self.list.push(InstanceCacheBuffer {
                    vertices,
                    buffer: (buffer, 0, self.one_mesh_max_instance_bytes as u32),
                    // key: KeyVertexBuffer::from(self.used_index.to_string().as_str()),
                });
                let buffer = self.list.get_mut(self.used_index).unwrap();
                return Some(
                    (buffer.buffer.0.clone(), 0, buffer.vertices.len() as u32)
                );
            } else {
                return None;
            }
        };
    }
    pub fn upload(&mut self, queue: &RenderQueue) {
        for idx in 0..(self.used_index + 1) {
            if let Some(buffer) = self.list.get_mut(idx) {
                if buffer.vertices.len() > 0 {
                    queue.write_buffer(buffer.buffer.0.buffer(), 0, &buffer.vertices);
                }
                buffer.vertices.clear();
            }
        }
        self.used_index = 0;
    }
}