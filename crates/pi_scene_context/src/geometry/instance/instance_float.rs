use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Number};

pub type InstanceFloatType = u16;

pub struct EInstanceFloatType;
impl EInstanceFloatType {
    pub const F00: InstanceFloatType = 1 << 00;
    pub const F01: InstanceFloatType = 1 << 01;
    pub const F02: InstanceFloatType = 1 << 02;
    pub const F03: InstanceFloatType = 1 << 03;
    pub const F04: InstanceFloatType = 1 << 04;
    pub const F05: InstanceFloatType = 1 << 05;
    pub const F06: InstanceFloatType = 1 << 06;
    pub const F07: InstanceFloatType = 1 << 07;
    pub const F08: InstanceFloatType = 1 << 08;
    pub const F09: InstanceFloatType = 1 << 09;
    pub const F10: InstanceFloatType = 1 << 10;
    pub const F11: InstanceFloatType = 1 << 11;
    pub const F12: InstanceFloatType = 1 << 12;
    pub const F13: InstanceFloatType = 1 << 13;
    pub const F14: InstanceFloatType = 1 << 14;
    pub const F15: InstanceFloatType = 1 << 15;
}

#[derive(Component)]
pub struct InstanceCustomVec4A(pub Number, pub Number, pub Number, pub Number);
impl Default for InstanceCustomVec4A { fn default() -> Self { Self(0., 0., 0., 0.) } }
impl TInstanceData for InstanceCustomVec4A {
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
// #[derive(Component)]
// pub struct InstanceBufferCustomVec4A {
//     pub slot: usize,
//     pub index: KeyVertexBuffer,
// }
// impl TInstanceBuffer for InstanceBufferCustomVec4A {
//     fn display_name() -> String { String::from("InstanceBufferCustomVec4A") }
//     fn slot(&self) -> EVertexBufferSlot { EVertexBufferSlot::from_u8_unsafe(self.slot as u8) }
//     fn id(&mut self) -> KeyVertexBuffer { self.index.clone() }
// }
#[derive(Component)]
pub struct InstanceCustomVec4ADirty(pub bool);
impl Default for InstanceCustomVec4ADirty { fn default() -> Self { Self(false) } }
impl TInstanceFlag for InstanceCustomVec4ADirty {
    fn dirty(&self) -> bool { self.0 }
    fn reset(&mut self) { self.0 = false; }
}

#[derive(Component)]
pub struct InstanceCustomVec4B(pub Number, pub Number, pub Number, pub Number);
impl Default for InstanceCustomVec4B { fn default() -> Self { Self(0., 0., 0., 0.) } }
impl TInstanceData for InstanceCustomVec4B {
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
// #[derive(Component)]
// pub struct InstanceBufferCustomVec4B {
//     pub slot: usize,
//     pub index: KeyVertexBuffer,
// }
// impl TInstanceBuffer for InstanceBufferCustomVec4B {
//     fn display_name() -> String { String::from("InstanceBufferCustomVec4B") }
//     fn slot(&self) -> EVertexBufferSlot { EVertexBufferSlot::from_u8_unsafe(self.slot as u8) }
//     fn id(&mut self) -> KeyVertexBuffer { self.index.clone() }
// }
#[derive(Component)]
pub struct InstanceCustomVec4BDirty(pub bool);
impl Default for InstanceCustomVec4BDirty { fn default() -> Self { Self(false) } }
impl TInstanceFlag for InstanceCustomVec4BDirty {
    fn dirty(&self) -> bool { self.0 }
    fn reset(&mut self) { self.0 = false; }
}

#[derive(Component)]
pub struct InstanceCustomVec4C(pub Number, pub Number, pub Number, pub Number);
impl Default for InstanceCustomVec4C { fn default() -> Self { Self(0., 0., 0., 0.) } }
impl TInstanceData for InstanceCustomVec4C {
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
// #[derive(Component)]
// pub struct InstanceBufferCsutomVec4C {
//     pub slot: usize,
//     pub index: KeyVertexBuffer,
// }
// impl TInstanceBuffer for InstanceBufferCsutomVec4C {
//     fn display_name() -> String { String::from("InstanceBufferCsutomVec4C") }
//     fn slot(&self) -> EVertexBufferSlot { EVertexBufferSlot::from_u8_unsafe(self.slot as u8) }
//     fn id(&mut self) -> KeyVertexBuffer { self.index.clone() }
// }
#[derive(Component)]
pub struct InstanceCustomVec4CDirty(pub bool);
impl Default for InstanceCustomVec4CDirty { fn default() -> Self { Self(false) } }
impl TInstanceFlag for InstanceCustomVec4CDirty {
    fn dirty(&self) -> bool { self.0 }
    fn reset(&mut self) { self.0 = false; }
}

#[derive(Component)]
pub struct InstanceCustomVec4D(pub Number, pub Number, pub Number, pub Number);
impl Default for InstanceCustomVec4D { fn default() -> Self { Self(0., 0., 0., 0.) } }
impl TInstanceData for InstanceCustomVec4D {
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
// #[derive(Component)]
// pub struct InstanceBufferCustomVec4D {
//     pub slot: usize,
//     pub index: KeyVertexBuffer,
// }
// impl TInstanceBuffer for InstanceBufferCustomVec4D {
//     fn display_name() -> String { String::from("InstanceBufferCustomVec4D") }
//     fn slot(&self) -> EVertexBufferSlot { EVertexBufferSlot::from_u8_unsafe(self.slot as u8) }
//     fn id(&mut self) -> KeyVertexBuffer { self.index.clone() }
// }
#[derive(Component)]
pub struct InstanceCustomVec4DDirty(pub bool);
impl Default for InstanceCustomVec4DDirty { fn default() -> Self { Self(false) } }
impl TInstanceFlag for InstanceCustomVec4DDirty {
    fn dirty(&self) -> bool { self.0 }
    fn reset(&mut self) { self.0 = false; }
}