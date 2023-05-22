use std::mem::replace;

use pi_engine_shell::prelude::*;

use super::{
    vertex_buffer_useinfo::*,
    base::*,
};

pub type ActionListGeometryCreate = ActionList<OpsGeomeryCreate>;
pub struct OpsGeomeryCreate(pub(crate) Entity, pub(crate) Entity, pub(crate) Vec<VertexBufferDesc>, pub(crate) Option<IndicesBufferDesc>);
impl OpsGeomeryCreate {
    pub fn ops(mesh: Entity, geo: Entity, vertices: Vec<VertexBufferDesc>, indices: Option<IndicesBufferDesc>) -> Self {
        Self(mesh, geo, vertices, indices)
    }
}