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

#[derive(Debug, Clone, Component, Default)]
pub struct DirtyInstanceSourceRefs;

pub type InstanceSourceRefs = EntityRefInfo<DirtyInstanceSourceRefs>;

#[derive(Debug, Clone, Component, Default)]
pub struct DirtyInstanceSourceForSingleBuffer;
