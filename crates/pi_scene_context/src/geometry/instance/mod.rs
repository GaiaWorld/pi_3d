use pi_scene_shell::prelude::*;
use pi_slotmap::Key;

pub mod instanced_buffer;
pub mod types;
pub mod instance_world_matrix;
pub mod instance_color;
pub mod instance_tilloff;
pub mod instance_boneoffset;
pub mod instance_float;
pub mod instance_vec4;
pub mod instance_vec3;
pub mod sys_instance;

/// 标识节点为 实例Mesh
#[derive(Debug, Clone, Component, Default)]
pub struct InstanceMesh(pub Entity);
impl InstanceMesh {
    pub fn id(&self) -> String {
        self.0.index().to_string()
    }
}
impl TEntityRef for InstanceMesh {
    fn id(&self) -> Entity {
        self.0
    }
}

/// 标识实例对应的源Mesh是否脏
#[derive(Debug, Clone, Component, Default)]
pub struct DirtyInstanceSourceRefs;

/// 记录实例的源Mesh派生的所有实例
pub type InstanceSourceRefs = EntityRefInfo<DirtyInstanceSourceRefs>;

/// 标识实例对应的 源Mesh 的顶点Buffer是否脏 - 针对大量实例的Mesh会使用单独的Buffer,一般情况使用共用的Buffer
#[derive(Debug, Clone, Component, Default)]
pub struct DirtyInstanceSourceForSingleBuffer;
