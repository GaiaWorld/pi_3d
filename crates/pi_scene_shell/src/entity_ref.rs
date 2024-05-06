use std::{marker::PhantomData, collections::hash_set::Iter};


use pi_hash::XHashSet;
use pi_world::world::Entity;

pub trait TEntityRef {
    fn id(&self) -> Entity;
}


pub struct EntityRefInfo<F: Default + Clone > {
    refs: XHashSet<Entity>,
    pub dirty: bool,
    pub request_dispose: bool,
    p: PhantomData<F>,
}
impl<F: Default + Clone > Default for EntityRefInfo<F> {
    fn default() -> Self {
        Self {
            refs: XHashSet::default(),
            dirty: false,
            request_dispose: false,
            p: PhantomData::default(),
        }
    }
}
impl<F: Default + Clone > EntityRefInfo<F> {
    pub fn iter(&self) -> Iter<Entity> {
        self.refs.iter()
    }
    pub fn len(&self) -> usize {
        self.refs.len()
    }
    pub fn insert(&mut self, entity: Entity) -> bool {
        if !self.refs.contains(&entity) {
            self.refs.insert(entity);
            self.dirty = true;
            true
        } else {
            false
        }
    }
    pub fn remove(&mut self, entity: &Entity) -> bool {
        if self.refs.remove(entity) {
            self.dirty = true;
            true
        } else {
            false
        }
    }
    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }
}

pub enum EventEntityRef<R:  TEntityRef> {
    Use(Entity, Entity, PhantomData<R>),
    UnUse(Entity, Entity, PhantomData<R>),
}
