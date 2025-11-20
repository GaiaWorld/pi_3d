
use crossbeam::queue::ArrayQueue;
use pi_slotmap::Key;

use crate::ecs::*;

use crate::prelude::*;

// #[derive(Debug, Clone, Copy, Default)]
// pub struct GameObject;

pub type ObjectID = Entity;

/// 标识实体准备销毁 - 下一步是处理关联内容可以销毁
#[derive(Component)]
pub struct DisposeReady(pub bool);
impl Default for DisposeReady {
    fn default() -> Self {
        Self(false)
    }
}

/// 标识实体可以销毁 - 下一步即销毁
#[derive(Component)]
pub struct DisposeCan(pub bool);
impl Default for DisposeCan {
    fn default() -> Self {
        Self(false)
    }
}

pub type BundleEntity = (DisposeReady, DisposeCan);

pub struct ActionEntity;
impl ActionEntity {
    pub fn init() -> BundleEntity {
        (DisposeReady::default(), DisposeCan::default())
    }
}

pub struct OpsDisposeReady(pub(crate) Entity);
impl OpsDisposeReady {
    pub fn ops(entity: Entity) -> Self {
        Self(entity)
    }
}
/// 外部操作， 内部不可操作
pub type ActionListDisposeReady = ActionList<OpsDisposeReady>;

pub struct OpsDisposeReadyForRef(pub(crate) Entity);
impl OpsDisposeReadyForRef {
    pub fn ops(entity: Entity) -> Self {
        Self(entity)
    }
}
pub type ActionListDisposeReadyForRef = ActionList<OpsDisposeReadyForRef>;

pub struct OpsDisposeCan(pub(crate) Entity);
impl OpsDisposeCan {
    pub fn ops(entity: Entity) -> Self {
        Self(entity)
    }
}
pub type ActionListDisposeCan = ActionList<OpsDisposeCan>;


// #[derive(Default)]
// pub struct EntityRecord(Vec<Entity>, EntityRepeatCheck, Vec<Entity>);
// impl EntityRecord {
//     pub fn insert(&mut self, entity: &Entity) -> bool {
//         let isinsert = self.1.insert(entity);
//         if isinsert {
//             self.0.push(*entity);
//         }
//         isinsert
//     }
//     pub fn contains(&self, entity: &Entity) -> bool {
//         return self.1.contains(entity);
//     }
//     pub fn sort_after_batch_insert(&mut self) {
//         self.0.sort_by(|a, b| a.index().cmp(&b.index()));
//     }
//     pub fn remove(&mut self, entity: &Entity) -> bool {
//         if self.1.remove(entity) {
//             self.2.push(*entity);
//             return true;
//         } else {
//             return false;
//         }
//     }
//     pub fn reset_after_batch_remove(&mut self) {
//         let mut tmp = vec![];
//         self.2.drain(..).for_each(|entity| {
//             match self.0.binary_search_by(|probe| probe.index().cmp(&entity.index())) {
//                 Ok(idx) => tmp.push(idx),
//                 Err(_) => {},
//             };
//         });
//         tmp.sort();

//         let deletecount = tmp.len();
//         if 0 < deletecount {
//             if let Some(last) = self.0.last() {
//                 let last = *last;
//                 while let Some(idx) = tmp.pop() {
//                     self.0[idx] = last;
//                 }
//                 self.0.sort_by(|a, b| a.index().cmp(&b.index()));
//                 self.0.truncate(self.0.len() - deletecount);
//             }
//         }
//     }

//     pub fn len(&self) -> usize {
//         self.0.len()
//     }
//     pub fn clear(&mut self) {
//         self.0.clear();
//         self.1.clear();
//         self.2.clear();
//     }
//     pub fn iter(&self) -> std::slice::Iter<Entity> {
//         return self.0.iter();
//     }
//     pub fn capacity(&self) -> usize {
//         self.0.capacity() + self.2.capacity()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.0.is_empty()
//     }
// }

