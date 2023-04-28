use pi_engine_shell::prelude::*;

// pub mod instanced_buffer;
// pub mod types;
pub mod instance_world_matrix;
pub mod instance_color;
pub mod instance_tilloff;
pub mod sys_instance;

#[derive(Debug, Clone, Component)]
pub struct InstanceSourceID(pub Entity);
impl InstanceSourceID {
    pub fn id(&self) -> String {
        self.0.to_bits().to_string()
    }
}
impl TEntityRef for InstanceSourceID {
    fn id(&self) -> Entity {
        self.0
    }
}


#[derive(Debug, Clone, Default, Component)]
pub struct DirtyInstanceSourceRefs;

pub type InstanceSourceRefs = EntityRefInfo<DirtyInstanceSourceRefs, InstanceSourceID>;
