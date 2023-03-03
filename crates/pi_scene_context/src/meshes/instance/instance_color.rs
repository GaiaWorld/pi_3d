use crate::{geometry::instance::{instance_color::{InstancedBufferColor, InstanceColor, InstancedColorDirty}, sys_instance::SysInstanceBufferUpdateFunc}, meshes::command::{SysMeshModifyCommand, SysInstanceMeshModifyCommand}};

use super::world_matrix::SysInstanceBufferWorldMatrixUpdate;

pub type SysInstanceBufferColorUpdate = SysInstanceBufferUpdateFunc<InstanceColor, InstancedBufferColor, InstancedColorDirty, SysInstanceBufferWorldMatrixUpdate>;
