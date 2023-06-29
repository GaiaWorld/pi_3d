use pi_engine_shell::prelude::*;

use super::{BindSceneEffect, scene_time::*, ambient_light::*, fog::*};



pub fn sys_bind_update_scene_time(
    mut scenes: Query<(&mut SceneTime, &mut BindSceneEffect)>,
) {
    scenes.iter_mut().for_each(|(mut scene_time, mut bind)| {
        scene_time.update(&mut bind);
    });
}

pub fn sys_bind_update_scene_ambient(
    mut scenes: Query<(&AmbientLight, &mut BindSceneEffect), Or<(Changed<AmbientLight>, Changed<BindSceneEffect>)>>,
) {
    scenes.iter_mut().for_each(|(item, mut bind)| {
        item.update(&mut bind);
    });
}

pub fn sys_bind_update_scene_fog(
    mut scenes: Query<(&SceneFog, &mut BindSceneEffect), Or<(Changed<SceneFog>, Changed<BindSceneEffect>)>>,
) {
    scenes.iter_mut().for_each(|(item, mut bind)| {
        item.update(&mut bind);
    });
}
