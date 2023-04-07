use pi_engine_shell::prelude::*;

use crate::{object::GameObject, environment::{fog::SceneFog, ambient_light::AmbientLight}, scene::scene_time::SceneTime};

// pub struct SceneUniformTickUpdate;
// #[setup]
// impl SceneUniformTickUpdate {
//     #[system]
    pub fn sys_uniform_scene(
        query_scenes: Query<(&SceneTime, &SceneFog, &AmbientLight)>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        //  log::debug!("Scene Uniform Tick Update");
        query_scenes.iter().for_each(|(time, fog, ambl)| {
            dynbuffer.set_uniform::<SceneTime>(&time.bind_offset, time);
            dynbuffer.set_uniform::<SceneFog>(&fog.bind_offset, fog);
            dynbuffer.set_uniform::<AmbientLight>(&ambl.bind_offset, ambl);
        });
    }
// }
