
use pi_scene_math::Vector4;
use pi_engine_shell::prelude::*;


#[derive(Component)]
pub struct InstanceTillOff(pub Vector4);
impl TInstanceData for InstanceTillOff {
    fn vertex_kind(&self) -> EVertexDataKind {
        todo!()
    }

    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];

        list.iter().for_each(|v| {
            bytemuck::cast_slice(v.0.as_slice()).iter().for_each(|v| {
                result.push(*v);
            });
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

#[derive(Component)]
pub struct InstanceBufferTillOff {
    pub slot: usize,
    pub index: IDAssetVertexBuffer,
    // buffer: Handle<EVertexBufferRange>,
}
impl TInstanceBuffer for InstanceBufferTillOff {
    fn display_name() -> String {
        String::from("InstanceBufferTillOff")
    }

    fn slot(&self) -> EVertexBufferSlot {
        EVertexBufferSlot::from_u8_unsafe(self.slot as u8)
    }

    fn id(&mut self) -> KeyVertexBuffer {
        // self.index += 1;
        // KeyVertexBuffer::from(self.id.clone() + self.index.to_string().as_str())
        KeyVertexBuffer::from(self.index)
    }
}

#[derive(Component)]
pub struct InstanceTillOffDirty(pub bool);
impl Default for InstanceTillOffDirty {
    fn default() -> Self {
        Self(false)
    }
}
impl TInstanceFlag for InstanceTillOffDirty {
    fn dirty(&self) -> bool {
        self.0
    }
    fn reset(&mut self) {
        self.0 = false;
    }
}