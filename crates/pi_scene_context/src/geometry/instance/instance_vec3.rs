use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Number};

pub type InstanceVec3Type = u8;

pub struct EInstanceVec3Type;
impl EInstanceVec3Type {
    pub const V3T00: InstanceVec3Type = 1 << 00;
    pub const V3T01: InstanceVec3Type = 1 << 01;
    pub const V3T02: InstanceVec3Type = 1 << 02;
    pub const V3T03: InstanceVec3Type = 1 << 03;
    pub const V3T04: InstanceVec3Type = 1 << 04;
    pub const V3T05: InstanceVec3Type = 1 << 05;
    pub const V3T06: InstanceVec3Type = 1 << 06;
    pub const V3T07: InstanceVec3Type = 1 << 07;
}
