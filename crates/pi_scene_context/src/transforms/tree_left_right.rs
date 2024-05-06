use pi_scene_shell::prelude::*;


#[derive(Debug, Clone, Copy, )]
pub struct TreeLeftRoot(pub ObjectID, pub usize);
impl TreeLeftRoot {
    pub fn new(id: ObjectID) -> Self {
        Self(id, 0)
    }
}

#[derive(Debug, Clone, Copy, )]
pub struct TreeRightRoot(pub ObjectID, pub usize);
impl TreeRightRoot {
    pub fn new(id: ObjectID) -> Self {
        Self(id, 0)
    }
}

#[derive( Deref, DerefMut, Default)]
pub struct NodeChilds(pub(crate) XHashSet<Entity>);

#[derive( Deref, DerefMut)]
pub struct NodeParent(pub(crate) Option<Entity>);
