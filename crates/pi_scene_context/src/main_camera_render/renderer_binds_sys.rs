use pi_ecs::{prelude::{Query, ResMut}, query::{Changed, Or, With}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::{TSystemStageInfo}};
use render_resource::uniform_buffer::RenderDynUniformBuffer;
use render_shader::{shader_set::{ShaderSetSceneAboutBindOffset}};

use crate::{
    flags::SceneID,
    scene::{
        scene_time::SceneTime, 
        environment::fog::SceneFog,
        command::SysSceneCommand
    },
    viewer::{ViewerViewMatrix, ViewerProjectionMatrix, ViewerTransformMatrix, ViewerGlobalPosition, ViewerDirection, ViewerID},
    cameras::{SysViewerUpdatedForCamera, camera::CameraParam},
    renderers::{render_object::RendererID}
};


pub struct SysRendererInitForCamera;
impl TSystemStageInfo for SysRendererInitForCamera {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSceneCommand::key(), SysViewerUpdatedForCamera::key()
        ]
    }
}
#[setup]
impl SysRendererInitForCamera {
    #[system]
    pub fn sys(
        cameras: Query<
            GameObject,
            (ObjectID, &SceneID, &RendererID, &ViewerViewMatrix, &ViewerProjectionMatrix, &ViewerTransformMatrix, &ViewerGlobalPosition, &ViewerDirection),
            (Changed<RendererID>, With<CameraParam>),
        >,
        scenes: Query<
            GameObject,
            (&SceneTime, &SceneFog),
        >,
        renderers: Query<GameObject, &ShaderSetSceneAboutBindOffset>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        log::debug!("SysRendererInitForCamera: ");
        cameras.iter().for_each(|(id_camera, id_scene, id_renderer, view_matrix, pro_matrix, vp_matrix, camera_pos, camera_dir)| {
            log::debug!("SysRendererInitForCamera: 0");
            if let Some((scene_time, scene_fog)) = scenes.get(id_scene.0.clone()) {
                log::debug!("SysRendererInitForCamera: 1");

                if let Some(renderer_bindoff) = renderers.get(id_renderer.0) {
                    dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), view_matrix);
                    dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), pro_matrix);
                    dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), vp_matrix);
                    dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), camera_pos);
                    dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), camera_dir);

                    if let Some(bind) = renderer_bindoff.time() {
                        dynbuffer.as_mut().set_uniform(bind.bind_offset(), scene_time);
                    }
                    if let Some(bind) = renderer_bindoff.fog() {
                        dynbuffer.as_mut().set_uniform(bind.bind_offset(), scene_fog);
                    }
                }
            }
        });
    }
}

pub struct SysRendererInitBindForCamera;
impl TSystemStageInfo for SysRendererInitBindForCamera {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysRendererInitForCamera::key(), SysViewerUpdatedForCamera::key()
        ]
    }
}
#[setup]
impl SysRendererInitBindForCamera {
    #[system]
    pub fn sys(
        viewers: Query<
            GameObject,
            (&ViewerViewMatrix, &ViewerProjectionMatrix, &ViewerTransformMatrix, &ViewerGlobalPosition, &ViewerDirection),
            With<CameraParam>
        >,
        scenes: Query<
            GameObject,
            (&SceneTime, &SceneFog),
        >,
        renderers: Query<
            GameObject,
            (&SceneID, &ViewerID, &ShaderSetSceneAboutBindOffset),
            Or<(Changed<SceneID>, Changed<ViewerID>, Changed<ShaderSetSceneAboutBindOffset>,)>,
        >,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        log::debug!("SysRendererInitBindForCamera:");
        renderers.iter().for_each(|(id_scene, id_viewerer, renderer_bindoff)| {
            log::debug!("SysRendererInitBindForCamera: 0");
            if let (
                Some((scene_time, scene_fog)),
                Some((view_matrix, pro_matrix, vp_matrix, camera_pos, camera_dir))
            ) = (
                scenes.get(id_scene.0.clone()),
                viewers.get(id_viewerer.0.clone())
            ) {

                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), view_matrix);
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), pro_matrix);
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), vp_matrix);
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), camera_pos);
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), camera_dir);

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

pub struct SysCameraBindUpdate;
impl TSystemStageInfo for SysCameraBindUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysViewerUpdatedForCamera::key(), SysRendererInitBindForCamera::key()
        ]
    }
}
#[setup]
impl SysCameraBindUpdate {
    #[system]
    pub fn sys(
        cameras: Query<
            GameObject,
            (&RendererID, &ViewerViewMatrix, &ViewerProjectionMatrix, &ViewerTransformMatrix, &ViewerGlobalPosition, &ViewerDirection),
            (Changed<ViewerTransformMatrix>, With<CameraParam>),
        >,
        renderers: Query<
            GameObject,
            (&ShaderSetSceneAboutBindOffset)
        >,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        log::debug!("SysCameraBindUpdate:");
        cameras.iter().for_each(|(id_renderer, view_matrix, pro_matrix, vp_matrix, camera_pos, camera_dir)| {
            log::debug!("SysCameraBindUpdate: 0");
            if let Some((renderer_bindoff)) = renderers.get(id_renderer.0.clone()) {
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), view_matrix);
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), pro_matrix);
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), vp_matrix);
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), camera_pos);
                dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), camera_dir);
            }
        });
    }
}