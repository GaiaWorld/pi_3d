
#[cfg(not(feature = "use_bevy"))]
use pi_world::world::ComponentIndex;
#[cfg(not(feature = "use_bevy"))]
pub use pi_world::*;
#[cfg(not(feature = "use_bevy"))]
pub use pi_world::prelude::{*, App};
#[cfg(not(feature = "use_bevy"))]
pub use pi_world_macros::{Resource};
#[cfg(not(feature = "use_bevy"))]
pub use pi_world::prelude::{SystemParam};
#[cfg(not(feature = "use_bevy"))]
pub use pi_world::prelude::{SingleResMut as ResMut, SingleRes as Res};
#[cfg(not(feature = "use_bevy"))]
pub use pi_world::{
    editor::EntityEditor, filter::Changed, schedule_config::{IntoSystemConfigs, StageLabel}, world::Entity,
    query::*,
};

#[cfg(not(feature = "use_bevy"))]
pub type Ref<'a, T> = Ticker<'a, &'a T>;

#[cfg(not(feature = "use_bevy"))]
pub type Or<T> = T;

#[cfg(not(feature = "use_bevy"))]
pub type Commands<'a> = EntityEditor<'a>;

#[cfg(not(feature = "use_bevy"))]
pub trait Component: Bundle + 'static + Send + Sync  {
    
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static + Send + Sync> Component for T {

}

#[cfg(not(feature = "use_bevy"))]
pub struct EntityCommandTemp(Entity);
#[cfg(not(feature = "use_bevy"))]
impl EntityCommandTemp {
    pub fn id(&self) -> Entity {
        self.0
    }
}

#[cfg(not(feature = "use_bevy"))]
pub struct EntityCommands<'w, 'a> {
    entity: Entity,
    commands: &'a mut EntityEditor<'w>,
}
#[cfg(not(feature = "use_bevy"))]
pub struct EntityCommandsEmpty<'w> {
    entity: Entity,
    commands: EntityEditor<'w>,
}
impl<'w> EntityCommandsEmpty<'w> {
    #[inline]
    #[must_use = "Omit the .id() call if you do not need to store the `Entity` identifier."]
    pub fn id(&self) -> Entity {
        self.entity
    }
}

#[cfg(not(feature = "use_bevy"))]
pub struct InsertTmp {
    pub list: Vec<ComponentIndex>,
    pub idx: usize,
}
#[cfg(not(feature = "use_bevy"))]
impl InsertTmp {
    #[cfg(not(feature = "use_bevy"))]
    pub fn idx(&mut self) -> ComponentIndex {
        let result = self.list.get(self.idx).unwrap().clone();
        self.idx += 1;
        return result;
    }
    #[cfg(not(feature = "use_bevy"))]
    pub fn push(&mut self, idx: ComponentIndex) {
        self.list.push(idx);
    }
}

