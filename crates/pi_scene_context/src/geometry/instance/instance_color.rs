
use pi_scene_math::{Vector4, Number};
use pi_engine_shell::prelude::*;

#[derive(Component)]
pub struct InstanceRGB(pub Number, pub Number, pub Number);

#[derive(Component)]
pub struct InstanceAlpha(pub Number);

#[derive(Component)]
pub struct InstanceColor(pub Vector4);
impl Default for InstanceColor {
    fn default() -> Self {
        Self(Vector4::new(1., 1., 1., 1.))
    }
}
impl TInstanceData for InstanceColor {
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

#[derive(Component)]
pub struct InstanceBufferColor {
    pub slot: usize,
    pub index: KeyVertexBuffer,
    // buffer: Handle<EVertexBufferRange>,
}
impl TInstanceBuffer for InstanceBufferColor {
    fn display_name() -> String {
        String::from("InstanceBufferColor")
    }

    fn slot(&self) -> EVertexBufferSlot {
        EVertexBufferSlot::from_u8_unsafe(self.slot as u8)
    }

    fn id(&mut self) -> KeyVertexBuffer {
        // self.index += 1;
        // KeyVertexBuffer::from(self.id.clone() + self.index.to_string().as_str())
        // KeyVertexBuffer::from(self.index)
        self.index.clone()
    }
}


#[derive(Component)]
pub struct InstanceColorDirty(pub bool);
impl Default for InstanceColorDirty {
    fn default() -> Self {
        Self(false)
    }
}
impl TInstanceFlag for InstanceColorDirty {
    fn dirty(&self) -> bool {
        self.0
    }
    fn reset(&mut self) {
        self.0 = false;
    }
}