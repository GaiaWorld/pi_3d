use pi_engine_shell::prelude::*;

// pub mod instanced_buffer;
// pub mod types;
pub mod instance_world_matrix;
pub mod instance_color;
pub mod instance_tilloff;
pub mod sys_instance;

pub struct InstanceSourceRecord {
    pub counter: usize,
}
impl InstanceSourceRecord {
    pub fn id(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}

#[derive(Debug, Clone)]
pub struct InstanceList {
    pub list: Vec<ObjectID>,
    id: usize,
}
impl InstanceList {
    pub fn new(record: &mut InstanceSourceRecord) -> Self {
        Self { list: vec![], id: record.id() }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
}


pub struct InstanceSource(pub ObjectID);