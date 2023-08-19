use std::mem::replace;

use pi_engine_shell::prelude::*;

use super::{
    vertex_buffer_useinfo::*,
    base::*,
};

pub struct OpsGeomeryCreate(pub(crate) Entity, pub(crate) Entity, pub(crate) Vec<VertexBufferDesc>, pub(crate) Option<IndicesBufferDesc>, pub(crate) u8);
impl OpsGeomeryCreate {
    pub fn ops(mesh: Entity, geo: Entity, vertices: Vec<VertexBufferDesc>, indices: Option<IndicesBufferDesc>) -> Self {
        Self(mesh, geo, vertices, indices, 0)
    }
}
pub type ActionListGeometryCreate = ActionList<OpsGeomeryCreate>;


pub struct OpsInstanceWorldMatrixs(pub(crate) Entity, pub(crate) Vec<u8>, pub(crate) u8);
impl OpsInstanceWorldMatrixs {
    pub fn ops(geo: Entity, data: Vec<u8>) -> Self {
        Self(geo, data, 0)
    }
}
pub type ActionListInstanceWorldMatrixs = ActionList<OpsInstanceWorldMatrixs>;

pub struct OpsInstanceColors(pub(crate) Entity, pub(crate) Vec<u8>, pub(crate) u8);
impl OpsInstanceColors {
    pub fn ops(geo: Entity, data: Vec<u8>) -> Self {
        Self(geo, data, 0)
    }
}
pub type ActionListInstanceColors = ActionList<OpsInstanceColors>;

pub struct OpsInstanceTilloffs(pub(crate) Entity, pub(crate) Vec<u8>, pub(crate) u8);
impl OpsInstanceTilloffs {
    pub fn ops(geo: Entity, data: Vec<u8>) -> Self {
        Self(geo, data, 0)
    }
}
pub type ActionListInstanceTilloffs = ActionList<OpsInstanceTilloffs>;