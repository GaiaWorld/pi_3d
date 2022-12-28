
use crate::{geometry::instance::instance_world_matrix::InstancedBufferWorldMatrix, meshes::model::RenderWorldMatrix};

use super::sys_instance::{SysInstancedBufferInit, SysInstanceBufferUpdate};


pub struct InstancedWorldMatrixDirty;

pub type SysInstanceBufferWorldMatrixInit = SysInstancedBufferInit<InstancedBufferWorldMatrix>;
pub type SysInstanceBufferWorldMatrixUpdate = SysInstanceBufferUpdate<InstancedBufferWorldMatrix, RenderWorldMatrix, InstancedWorldMatrixDirty>;
