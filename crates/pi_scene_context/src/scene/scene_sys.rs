

use pi_ecs::prelude::{Query, Res};
use pi_ecs_macros::setup;
use pi_engine_shell::{run_stage::TSystemStageInfo, frame_time::SingleFrameTimeCommand};

use crate::{
    object::GameObject,
    scene::{
        scene_time::SceneTime,
        environment::{fog::SceneFog}
    }
};

use super::command::SysSceneCommand;

pub struct SysDirtySceneTick;
impl TSystemStageInfo for SysDirtySceneTick {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
        ]
    }
}
#[setup]
impl SysDirtySceneTick {
    #[system]
    pub fn tick(
        mut query_scenes: Query<GameObject, (&mut SceneTime, &SceneFog)>,
        frame: Res<SingleFrameTimeCommand>,
    ) {
        query_scenes.iter_mut().for_each(|(mut scene_time, mut scene_fog)| {
            scene_time.update(frame.frame_ms);
            scene_time.dirty = false;
        });
    }
}