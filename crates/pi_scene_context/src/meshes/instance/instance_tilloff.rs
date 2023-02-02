use crate::{geometry::instance::{instance_tilloff::{InstancedBufferTillOff, InstanceTillOff, InstanceTillOffDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::command::{SysMeshModifyCommand, SysInstanceMeshModifyCommand}};


pub type SysInstanceBufferTillOffUpdate = SysInstanceBufferUpdateFunc<InstancedBufferTillOff, InstanceTillOff, InstanceTillOffDirty, SysInstanceMeshModifyCommand>;
