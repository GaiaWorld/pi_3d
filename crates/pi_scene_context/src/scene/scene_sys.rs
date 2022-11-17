

use pi_ecs::prelude::{Query, ResMut};
use pi_ecs_macros::setup;

use crate::{object::GameObject, environment::{fog::SceneFog, ambient_light::AmbientLight}, scene::scene_time::SceneTime};

pub struct SysDirtySceneTick;
#[setup]
impl SysDirtySceneTick {
    #[system]
    pub fn tick(
        mut query_scenes: Query<GameObject, (&mut SceneTime, &SceneFog)>
    ) {
        query_scenes.iter_mut().for_each(|(mut scene_time, mut scene_fog)| {
            scene_time.dirty = false;
        });
    }
}