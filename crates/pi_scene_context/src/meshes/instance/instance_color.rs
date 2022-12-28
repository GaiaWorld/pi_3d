use crate::geometry::instance::instance_color::{InstancedBufferColor, InstanceColor};

use super::sys_instance::{SysInstancedBufferInit, SysInstanceBufferUpdate};


pub struct InstancedColorDirty;

pub type SysInstanceBufferColorInit = SysInstancedBufferInit<InstancedBufferColor>;
pub type SysInstanceBufferColorUpdate = SysInstanceBufferUpdate<InstancedBufferColor, InstanceColor, InstancedColorDirty>;
