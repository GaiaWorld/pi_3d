
use pi_engine_shell::prelude::*;

use crate::commands::*;
use super::material::*;


pub fn sys_dispose_about_material(
    items: Query<(Entity, &DisposeReady, &MaterialRefs), Changed<DisposeReady>>,
    mut _disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
) {
    items.iter().for_each(|(entity, state, refs)| {
        if defaultmat.0 == entity { return; }

        if state.0 == true && refs.len() == 0 {
            disposecanlist.push(OpsDisposeCan::ops(entity));
        }
    });
}