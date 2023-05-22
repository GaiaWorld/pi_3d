
use pi_engine_shell::prelude::*;

use crate::{pass::EPassTag, };

pub struct OpsMaterialCreate(pub Entity, pub KeyShaderMeta, pub EPassTag);
impl OpsMaterialCreate {
    pub fn ops(mat: Entity, shader_meta: &str, pass: EPassTag) -> Self {
        Self(mat, Atom::from(shader_meta), pass)
    }
}
pub type ActionListMaterialCreate = ActionList<OpsMaterialCreate>;

#[derive(Debug)]
pub enum OpsMaterialUse {
    Use(Entity, Entity),
    UnUse(Entity, Entity),
}
impl OpsMaterialUse {
    pub fn ops(id_mesh: Entity, id_mat: Entity) -> Self {
        Self::Use(id_mesh, id_mat)
    }
}
pub type ActionListMaterialUse = ActionList<OpsMaterialUse>;