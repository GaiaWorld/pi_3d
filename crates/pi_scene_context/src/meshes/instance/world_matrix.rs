
use crate::{geometry::instance::{instance_world_matrix::{InstanceBufferWorldMatrix, InstanceWorldMatrixDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::model::{RenderWorldMatrix, SysRenderMatrixUpdate}};


pub type SysInstanceBufferWorldMatrixUpdate = SysInstanceBufferUpdateFunc<RenderWorldMatrix, InstanceBufferWorldMatrix, InstanceWorldMatrixDirty, SysRenderMatrixUpdate>;
