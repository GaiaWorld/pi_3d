use pi_engine_shell::object::ObjectID;

pub struct InstanceSourceRecord {
    pub counter: usize,
}
impl InstanceSourceRecord {
    pub fn id(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}

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