#[derive(Clone)]
pub struct EntityRepeatCheck(Vec<u8>, usize);
impl Default for EntityRepeatCheck {
    fn default() -> Self {
        Self(Vec::with_capacity(1024), 0)
    }
}
impl EntityRepeatCheck {
    pub fn insert(&mut self, entity: &Entity) -> bool {
        let idx = entity.index();
        let i = idx / 8;
        if self.0.len() < i + 1 {
            for _ in self.0.len()..(i + 1) {
                self.0.push(0);
            }
        }
        let v0 = self.0[i];
        let v1 = 1 << (idx % 8);
        let has = (v0 & v1) > 0;
        if !has {
            self.1 += 1;
        }
        self.0[idx / 8] = v0 | v1;
        return !has;
    }
    pub fn contains(&self, entity: &Entity) -> bool {
        let idx = entity.index();
        let i = idx / 8;
        if self.0.len() < i + 1 {
            return false;
        } else {
            let v0 = self.0[i];
            let v1 = 1 << (idx % 8);
            let has = (v0 & v1) > 0;
            return has;
        }
    }
    pub fn remove(&mut self, entity: &Entity) -> bool {
        let idx = entity.index();
        let i = idx / 8;
        if self.0.len() < i + 1 {
            return false;
        } else {
            let v0 = self.0[i];
            let v1 = 1 << (idx % 8);
            let has = (v0 & v1) > 0;
            if has {
                self.0[idx / 8] -= v1;
                self.1 -= 1;
            }
            return has;
        }
    }
    pub fn len(&self) -> usize {
        self.1
    }
    pub fn clear(&mut self) {
        self.0.clear();
        self.1 = 0;
    }
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

#[derive(Resource)]
pub struct EntityFilterForComponentChanged(pub ArrayQueue<EntityRepeatCheck>);
impl Default for EntityFilterForComponentChanged {
    fn default() -> Self {
        Self(ArrayQueue::new(16))
    }
}
impl EntityFilterForComponentChanged {
    pub fn pop(&self) -> EntityRepeatCheck {
        if let Some(mut set) = self.0.pop() {
            set.clear();
            set
        } else {
            EntityRepeatCheck::default()
        }
    }
    pub fn push(&self, set: EntityRepeatCheck) {
        let _ = self.0.push(set);
    }
}

pub fn sys_dispose_ready(
    mut cmds: ResMut<ActionListDisposeReady>,
    mut cmdsforref: ResMut<ActionListDisposeReadyForRef>,
    mut items: Query<&mut DisposeReady>,
    empty: Res<SingleEmptyEntity>,
    mut commands: Commands,
) {
    cmds.drain().for_each(|OpsDisposeReady(entity)| {
        if empty.id() == entity || commands.contains_entity(entity) == false { return }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeReady(true);
        } else {
            // log::warn!("sys_dispose_ready  ====={:?}", entity);
            if let Some(mut commands) = commands.get_entity(entity) {
                // log::warn!("despawn====={:?}", commands.id());
                commands.despawn();
            }
            // cmds.push(OpsDisposeReady(entity))
        }
    });
    cmdsforref.drain().for_each(|OpsDisposeReadyForRef(entity)| {
        if empty.id() == entity || commands.contains_entity(entity) == false { return }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeReady(true);
        } else {
            // log::warn!("sys_dispose_ready Ref ====={:?}", entity);
            if let Some(mut commands) = commands.get_entity(entity) {
                commands.despawn();
            }
            // cmdsforref.push(OpsDisposeReadyForRef(entity))
        }
    });
}
pub fn sys_dispose_can(
    mut cmds: ResMut<ActionListDisposeCan>,
    readys: Query<(Entity, &DisposeReady), Changed<DisposeReady>>,
    mut items: Query<&mut DisposeCan>,
    empty: Res<SingleEmptyEntity>,
    mut commands: Commands,
) {
    readys.iter().for_each(|(entity, dispose)| {
        if dispose.0 {
            if let Ok(mut item) = items.get_mut(entity) {
                *item = DisposeCan(true);
            }
        }
    });
    cmds.drain().for_each(|OpsDisposeCan(entity)| {
        if empty.id() == entity { return }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeCan(true);
        } else {
            if let Some(mut commands) = commands.get_entity(entity) {
                // log::warn!("despawn====={:?}", commands.id());
                commands.despawn();
            }
        }
    });
}

