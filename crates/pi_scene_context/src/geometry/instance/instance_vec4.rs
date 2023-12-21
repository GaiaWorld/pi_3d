use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Number};

pub type InstanceVec4Type = u8;

pub struct EInstanceVec4Type;
impl EInstanceVec4Type {
    pub const V4T00: InstanceVec4Type = 1 << 00;
    pub const V4T01: InstanceVec4Type = 1 << 01;
    pub const V4T02: InstanceVec4Type = 1 << 02;
    pub const V4T03: InstanceVec4Type = 1 << 03;
    pub const V4T04: InstanceVec4Type = 1 << 04;
    pub const V4T05: InstanceVec4Type = 1 << 05;
    pub const V4T06: InstanceVec4Type = 1 << 06;
    pub const V4T07: InstanceVec4Type = 1 << 07;
}

#[derive(Component)]
pub struct InstanceV4A(pub Vector4);
impl Default for InstanceV4A { fn default() -> Self { Self(Vector4::zeros()) } }
impl TInstanceData for InstanceV4A {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsVec4A
    }
    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];
        list.iter().for_each(|v| {
            bytemuck::cast_slice(v.0.as_slice()).iter().for_each(|v| { result.push(*v); })
        });
        result
    }
}