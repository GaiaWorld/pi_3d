use crate::{geometry::instance::{instance_tilloff::{InstancedBufferTillOff, InstanceTillOff, InstanceTillOffDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::command::{SysMeshModifyCommand, SysInstanceMeshModifyCommand}};

use super::world_matrix::SysInstanceBufferWorldMatrixUpdate;


pub type SysInstanceBufferTillOffUpdate = SysInstanceBufferUpdateFunc<InstanceTillOff, InstancedBufferTillOff, InstanceTillOffDirty, SysInstanceBufferWorldMatrixUpdate>;
