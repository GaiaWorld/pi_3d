use pi_scene_shell::prelude::*;

use super::base::Scene;

pub fn sys_dispose_about_scene(
    scenes: Query<(Entity, &DisposeReady, &Scene), Changed<DisposeReady>>,
    items: Query<(Entity, &SceneID)>,
    mut disposecan: Query<&mut DisposeCan>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_dispose_about_scene"));
    scenes.iter().for_each(|(entity, state, _)| {
        if state.0 == false { return; }

        items.iter().for_each(|(item, sceneid)| {
            if sceneid.0 == entity {
                if let Ok(mut dispose) = disposecan.get_mut(item) { dispose.0 = true };
            }
        });

        if let Ok(mut dispose) = disposecan.get_mut(entity) { dispose.0 = true };
    });
}