#[cfg(not(feature = "use_bevy"))]
pub trait InsertBundle {
    fn index(editor: &mut EntityEditor, list: &mut InsertTmp);
    fn insert(self, editor: &mut EntityCommands, list: &mut InsertTmp);
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static> InsertBundle for (T,) {
    fn index(editor: &mut EntityEditor, list: &mut InsertTmp) {
        list.push(editor.init_component::<T>());
    }
    fn insert(self, cmd: &mut EntityCommands, list:&mut InsertTmp) {
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.0;
    }
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static, T1: Bundle + 'static> InsertBundle for (T, T1) {
    fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
        list.push(editor.init_component::<T>());
        list.push(editor.init_component::<T1>());
    }
    fn insert(self, cmd: &mut EntityCommands, list:&mut InsertTmp) {
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.0;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.1;
    }
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static, T1: Bundle + 'static, T2: Bundle + 'static> InsertBundle for (T, T1, T2) {
    fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
        list.push(editor.init_component::<T>());
        list.push(editor.init_component::<T1>());
        list.push(editor.init_component::<T2>());
    }
    fn insert(self, cmd: &mut EntityCommands, list:&mut InsertTmp) {
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.0;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.1;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.2;
    }
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static, T1: Bundle + 'static, T2: Bundle + 'static, T3: Bundle + 'static> InsertBundle for (T, T1, T2, T3) {
    fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
        list.push(editor.init_component::<T>());
        list.push(editor.init_component::<T1>());
        list.push(editor.init_component::<T2>());
        list.push(editor.init_component::<T3>());
    }
    fn insert(self, cmd: &mut EntityCommands, list:&mut InsertTmp) {
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.0;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.1;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.2;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.3;
    }
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static, T1: Bundle + 'static, T2: Bundle + 'static, T3: Bundle + 'static, T4: Bundle + 'static> InsertBundle for (T, T1, T2, T3, T4) {
    fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
        list.push(editor.init_component::<T>());
        list.push(editor.init_component::<T1>());
        list.push(editor.init_component::<T2>());
        list.push(editor.init_component::<T3>());
        list.push(editor.init_component::<T4>());
    }
    fn insert(self, cmd: &mut EntityCommands, list:&mut InsertTmp) {
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.0;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.1;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.2;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.3;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.4;
    }
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static, T1: Bundle + 'static, T2: Bundle + 'static, T3: Bundle + 'static, T4: Bundle + 'static, T5: Bundle + 'static> InsertBundle for (T, T1, T2, T3, T4, T5) {
    fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
        list.push(editor.init_component::<T>());
        list.push(editor.init_component::<T1>());
        list.push(editor.init_component::<T2>());
        list.push(editor.init_component::<T3>());
        list.push(editor.init_component::<T4>());
        list.push(editor.init_component::<T5>());
    }
    fn insert(self, cmd: &mut EntityCommands, list:&mut InsertTmp) {
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.0;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.1;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.2;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.3;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.4;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.5;
    }
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static, T1: Bundle + 'static, T2: Bundle + 'static, T3: Bundle + 'static, T4: Bundle + 'static, T5: Bundle + 'static, T6: Bundle + 'static> InsertBundle for (T, T1, T2, T3, T4, T5, T6) {
    fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
        list.push(editor.init_component::<T>());
        list.push(editor.init_component::<T1>());
        list.push(editor.init_component::<T2>());
        list.push(editor.init_component::<T3>());
        list.push(editor.init_component::<T4>());
        list.push(editor.init_component::<T5>());
        list.push(editor.init_component::<T6>());
    }
    fn insert(self, cmd: &mut EntityCommands, list:&mut InsertTmp) {
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.0;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.1;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.2;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.3;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.4;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.5;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.6;
    }
}
#[cfg(not(feature = "use_bevy"))]
impl<T: Bundle + 'static, T1: Bundle + 'static, T2: Bundle + 'static, T3: Bundle + 'static, T4: Bundle + 'static, T5: Bundle + 'static, T6: Bundle + 'static, T7: Bundle + 'static> InsertBundle for (T, T1, T2, T3, T4, T5, T6, T7) {
    fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
        list.push(editor.init_component::<T>());
        list.push(editor.init_component::<T1>());
        list.push(editor.init_component::<T2>());
        list.push(editor.init_component::<T3>());
        list.push(editor.init_component::<T4>());
        list.push(editor.init_component::<T5>());
        list.push(editor.init_component::<T6>());
        list.push(editor.init_component::<T7>());
    }
    fn insert(self, cmd: &mut EntityCommands, list:&mut InsertTmp) {
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.0;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.1;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.2;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.3;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.4;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.5;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.6;
        *cmd.commands.get_component_unchecked_mut_by_id(cmd.entity, list.idx()) = self.7;
    }
}

// impl<T: InsertBundle> InsertBundle for (T,) {
//     fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
//         T::index(editor, list);
//     }
// }
// impl<T: InsertBundle, T1: InsertBundle> InsertBundle for (T, T1) {
//     fn index(editor: &mut EntityEditor, list:&mut InsertTmp) {
//         T::index(editor, list);
//         T1::index(editor, list);
//     }
// }

#[cfg(not(feature = "use_bevy"))]
impl<'w, 'a> EntityCommands<'w, 'a> {
    #[inline]
    #[must_use = "Omit the .id() call if you do not need to store the `Entity` identifier."]
    pub fn id(&self) -> Entity {
        self.entity
    }
    pub fn insert<A: Bundle + 'static>(&mut self, bundle: A) -> &mut Self {

        let entity = self.entity;
        let mut alter = self.alter::<A>();
        match alter.alter(entity, bundle) {
            Ok(_) => {}
            Err(e) => { log::error!(" insert {:?}", e); }
        }

        self
    }
    // pub fn insert_alter<A: Bundle + 'static>(&mut self, bundle: A, mut alter: Option<pi_world::alter::Alterer<'a, (), (), A, ()>>) -> Option<pi_world::alter::Alterer<'a, (), (), A, ()>> {

    //     let entity = self.entity;
    //     if alter.is_none() {
    //         alter = Some(self.commands.world().make_alterer::<(), (), A, ()>());
    //     }
    //     match alter.as_mut().unwrap().alter(entity, bundle) {
    //         Ok(_) => {}
    //         Err(e) => { log::error!(" insert {:?}", e); }
    //     }
    //     alter
    // }
    pub fn alter<A: Bundle + 'static>(&mut self) -> pi_world::alter::Alterer<(), (), A, ()> {
        self.commands.world().make_alterer::<(), (), A, ()>()
    }
    pub fn despawn(&mut self) {
        self.commands.destroy(self.entity);
    }
}

