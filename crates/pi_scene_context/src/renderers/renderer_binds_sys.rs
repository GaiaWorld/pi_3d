use pi_ecs::{prelude::{Query, ResMut}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject}, run_stage::{TSystemStageInfo}};
use render_resource::uniform_buffer::RenderDynUniformBuffer;
use render_shader::{shader_set::{ShaderSetSceneAboutBindOffset}};

use crate::{
    flags::SceneID,
    scene::{
        scene_time::SceneTime, 
        environment::fog::SceneFog,
        command::SysSceneCommand
    },
    main_camera_render::command::SysMainCameraRenderCommand
};


pub struct SysSceneBindUpdate;
impl TSystemStageInfo for SysSceneBindUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSceneCommand::key(), SysMainCameraRenderCommand::key(),
        ]
    }
}
#[setup]
impl SysSceneBindUpdate {
    #[system]
    pub fn sys(
        scenes: Query<
            GameObject,
            (&SceneTime, &SceneFog),
            Or<(Changed<SceneTime>, Changed<SceneFog>)>,
        >,
        renderers: Query<
            GameObject,
            (&SceneID, &ShaderSetSceneAboutBindOffset)
        >,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        renderers.iter().for_each(|(id_scene, renderer_bindoff)| {
            if let Some((scene_time, scene_fog)) = scenes.get(id_scene.0.clone()) {
                if let Some(bind) = renderer_bindoff.time() {
                    dynbuffer.as_mut().set_uniform(bind.bind_offset(), scene_time);
                }
                if let Some(bind) = renderer_bindoff.fog() {
                    dynbuffer.as_mut().set_uniform(bind.bind_offset(), scene_fog);
                }
            }
        });
    }
}