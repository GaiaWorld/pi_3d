use crate::geometry::instance::instance_tilloff::{InstancedBufferTillOff, InstanceTillOff};

use super::sys_instance::{SysInstancedBufferInit, SysInstanceBufferUpdate};


pub struct InstanceTillOffDirty;

pub type SysInstanceBufferTillOffInit = SysInstancedBufferInit<InstancedBufferTillOff>;
pub type SysInstanceBufferTillOffUpdate = SysInstanceBufferUpdate<InstancedBufferTillOff, InstanceTillOff, InstanceTillOffDirty>;