#[cfg(not(feature = "use_bevy"))]
pub trait TEntityCommands<'w> {
    fn spawn_empty<'a>(&'a mut self) -> EntityCommands<'w, 'a>;
    fn spawn<'a, M: Bundle + 'static>(&'a mut self, bundle: M) -> EntityCommands<'w, 'a>;
    fn get_entity<'a>(&'a mut self, entity: Entity) -> Option<EntityCommands<'w, 'a>>;
    fn entity<'a>(&'a mut self, entity: Entity) -> EntityCommands<'w, 'a>;
}
#[cfg(not(feature = "use_bevy"))]
impl<'w> TEntityCommands<'w> for EntityEditor<'w> {
    fn spawn_empty<'a>(&'a mut self) -> EntityCommands<'w, 'a> {
        let entity = self.world().make_inserter().insert(());

        // let entity = self.alloc_entity();
        EntityCommands {
            entity: entity,
            commands: self
        }
    }
    fn spawn<'a, A: Bundle + 'static>(&'a mut self, bundle: A) -> EntityCommands<'w, 'a> {
        let entity = self.world().make_inserter().insert(bundle);
        // match self.world().make_alterer::<(), (), A, ()>().alter(entity, bundle) {
        //     Ok(_) => {},
        //     Err(e) => { log::error!("spawn {:?}", e); }
        // }

        // let entity = self.alloc_entity();
        // let mut components = A::components(vec![]);
        // let mut indexs = vec![];

        // components.drain(..).for_each(|item| {
        //     indexs.push(item.world_index);
        // });
        // self.insert_components_by_index(&indexs);
        // *self.get_component_mut::<A>(entity).unwrap() = bundle;

        EntityCommands {
            entity,
            commands: self
        }
    }
    fn get_entity<'a>(&'a mut self, entity: Entity) -> Option<EntityCommands<'w, 'a>> {
        if self.contains_entity(entity) {
            Some(EntityCommands { entity, commands: self })
        } else {
            None
        }
    }
    fn entity<'a>(&'a mut self, entity: Entity) -> EntityCommands<'w, 'a> {
        EntityCommands { entity, commands: self }
    }
}

#[cfg(not(feature = "use_bevy"))]
pub trait Resource {

}
#[cfg(not(feature = "use_bevy"))]
impl<T: 'static> Resource for T {

}

#[cfg(not(feature = "use_bevy"))]
pub trait AppResourceTemp {
    fn insert_resource<T: 'static>(&mut self, resource: T) -> &mut Self;
    fn add_systems<M>(&mut self, stage_label: impl StageLabel, system: impl IntoSystemConfigs<M>) -> &mut Self;
    fn update(&mut self);
}
#[cfg(not(feature = "use_bevy"))]
impl AppResourceTemp for App {
    fn insert_resource<T: 'static>(&mut self, resource: T) -> &mut Self {
        self.world.insert_single_res(resource);
        self
    }
    fn add_systems<M>(&mut self, stage_label: impl StageLabel, system: impl IntoSystemConfigs<M>) -> &mut Self {
        self.add_system(stage_label, system)
    }
    fn update(&mut self) {
        self.run()
    }
}
#[cfg(not(feature = "use_bevy"))]
pub trait WorldResourceTemp {
    fn insert_resource<T: 'static>(&mut self, resource: T)  -> &mut Self;
    fn get_resource<T: 'static>(&self) -> Option<& T>;
    fn get_resource_mut<T: 'static>(&mut self) -> Option<& mut T>;
    fn contains_resource<T: 'static>(&self) -> bool;
    fn spawn_empty<'w>(&'w mut self) -> EntityCommandsEmpty<'w>;
}
#[cfg(not(feature = "use_bevy"))]
impl WorldResourceTemp for World {
    fn insert_resource<T: 'static>(&mut self, resource: T)  -> &mut Self {
        self.insert_single_res(resource);
        self
    }

    fn get_resource<T: 'static>(& self) -> Option<& T> {
        self.get_single_res::<T>()
    }

    fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_single_res_mut::<T>()
    }

    fn contains_resource<T: 'static>(&self) -> bool {
        self.contains_resource::<T>()
    }
    fn spawn_empty<'w>(&'w mut self) -> EntityCommandsEmpty<'w> {
        let entity = self.make_inserter().insert(());
        let mut editor = self.make_entity_editor();
        EntityCommandsEmpty {
            entity: entity,
            commands: editor
        }
    }
}

#[cfg(not(feature = "use_bevy"))]
pub fn add_systems(label: impl StageLabel, ) {
    
}