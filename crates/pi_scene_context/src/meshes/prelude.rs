
use pi_engine_shell::prelude::*;

use crate::prelude::ActionListBlend;

pub use super::{
    model::*,
    command::*,
    render_group::*,
    shader_about::*,
    abstract_mesh::*,
    skeleton::*,
    lighting::*,
};


#[derive(SystemParam)]
pub struct ActionSetMesh<'w> {
    pub create: ResMut<'w, ActionListMeshCreate>,
    pub shadow: ResMut<'w, ActionListMeshShadow>,
    pub blend: ResMut<'w, ActionListBlend>,
}

#[derive(SystemParam)]
pub struct ActionSetInstanceMesh<'w> {
    pub create: ResMut<'w, ActionListInstanceMeshCreate>,
    pub color: ResMut<'w, ActionListInstanceColor>,
    pub tilloff: ResMut<'w, ActionListInstanceTillOff>,
}