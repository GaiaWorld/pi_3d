use std::{marker::PhantomData, collections::hash_set::Iter};

use bevy_ecs::prelude::{Entity, Component};
use pi_hash::XHashSet;

pub trait TEntityRef {
    fn id(&self) -> Entity;
}

#[derive(Component)]
pub struct EntityRefInfo<F: Default + Clone + Component> {
    refs: XHashSet<Entity>,
    pub dirty: bool,
    pub request_dispose: bool,
    p: PhantomData<F>,
}
impl<F: Default + Clone + Component> Default for EntityRefInfo<F> {
    fn default() -> Self {
        Self {
            refs: XHashSet::default(),
            dirty: false,
            request_dispose: false,
            p: PhantomData::default(),
        }
    }
}
impl<F: Default + Clone + Component> EntityRefInfo<F> {
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

pub enum EventEntityRef<R: Component + TEntityRef> {
    Use(Entity, Entity, PhantomData<R>),
    UnUse(Entity, Entity, PhantomData<R>),
}

// pub fn sys_entity_ref_modify<F: Default + Clone + Component, R: Component + TEntityRef>(
//     mut commands: Commands,
//     entities: Query<(Entity, &R), Changed<R>>,
//     mut items: Query<&mut EntityRefInfo<F, R>>
// ) {
//     entities.iter().for_each(|(entity, target)| {
//         let id = target.id();
//         if let Ok(mut target) = items.get_mut(id) {
//             target.refs.insert(entity);
//             commands.entity(id).insert(F::default());
//         }
//     });
// }

// pub struct EntityRefRecorder(pub XHashMap<Entity, EntityRefInfo>);