pub use pi_engine_shell::prelude::*;
use pi_bevy_ecs_extend::prelude::EntityTreeMut;

use crate::{
    commands::*, prelude::*,
};

pub struct ActionEntity;
impl ActionEntity {
    pub fn init(entitycmd: &mut EntityCommands) {
        entitycmd.insert(DisposeReady::default()).insert(DisposeCan::default());
    }
}

pub type OpsDispose = OpsDisposeReady;
pub type ActionListDispose = ActionListDisposeReady;

pub fn sys_dispose_ready(
    mut cmds: ResMut<ActionListDisposeReady>,
    mut cmdsforref: ResMut<ActionListDisposeReadyForRef>,
    mut items: Query<&mut DisposeReady>,
    mut enables: Query<(&mut Enable, &mut GlobalEnable)>,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsDisposeReady(entity)| {
        if empty.id() == entity { return }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeReady(true);
            if let Ok((mut enable, mut globalenable)) = enables.get_mut(entity) {
                *enable = Enable(0.);
                *globalenable = GlobalEnable(false);
            }
        } else {
            cmds.push(OpsDisposeReady(entity))
        }
    });
    cmdsforref.drain().drain(..).for_each(|OpsDisposeReadyForRef(entity)| {
        if empty.id() == entity { return }

        if let Ok(mut item) = items.get_mut(entity) {
            *item = DisposeReady(true);
            if let Ok((mut enable, mut globalenable)) = enables.get_mut(entity) {
                *enable = Enable(0.);
                *globalenable = GlobalEnable(false);
            }
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
        app.insert_resource(ActionListDispose::default());
        app.insert_resource(ActionListDisposeCan::default());
        app.add_systems(Update, sys_act_scene_dispose.in_set(ERunStageChap::Initial));
        app.add_systems(Update, sys_dispose_ready.in_set(ERunStageChap::Dispose));
        app.add_systems(Update, sys_dispose_can.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));
        app.add_systems(Update, sys_dispose.after(sys_dispose_can).in_set(ERunStageChap::Dispose));
    }
}