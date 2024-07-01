use pi_scene_shell::prelude::*;

use super::base::Scene;

pub fn sys_dispose_about_scene(
    mut event: ComponentChanged<DisposeReady>,
    mut scenes: Query<(Entity, &mut DisposeReady, &Scene), Changed<DisposeReady>>,
    items: Query<(Entity, &SceneID)>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    tree: EntityTree,
) {
    scenes.iter_mut().for_each(|(entity,  mut state, _)| {
        if state.0 == false { return; }

        items.iter().for_each(|(item, sceneid)| {
            if sceneid.0 == entity {
                disposereadylist.push(OpsDisposeReady::ops(item));
            }
        });

        if let Some(down) = tree.get_down(entity) {
            // log::error!("Dispose Scene  {:?}", down.count());
            if down.count() == 0 {
                // log::error!("Dispose Scene !!!!!!!!!!!!!");
                disposecanlist.push(OpsDisposeCan::ops(entity));
            }
        }
        disposereadylist.push(OpsDisposeReady::ops(entity));
    });
}
