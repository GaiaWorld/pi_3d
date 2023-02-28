use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_render::{renderer::{attributes::EVertexDataKind, vertex_buffer::{EVertexBufferRange, KeyVertexBuffer, VertexBufferAllocator}, instance::{types::{TInstancedData, TInstanceFlag}, instanced_buffer::TInstancedBuffer}, vertex_buffer_desc::EVertexBufferSlot}, rhi::{device::RenderDevice, RenderQueue}};
use pi_scene_math::Vector4;
use pi_share::Share;

use crate::geometry::vertex_buffer_useinfo;


pub struct InstanceColor(pub Vector4);
impl TInstancedData for InstanceColor {
    fn vertex_kind(&self) -> EVertexDataKind {
        todo!()
    }

    fn collect(list: &Vec<&Self>, key: KeyVertexBuffer, device: &RenderDevice, queue: &RenderQueue, allocator: &mut VertexBufferAllocator, asset_mgr: &Share<AssetMgr<EVertexBufferRange>>) -> Option<Handle<EVertexBufferRange>> {
        let mut result = vec![];

        list.iter().for_each(|v| {
            v.0.as_slice().iter().for_each(|v| {
                result.push(*v);
            })
        });

        if let Some(buffer) = allocator.create_not_updatable_buffer(device, queue, bytemuck::cast_slice(&result)) {
            asset_mgr.insert(key, buffer)
        } else {
            None
        }
    }

    // fn size() -> usize {
    //     4
    // }
    // fn bytes_size() -> usize {
    //     4 * 4
    // }

    // fn local_offset(&self) -> usize {
    //     0
    // }

}

pub struct InstancedBufferColor {
    pub slot: usize,
    pub id: String,
    pub index: usize,
    // buffer: Handle<EVertexBufferRange>,
}
impl TInstancedBuffer for InstancedBufferColor {
    fn display_name() -> String {
        String::from("InstancedBufferColor")
    }

    fn slot(&self) -> EVertexBufferSlot {
        EVertexBufferSlot::from_u8_unsafe(self.slot as u8)
    }

    fn id(&mut self) -> KeyVertexBuffer {
        self.index += 1;
        KeyVertexBuffer::from(self.id.clone() + self.index.to_string().as_str())
    }
}

pub struct InstancedColorDirty(pub bool);
impl TInstanceFlag for InstancedColorDirty {
    fn dirty(&self) -> bool {
        self.0
    }
    fn reset(&mut self) {
        self.0 = false;
    }
}