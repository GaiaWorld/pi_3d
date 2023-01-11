
use crate::{geometry::instance::{instance_world_matrix::{InstancedBufferWorldMatrix, InstancedWorldMatrixDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::model::{RenderWorldMatrix, SysRenderMatrixUpdate}};


pub type SysInstanceBufferWorldMatrixUpdate = SysInstanceBufferUpdateFunc<InstancedBufferWorldMatrix, RenderWorldMatrix, InstancedWorldMatrixDirty, SysRenderMatrixUpdate>;
