use pi_ecs::prelude::{Query, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;

use crate::{object::GameObject, environment::{fog::SceneFog, ambient_light::AmbientLight}, scene::scene_time::SceneTime};

pub struct SceneUniformTickUpdate;
#[setup]
impl SceneUniformTickUpdate {
    #[system]
    pub fn tick(
        query_scenes: Query<GameObject, (&SceneTime, &SceneFog, &AmbientLight)>,
        mut dynbuffer: ResMut<DynUniformBuffer>,
    ) {
        println!("Scene Uniform Tick Update");
        query_scenes.iter().for_each(|(time, fog, ambl)| {
            dynbuffer.set_uniform::<SceneTime>(&time.bind_offset, time);
            dynbuffer.set_uniform::<SceneFog>(&fog.bind_offset, fog);
            dynbuffer.set_uniform::<AmbientLight>(&ambl.bind_offset, ambl);
        });
    }
}
