use crate::ecs::*;

use crate::prelude::*;

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
    nodes: Query<(&Up), (With<Layer>, With<Down>, With<Up>)>,
    mut tree: EntityTreeMut,
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
            commands.despawn();
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

#[cfg(feature = "use_bevy")]
{
    app.add_systems(Update,
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
    .add_systems(Update, sys_act_scene_dispose                                            .in_set(ERunStageChap::Dispose))
    .add_systems(Update, sys_dispose_ready    .after(sys_act_scene_dispose)       .in_set(ERunStageChap::Dispose))
    .add_systems(Update, sys_dispose_can      .after(sys_dispose_ready)           .in_set(ERunStageChap::Dispose))
    .add_systems(Update, sys_dispose          .after(sys_dispose_can)             .in_set(ERunStageChap::Dispose))
    ;
}
    }
}