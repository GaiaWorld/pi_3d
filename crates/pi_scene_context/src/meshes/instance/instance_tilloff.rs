use crate::{geometry::instance::{instance_tilloff::{InstanceBufferTillOff, InstanceTillOff, InstanceTillOffDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::command::{SysMeshModifyCommand, SysInstanceMeshModifyCommand}};

use super::world_matrix::SysInstanceBufferWorldMatrixUpdate;


pub type SysInstanceBufferTillOffUpdate = SysInstanceBufferUpdateFunc<InstanceTillOff, InstanceBufferTillOff, InstanceTillOffDirty, SysInstanceBufferWorldMatrixUpdate>;