pub fn sys_dispose(
    mut commands: Commands,
    items: Query<(Entity, &DisposeCan), Changed<DisposeCan>>,
    nodes: Query<&Up, (With<Layer>, With<Down>, With<Up>)>,
    mut tree: EntityTreeMut,
    empty: Res<SingleEmptyEntity>
) {
   
    let mut removes = vec![];
    items.iter().for_each(|(entity, state)| {
        if state.0 == true {
            if let Ok(up) = nodes.get(entity) {
                // log::error!("Dispose {:?}", (entity, up.parent(), up.prev(), up.next(), nodes.contains(up.parent()), nodes.contains(up.prev()), nodes.contains(up.next())));
                if nodes.contains(up.parent()) {
                    tree.remove(entity);
                }
            }
            removes.push(entity);
        }
    });
    // log::error!("sys_dispose >>> {:?} ", removes);
    removes.drain(..).for_each(|entity| {
        if let Some(mut commands) = commands.get_entity(entity) {
            // log::warn!("despawn====={:?}", commands.id());
            if empty.id() != entity {
                commands.despawn();
            }
        }
    });
}

pub struct OpsSceneDispose(Entity);
impl OpsSceneDispose {
    pub fn ops(entity: Entity) -> OpsSceneDispose {
        OpsSceneDispose(entity)
    }
}
pub type ActionListSceneDispose = ActionList<OpsSceneDispose>;
pub fn sys_act_scene_dispose(
    mut cmds: ResMut<ActionListSceneDispose>,
    items: Query<(Entity, &SceneID)>,
    mut disposeready: Query<&mut DisposeReady>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_scene_dispose"));
    cmds.drain().for_each(|OpsSceneDispose(idscene)| {
        items.iter().for_each(|(entity, sceneid)| {
            if sceneid.0 == entity {
                if let Ok(mut dispose) = disposeready.get_mut(entity) { dispose.0 = true };
            }
        });

        if let Ok(mut dispose) = disposeready.get_mut(idscene) { dispose.0 = true };
    });
}

pub struct PluginDispose;
impl Plugin for PluginDispose {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListSceneDispose::default());
        app.insert_resource(ActionListDisposeReadyForRef::default());
        app.insert_resource(ActionListDisposeReady::default());
        app.insert_resource(ActionListDisposeCan::default());
        app.insert_resource(EntityFilterForComponentChanged::default());

#[cfg(feature = "use_bevy")]
{
    app.add_systems(StageD3,
        (
            sys_act_scene_dispose,
            sys_dispose_ready,
            sys_dispose_can,
            sys_dispose
        ).chain().in_set(ERunStageChap::Dispose)
    );
}

#[cfg(not(feature = "use_bevy"))]
{
    app
    .add_systems(StageD3, sys_act_scene_dispose
        // .run_if(runif_acts::<OpsSceneDispose>)                         
        .in_set(ERunStageChap::Dispose))
    .add_systems(StageD3, sys_dispose_ready    .after(sys_act_scene_dispose)       .in_set(ERunStageChap::Dispose))
    .add_systems(StageD3, sys_dispose_can      .after(sys_dispose_ready)           .in_set(ERunStageChap::_Dispose))
    .add_systems(StageD3Final, sys_dispose          .after(sys_dispose_can)             .in_set(ERunStageChap::StateCheck))
    ;
}
    }
}