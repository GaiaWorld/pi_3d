use pi_scene_shell::prelude::*;

use super::{BindSceneEffect, scene_time::*, ambient_light::*, fog::*};

pub fn sys_bind_update_scene_ambient(
    mut scenes: Query<(&mut SceneTime, &AmbientColor, &SceneFog, &mut BindSceneEffect), (Changed<AmbientColor>, Changed<SceneFog>)>,
) {
    scenes.iter_mut().for_each(|( scene_time, color, fog, mut bind)| {
        scene_time.update(&bind);
        update_ambient_uniform(color, &mut bind);
        update_scenefog_uniform(fog, &mut bind);
    });
}
