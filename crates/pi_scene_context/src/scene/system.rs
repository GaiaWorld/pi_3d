use bevy::prelude::{Query, Entity, Changed, ResMut};
use pi_engine_shell::prelude::SceneID;

use crate::{commands::{DisposeReady, ActionListDisposeReady, ActionListDisposeCan, OpsDisposeCan, OpsDisposeReady}, prelude::ScenePassRenderCfg};


pub fn sys_dispose_about_scene(
    scenes: Query<(Entity, &DisposeReady, &ScenePassRenderCfg), Changed<DisposeReady>>,
    items: Query<(Entity, &SceneID)>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    scenes.iter().for_each(|(entity,  state, _)| {
        if state.0 == false { return; }

        items.iter().for_each(|(item, sceneid)| {
            if sceneid.0 == entity {
                disposereadylist.push(OpsDisposeReady::ops(item));
            }
        });

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}