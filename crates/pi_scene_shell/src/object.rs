use pi_bevy_ecs_extend::system_param::tree::EntityTreeMut;

use pi_world::{
    editor::EntityEditor, filter::Changed, prelude::App, query::Query, schedule::Update, single_res::{SingleRes, SingleResMut}, world::Entity
};

// use std::mem::replace;
// use pi_bevy_ecs_extend::prelude::EntityTreeMut;
use crate::prelude::*;



// #[derive(Debug, Clone, Copy, Default)]
// pub struct GameObject;

pub type ObjectID = Entity;

/// 准备销毁
#[derive(Debug, Component)]
pub struct DisposeReady(pub bool);
impl Default for DisposeReady {
    fn default() -> Self {
        Self(false)
    }
}

/// 可以销毁
#[derive(Debug, Component)]
pub struct DisposeCan(pub bool);
impl Default for DisposeCan {
    fn default() -> Self {
        Self(false)
    }
}

pub struct ActionEntity;
impl ActionEntity {
    pub fn init(entity: Entity, editor: &mut EntityEditor) {
        let _ = editor
            .add_components(entity, &[editor.init_component::<DisposeReady>(), editor.init_component::<DisposeCan>()]);
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
    mut cmds: SingleResMut<ActionListDisposeReady>,
    mut cmdsforref: SingleResMut<ActionListDisposeReadyForRef>,
    mut items: Query<&mut DisposeReady>,
    empty: SingleRes<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsDisposeReady(entity)| {
        if empty.id() == entity {
            return;
        }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeReady(true);
        } else {
            cmds.push(OpsDisposeReady(entity))
        }
    });
    cmdsforref
        .drain()
        .drain(..)
        .for_each(|OpsDisposeReadyForRef(entity)| {
            if empty.id() == entity {
                return;
            }

            if let Ok(mut item) = items.get_mut(entity) {
                *item = DisposeReady(true);
            } else {
                cmdsforref.push(OpsDisposeReadyForRef(entity))
            }
        });
}
pub fn sys_dispose_can(
    mut cmds: SingleResMut<ActionListDisposeCan>,
    mut items: Query<&mut DisposeCan>,
    empty: SingleRes<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsDisposeCan(entity)| {
        if empty.id() == entity {
            return;
        }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeCan(true);
        }
    });
}

pub fn sys_dispose(
    editor: EntityEditor,
    items: Query<(Entity, &DisposeCan), Changed<DisposeCan>>,
    mut tree: EntityTreeMut,
) {
    items.iter().for_each(|(entity, state)| {
        if state.0 == true {
            if editor.contains_entity(entity) {
                // log::debug!("despawn====={:?}", commands.id());
                tree.remove(entity);
                let _ = editor.destroy(entity);
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
    mut cmds: SingleResMut<ActionListSceneDispose>,
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
        app.world.insert_single_res(ActionListSceneDispose::default());
        app.world.insert_single_res(ActionListDisposeReadyForRef::default());
        app.world.insert_single_res(ActionListDisposeReady::default());
        app.world.insert_single_res(ActionListDisposeCan::default());
        
        app.add_system(Update, sys_act_scene_dispose.in_set(ERunStageChap::Dispose));
        app.add_system(Update, sys_dispose_ready.in_set(ERunStageChap::Dispose));
        app.add_system(Update, sys_dispose_can.in_set(ERunStageChap::Dispose));
        app.add_system(Update, sys_dispose.in_set(ERunStageChap::Dispose));
    }
}
