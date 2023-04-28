use crate::{geometry::instance::{instance_color::{InstanceBufferColor, InstanceColor, InstanceColorDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::command::{SysMeshModifyCommand, SysInstanceMeshModifyCommand}};

use super::world_matrix::SysInstanceBufferWorldMatrixUpdate;

pub type SysInstanceBufferColorUpdate = SysInstanceBufferUpdateFunc<InstanceColor, InstanceBufferColor, InstanceColorDirty, SysInstanceBufferWorldMatrixUpdate>;
