use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Number};


#[derive(Component)]
pub struct InstanceVec4A(pub Number, pub Number, pub Number, pub Number);
impl Default for InstanceVec4A { fn default() -> Self { Self(0., 0., 0., 0.) } }
impl TInstanceData for InstanceVec4A {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsCustomVec4A
    }
    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];
        list.iter().for_each(|v| {
            bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { result.push(*v); })
        });
        result
    }
}
#[derive(Component)]
pub struct InstanceBufferVec4A {
    pub slot: usize,
    pub index: KeyVertexBuffer,
}
impl TInstanceBuffer for InstanceBufferVec4A {
    fn display_name() -> String { String::from("InstanceBufferVec4A") }
    fn slot(&self) -> EVertexBufferSlot { EVertexBufferSlot::from_u8_unsafe(self.slot as u8) }
    fn id(&mut self) -> KeyVertexBuffer { self.index.clone() }
}
#[derive(Component)]
pub struct InstanceVec4ADirty(pub bool);
impl Default for InstanceVec4ADirty { fn default() -> Self { Self(false) } }
impl TInstanceFlag for InstanceVec4ADirty {
    fn dirty(&self) -> bool { self.0 }
    fn reset(&mut self) { self.0 = false; }
}

#[derive(Component)]
pub struct InstanceVec4B(pub Number, pub Number, pub Number, pub Number);
impl Default for InstanceVec4B { fn default() -> Self { Self(0., 0., 0., 0.) } }
impl TInstanceData for InstanceVec4B {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsCustomVec4B
    }
    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];
        list.iter().for_each(|v| {
            bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { result.push(*v); })
        });
        result
    }
}
#[derive(Component)]
pub struct InstanceBufferVec4B {
    pub slot: usize,
    pub index: KeyVertexBuffer,
}
impl TInstanceBuffer for InstanceBufferVec4B {
    fn display_name() -> String { String::from("InstanceBufferVec4B") }
    fn slot(&self) -> EVertexBufferSlot { EVertexBufferSlot::from_u8_unsafe(self.slot as u8) }
    fn id(&mut self) -> KeyVertexBuffer { self.index.clone() }
}
#[derive(Component)]
pub struct InstanceVec4BDirty(pub bool);
impl Default for InstanceVec4BDirty { fn default() -> Self { Self(false) } }
impl TInstanceFlag for InstanceVec4BDirty {
    fn dirty(&self) -> bool { self.0 }
    fn reset(&mut self) { self.0 = false; }
}

#[derive(Component)]
pub struct InstanceVec4C(pub Number, pub Number, pub Number, pub Number);
impl Default for InstanceVec4C { fn default() -> Self { Self(0., 0., 0., 0.) } }
impl TInstanceData for InstanceVec4C {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsCustomVec4C
    }
    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];
        list.iter().for_each(|v| {
            bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { result.push(*v); })
        });
        result
    }
}
#[derive(Component)]
pub struct InstanceBufferVec4C {
    pub slot: usize,
    pub index: KeyVertexBuffer,
}
impl TInstanceBuffer for InstanceBufferVec4C {
    fn display_name() -> String { String::from("InstanceBufferVec4C") }
    fn slot(&self) -> EVertexBufferSlot { EVertexBufferSlot::from_u8_unsafe(self.slot as u8) }
    fn id(&mut self) -> KeyVertexBuffer { self.index.clone() }
}
#[derive(Component)]
pub struct InstanceVec4CDirty(pub bool);
impl Default for InstanceVec4CDirty { fn default() -> Self { Self(false) } }
impl TInstanceFlag for InstanceVec4CDirty {
    fn dirty(&self) -> bool { self.0 }
    fn reset(&mut self) { self.0 = false; }
}

#[derive(Component)]
pub struct InstanceVec4D(pub Number, pub Number, pub Number, pub Number);
impl Default for InstanceVec4D { fn default() -> Self { Self(0., 0., 0., 0.) } }
impl TInstanceData for InstanceVec4D {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsCustomVec4D
    }
    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];
        list.iter().for_each(|v| {
            bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { result.push(*v); })
        });
        result
    }
}
#[derive(Component)]
pub struct InstanceBufferVec4D {
    pub slot: usize,
    pub index: KeyVertexBuffer,
}
impl TInstanceBuffer for InstanceBufferVec4D {
    fn display_name() -> String { String::from("InstanceBufferVec4D") }
    fn slot(&self) -> EVertexBufferSlot { EVertexBufferSlot::from_u8_unsafe(self.slot as u8) }
    fn id(&mut self) -> KeyVertexBuffer { self.index.clone() }
}
#[derive(Component)]
pub struct InstanceVec4DDirty(pub bool);
impl Default for InstanceVec4DDirty { fn default() -> Self { Self(false) } }
impl TInstanceFlag for InstanceVec4DDirty {
    fn dirty(&self) -> bool { self.0 }
    fn reset(&mut self) { self.0 = false; }
}