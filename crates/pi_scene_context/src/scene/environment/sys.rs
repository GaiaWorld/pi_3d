use pi_engine_shell::prelude::*;

use super::{BindSceneEffect, scene_time::*, ambient_light::*, fog::*};



pub fn sys_bind_update_scene_time(
    mut scenes: Query<(&mut SceneTime, &mut BindSceneEffect)>,
) {
    scenes.iter_mut().for_each(|(mut scene_time, mut bind)| {
        scene_time.update(&bind);
    });
}

pub fn sys_bind_update_scene_ambient(
    mut scenes: Query<(&AmbientColor, &AmbientIntensity, &mut BindSceneEffect), Or<(Changed<AmbientColor>, Changed<AmbientIntensity>)>>,
) {
    scenes.iter_mut().for_each(|(color, intensity, mut bind)| {
        update_ambient_uniform(color, intensity, &mut bind);
    });
}

pub fn sys_bind_update_scene_fog(
    mut scenes: Query<(&SceneFogColor, &SceneFogParam, &mut BindSceneEffect), Or<(Changed<SceneFogColor>, Changed<SceneFogParam>)>>,
) {
    scenes.iter_mut().for_each(|(color, param, mut bind)| {
        update_scenefog_uniform(color, param, &mut bind);
    });
}
