use bevy::prelude::{Query, Changed, ResMut};
use pi_engine_shell::prelude::*;

use crate::{commands::*, prelude::{MaterialID, RendererID}};

use super::base::Light;


pub fn sys_dispose_about_light(
    items: Query<(Entity, &DisposeReady, &Light), Changed<DisposeReady>>,
    materials: Query<&MaterialID>,
    renderers: Query<&RendererID>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    empty: Res<SingleEmptyEntity>,
) {
    items.iter().for_each(|(entity, state, light)| {
        if state.0 == false { return; }

        if let Ok(id) = materials.get(entity) {
            disposereadylist.push(OpsDisposeReady::ops(id.0));
        }
        if let Ok(id) = renderers.get(entity) {
            disposecanlist.push(OpsDisposeCan::ops(id.0));
        }

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}
