use crate::{geometry::instance::{instance_color::{InstancedBufferColor, InstanceColor, InstancedColorDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::command::SysMeshCommand};

pub type SysInstanceBufferColorUpdate = SysInstanceBufferUpdateFunc<InstancedBufferColor, InstanceColor, InstancedColorDirty, SysMeshCommand>;
