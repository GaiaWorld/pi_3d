use crate::ecs::*;

use std::{collections::BTreeSet, marker::PhantomData};

use crate::object::EntityRepeatCheck;
use pi_slotmap::{DefaultKey, SlotMap};

pub trait TEntityRef {
    fn id(&self) -> Entity;
}

#[derive(Component)]
pub struct EntityRefInfo<F: Default + Component> {
    // refs: Vec<Entity>,
    refs: BTreeSet<Entity>,
    p: PhantomData<F>,
}
impl<F: Default + Component> Default for EntityRefInfo<F> {
    fn default() -> Self {
        Self {
            // refs: Vec::default(),
            refs: BTreeSet::default(),
            p: PhantomData::default(),
        }
    }
}
impl<F: Default + Component> EntityRefInfo<F> {
    pub fn iter(&self) -> std::collections::btree_set::Iter<Entity> {
    // pub fn iter(&self) -> std::slice::Iter<Entity> {
        self.refs.iter()
    }
    pub fn len(&self) -> usize {
        self.refs.len()
    }
    pub fn capacity(&self) -> usize {
        self.refs.len()
    }
    pub fn insert(&mut self, entity: Entity) -> bool {
        // let idx = match self.refs.binary_search(&entity) {
        //     Ok(_idx) => return false,
        //     Err(idx) => idx,
        // };
        // self.refs.insert(idx, entity);
        // // self.refs.insert(entity)
        // return true;
        
        self.refs.insert(entity)

        // let idx = entity.index();
        // if idx >= self.refs.len() {
        //     let len = idx - self.refs.len() + 1;
        //     for _ in 0..len {
        //         self.refs.push(None);
        //     }
        // }
        // if !self.refs[idx].is_some() {
        //     self.refs[idx] = Some(entity);
        //     self.dirty = true;
        //     true
        // } else {
        //     false
        // }
    }
    pub fn remove(&mut self, entity: &Entity) -> bool {
        // let idx = match self.refs.binary_search(&entity) {
        //     Ok(idx) => idx,
        //     Err(_) => return false,
        // };
        // self.refs.remove(idx);
        // // self.refs.insert(entity)
        // return true;

        self.refs.remove(entity)

        // let idx = entity.index();
        // if idx < self.refs.len() {
        //     self.refs[idx] = None;
        //     self.dirty = true;
        //     true
        // } else {
        //     false
        // }
        // // if self.refs.remove(entity) {
        // //     self.dirty = true;
        // //     true
        // // } else {
        // //     false
        // // }
    }
    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }
}

pub enum EventEntityRef<R: Component + TEntityRef> {
    Use(Entity, Entity, PhantomData<R>),
    UnUse(Entity, Entity, PhantomData<R>),
}

#[derive(Component)]
pub struct EntityConnectInfo<F: Default + Component> {
    // refs: Vec<Entity>,
    refs: SlotMap<DefaultKey, Entity>,
    check: EntityRepeatCheck,
    p: PhantomData<F>,
}
impl<F: Default + Component> Default for EntityConnectInfo<F> {
    fn default() -> Self {
        Self {
            // refs: Vec::default(),
            refs: SlotMap::default(),
            check: EntityRepeatCheck::default(),
            p: PhantomData::default(),
        }
    }
}
impl<F: Default + Component> EntityConnectInfo<F> {
    pub fn iter(&self) -> pi_slotmap::basic::Iter<DefaultKey, Entity> {
    // pub fn iter(&self) -> std::slice::Iter<Entity> {
        self.refs.iter()
    }
    pub fn len(&self) -> usize {
        self.refs.len()
    }
    pub fn capacity(&self) -> usize {
        self.refs.capacity()
    }
    pub fn insert(&mut self, entity: Entity) -> Option<EntityRefID<F>> {
        // let idx = match self.refs.binary_search(&entity) {
        //     Ok(_idx) => return false,
        //     Err(idx) => idx,
        // };
        // self.refs.insert(idx, entity);
        // // self.refs.insert(entity)
        // return true;
        if self.check.contains(&entity) {
            None
        } else {
            let key = self.refs.insert(entity);
            Some(EntityRefID::new(key))
        }

        // let idx = entity.index();
        // if idx >= self.refs.len() {
        //     let len = idx - self.refs.len() + 1;
        //     for _ in 0..len {
        //         self.refs.push(None);
        //     }
        // }
        // if !self.refs[idx].is_some() {
        //     self.refs[idx] = Some(entity);
        //     self.dirty = true;
        //     true
        // } else {
        //     false
        // }
    }
    pub fn remove(&mut self, entity: &Entity, key: &EntityRefID<F>) -> bool {
        // let idx = match self.refs.binary_search(&entity) {
        //     Ok(idx) => idx,
        //     Err(_) => return false,
        // };
        // self.refs.remove(idx);
        // // self.refs.insert(entity)
        // return true;
        self.check.remove(entity);
        if let Some(val) = self.refs.remove(key.0) {
            val == *entity
        } else {
            false
        }

        // let idx = entity.index();
        // if idx < self.refs.len() {
        //     self.refs[idx] = None;
        //     self.dirty = true;
        //     true
        // } else {
        //     false
        // }
        // // if self.refs.remove(entity) {
        // //     self.dirty = true;
        // //     true
        // // } else {
        // //     false
        // // }
    }
    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }
}

#[derive(Default, Component)]
pub struct EntityRefID<F: Default + Component>(pub DefaultKey, pub PhantomData<F>);
impl<F: Default + Component> EntityRefID<F> {
    pub fn new(key: DefaultKey) -> Self {
        Self(key, PhantomData::default())
    }
}
