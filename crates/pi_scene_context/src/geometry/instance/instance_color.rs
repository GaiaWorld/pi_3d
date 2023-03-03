
use pi_render::{renderer::{attributes::EVertexDataKind, vertex_buffer::{KeyVertexBuffer}, instance::{types::{TInstancedData, TInstanceFlag}, instanced_buffer::TInstancedBuffer}, vertex_buffer_desc::EVertexBufferSlot}, rhi::{device::RenderDevice, RenderQueue}};
use pi_scene_math::Vector4;


pub struct InstanceColor(pub Vector4);
impl TInstancedData for InstanceColor {
    fn vertex_kind(&self) -> EVertexDataKind {
        todo!()
    }

    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];

        list.iter().for_each(|v| {
            bytemuck::cast_slice(v.0.as_slice()).iter().for_each(|v| {
                result.push(*v);
            })
        });

        result
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