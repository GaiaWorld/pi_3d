use crate::{geometry::instance::{instance_tilloff::{InstancedBufferTillOff, InstanceTillOff, InstanceTillOffDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::command::SysMeshCommand};


pub type SysInstanceBufferTillOffUpdate = SysInstanceBufferUpdateFunc<InstancedBufferTillOff, InstanceTillOff, InstanceTillOffDirty, SysMeshCommand>;
