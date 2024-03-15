// use std::mem::replace;
// use pi_bevy_ecs_extend::prelude::EntityTreeMut;
use crate::prelude::*;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{prelude::*, system::EntityCommands};

// #[derive(Debug, Clone, Copy, Default)]
// pub struct GameObject;

pub type ObjectID = Entity;

/// 准备销毁
#[derive(Component)]
pub struct DisposeReady(pub bool);
impl Default for DisposeReady {
    fn default() -> Self {
        Self(false)
    }
}

/// 可以销毁
#[derive(Component)]
pub struct DisposeCan(pub bool);
impl Default for DisposeCan {
    fn default() -> Self {
        Self(false)
    }
}

pub struct ActionEntity;
impl ActionEntity {
    pub fn init(entitycmd: &mut EntityCommands) {
        entitycmd.insert(DisposeReady::default()).insert(DisposeCan::default());
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

pub fn sys_dispose_ready(
    mut cmds: ResMut<ActionListDisposeReady>,
    mut cmdsforref: ResMut<ActionListDisposeReadyForRef>,
    mut items: Query<&mut DisposeReady>,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsDisposeReady(entity)| {
        if empty.id() == entity { return }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeReady(true);
        } else {
            cmds.push(OpsDisposeReady(entity))
        }
    });
    cmdsforref.drain().drain(..).for_each(|OpsDisposeReadyForRef(entity)| {
        if empty.id() == entity { return }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeReady(true);
        } else {
            cmdsforref.push(OpsDisposeReadyForRef(entity))
        }
    });
}
pub fn sys_dispose_can(
    mut cmds: ResMut<ActionListDisposeCan>,
    mut items: Query<&mut DisposeCan>,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsDisposeCan(entity)| {
        if empty.id() == entity { return }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeCan(true);
        }
    });
}

pub fn sys_dispose(
    mut commands: Commands,
    items: Query<(Entity, &DisposeCan), Changed<DisposeCan>>,
    mut tree: EntityTreeMut,
) {
    items.iter().for_each(|(entity, state)| {
        if state.0 == true {
            if let Some(mut commands) = commands.get_entity(entity) {
                // log::debug!("despawn====={:?}", commands.id());
                tree.remove(entity);
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
    mut items: Query<&mut DisposeReady>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneDispose(idscene)| {
        if let Ok(mut item) = items.get_mut(idscene) {
            *item = DisposeReady(true);
        } else {
            cmds.push(OpsSceneDispose(idscene))
        }
    });
}

pub struct PluginDispose;
impl Plugin for PluginDispose {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListSceneDispose::default());
        app.insert_resource(ActionListDisposeReadyForRef::default());
        app.insert_resource(ActionListDisposeReady::default());
        app.insert_resource(ActionListDisposeCan::default());
        app.add_systems(Update,
            (
                sys_act_scene_dispose,
                sys_dispose_ready,
                sys_dispose_can,
                sys_dispose
            ).in_set(ERunStageChap::Dispose)
        );
    }
}