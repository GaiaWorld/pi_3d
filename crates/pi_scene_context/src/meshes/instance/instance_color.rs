use crate::{geometry::instance::{instance_color::{InstancedBufferColor, InstanceColor, InstancedColorDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::command::{SysMeshModifyCommand, SysInstanceMeshModifyCommand}};

pub type SysInstanceBufferColorUpdate = SysInstanceBufferUpdateFunc<InstanceColor, InstancedBufferColor, InstancedColorDirty, SysInstanceMeshModifyCommand>;
