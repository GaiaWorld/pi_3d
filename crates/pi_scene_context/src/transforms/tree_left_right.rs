use pi_engine_shell::prelude::*;


#[derive(Debug, Clone, Copy, Component)]
pub struct TreeLeftRoot(pub ObjectID, pub usize);
impl TreeLeftRoot {
    pub fn new(id: ObjectID) -> Self {
        Self(id, 0)
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct TreeRightRoot(pub ObjectID, pub usize);
impl TreeRightRoot {
    pub fn new(id: ObjectID) -> Self {
        Self(id, 0)
    }
}
