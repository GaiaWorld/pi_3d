
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    viewer::{prelude::*, command_sys::ActionViewer},
    renderers::{prelude::*, command_sys::*}, 
    transforms::command_sys::ActionTransformNode,
    layer_mask::prelude::*, prelude::{SceneMainCameraID, SceneID},
};

use super::{
    target_camera::*,
    camera::*,
    command::*,
};

pub fn sys_create_camera(
    mut cmds: ResMut<ActionListCameraCreate>,
    mut commands: Commands,
    mut dynallocator: ResMut<ResBindBufferAllocator>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraCreation(scene, entity, toscreen)| {
        if let Some(mut commands) = commands.get_entity(entity) {

            ActionCamera::init(&mut commands, scene, toscreen);
            ActionAnime::as_anime_group_target(&mut commands);

            if let Some(bindviewer) = BindViewer::new(&mut dynallocator) {
                commands.insert(bindviewer);
            }
        }
    })
}


pub fn sys_act_camera_mode(
    mut cmds: ResMut<ActionListCameraMode>,
    mut cameras: Query<&mut EFreeCameraMode>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraMode(entity, mode)| {
        if let Ok(mut camera) = cameras.get_mut(entity) {
            if *camera != mode {
                *camera = mode;
            }
        } else {
            cmds.push(OpsCameraMode(entity, mode))
        }
    });
}


pub fn sys_act_camera_active(
    mut cmds: ResMut<ActionListCameraActive>,
    mut cameras: Query<(&SceneID, &mut Camera, &mut ViewerActive)>,
    mut scenes: Query<&mut SceneMainCameraID>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraActive(entity, mode)| {
        // log::warn!("CameraActive ");
        if let Ok((idscene, mut camera, mut viewer)) = cameras.get_mut(entity) {
            // log::warn!("CameraActive {:?}, New {:?}", viewer, mode);
            if camera.0 != mode {
                *camera = Camera(mode);
                *viewer = ViewerActive(mode);
                // log::warn!("CameraActive Ok");
            }
            if mode {
                if let Ok(mut maincamera) = scenes.get_mut(idscene.0) {
                    *maincamera = SceneMainCameraID(Some(entity));
                }
            }
        } else {
            cmds.push(OpsCameraActive(entity, mode))
        }
    });
}

pub fn sys_act_camera_fixed_mode(
    mut cmds: ResMut<ActionListCameraFixedMode>,
    mut cameras: Query<&mut EFixedMode>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraFixedMode(entity, mode)| {
        if let Ok(mut camera) = cameras.get_mut(entity) {
            if *camera != mode {
                *camera = mode;
            }
        } else {
            cmds.push(OpsCameraFixedMode(entity, mode))
        }
    });
}

pub fn sys_act_camera_nearfar(
    mut cmds: ResMut<ActionListCameraNearFar>,
    mut cameras: Query<&mut CameraNearFar>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraNearFar(entity, mode)| {
        if let Ok(mut camera) = cameras.get_mut(entity) {
            *camera = mode;
        } else {
            cmds.push(OpsCameraNearFar(entity, mode))
        }
    });
}

pub fn sys_act_camera_fov(
    mut cmds: ResMut<ActionListCameraFov>,
    mut cameras: Query<(&mut CameraFov, &mut RecordCameraFov)>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraFov(entity, mode)| {
        if let Ok((mut camera, mut record)) = cameras.get_mut(entity) {
            record.0 = mode.clone();
            *camera = mode;
        } else {
            cmds.push(OpsCameraFov(entity, mode))
        }
    });
}

pub fn sys_act_camera_orth_size(
    mut cmds: ResMut<ActionListCameraOrthSize>,
    mut cameras: Query<(&mut CameraOrthSize, &mut RecordCameraOrthSize)>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraOrthSize(entity, mode)| {
        if let Ok((mut camera, mut record)) = cameras.get_mut(entity) {
            record.0 = mode.clone();
            *camera = mode;
        } else {
            cmds.push(OpsCameraOrthSize(entity, mode))
        }
    });
}

pub fn sys_act_camera_aspect(
    mut cmds: ResMut<ActionListCameraAspect>,
    mut cameras: Query<&mut ViewerAspect>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraAspect(entity, val)| {
        if let Ok(mut camera) = cameras.get_mut(entity) {
            *camera = val;
        } else {
            cmds.push(OpsCameraAspect(entity, val))
        }
    });
}


pub fn sys_act_camera_pixel_size(
    mut cmds: ResMut<ActionListCameraPixelSize>,
    mut cameras: Query<&mut ViewerSize>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraPixelSize(entity, w, h)| {
        if let Ok(mut camera) = cameras.get_mut(entity) {
            *camera = ViewerSize(w, h);
        } else {
            cmds.push(OpsCameraPixelSize(entity, w, h))
        }
    });
}

pub fn sys_act_camera_toscreen(
    mut cmds: ResMut<ActionListCameraToScreen>,
    mut cameras: Query<(&mut CameraToScreen, &ViewerRenderersInfo)>,
    mut renderercmds: ResMut<ActionListRendererModify>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraToScreen(entity, val)| {
        if let Ok((mut camera, renderers)) = cameras.get_mut(entity) {
            *camera = CameraToScreen(val);
            renderers.map.iter().for_each(|v| {
                renderercmds.push(OpsRendererCommand::RenderToFinal(v.1.1.0, val));
            });
        } else {
            cmds.push(OpsCameraToScreen(entity, val))
        }
    });
}

pub fn sys_act_camera_target(
    mut cameras: Query<&mut CameraTarget>,
    mut cmds: ResMut<ActionListCameraTarget>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraTarget(entity, target)| {
        if let Ok(mut camera) = cameras.get_mut(entity) {
            *camera = CameraTarget(target);
        } else {
            cmds.push(OpsCameraTarget(entity, target))
        }
    });
}

pub fn sys_create_camera_renderer(
    mut cmds: ResMut<ActionListCameraRenderer>,
    mut commands: Commands,
    mut viewers: Query<
        (
            &mut ViewerRenderersInfo, &CameraToScreen, &mut DirtyViewerRenderersInfo
        )
    >,
) {
    cmds.drain().drain(..).for_each(|OpsCameraRendererInit(id_viewer, id_renderer, rendername, passorders, color_format, depth_stencil_format, count)| {
        log::warn!("OpsCameraRenderer: A");

        if let Ok((mut viewer_renderers, toscreen, mut dirtyflag)) = viewers.get_mut(id_viewer) {
            log::warn!("OpsCameraRenderer: AA");

            if let Some((_, id_render)) = viewer_renderers.map.get(&rendername) {
                if let Some(mut cmd) = commands.get_entity(id_render.0) {
                    cmd.despawn();
                }
            }

            log::warn!("Camera Renderer Init!! ");
            *dirtyflag = DirtyViewerRenderersInfo;

            viewer_renderers.map.insert(rendername.clone(), (passorders.clone(), RendererID(id_renderer)));

            if let Some(mut cmd) = commands.get_entity(id_renderer) {
                ActionRenderer::init(
                    &mut cmd, id_viewer, passorders, ViewerSize::DEFAULT_WIDTH, ViewerSize::DEFAULT_HEIGHT,
                    color_format, depth_stencil_format, toscreen.0
                );
            }

        } else {
            cmds.push(OpsCameraRendererInit(id_viewer, id_renderer, rendername, passorders, color_format, depth_stencil_format, count + 1));
        }
    });
}

pub struct ActionCamera;
impl ActionCamera {
    pub fn init(
        commands: &mut EntityCommands,
        scene: Entity,
        toscreen: bool,
    ) {
        ActionTransformNode::init(commands, scene);
        ActionCamera::as_camera(commands, toscreen);
    }
    pub(crate) fn as_camera(
        commands: &mut EntityCommands,
        toscreen: bool,
    ) {
        commands.insert(Camera(false))
            .insert(EFreeCameraMode::default())
            .insert(EFixedMode::default())
            .insert(CameraFov::default())
            .insert(CameraOrthSize::default())
            .insert(RecordCameraFov::default())
            .insert(RecordCameraOrthSize::default()) 
            .insert(CameraNearFar::default())
            .insert(CameraToScreen(toscreen))
            .insert(CameraViewport::default())
            .insert(LayerMask::default())
            .insert(CameraUp(CoordinateSytem3::up()))
            .insert(CameraTarget(Vector3::new(0., 0., 1.)))
            .insert(TargetCameraParam::default())
            .insert(CameraParam::default())
            ;
        
        ActionViewer::as_viewer(commands);
        commands.insert(ViewerSize::default());
    }
    pub fn create(
        app: &mut App,
        scene: Entity,
        toscreen: bool,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);
        let entity = commands.spawn_empty().id();

        queue.apply(&mut app.world);

        let mut cmds = app.world.get_resource_mut::<ActionListCameraCreate>().unwrap();
        cmds.push(OpsCameraCreation(scene, entity, toscreen));

        entity
    }

    pub fn active_camera(
        commands: &mut EntityCommands,
        value: bool,
    ) {
        commands.insert(ViewerActive(value));
    }

}

    pub fn sys_update_target_camera_modify(
        mut cameras: Query<(&CameraUp, &CameraTarget, &mut TargetCameraParam), Or<(Changed<CameraUp>, Changed<CameraTarget>)>>,
    ) {
        cameras.iter_mut().for_each(|(up, target, mut param)| {
            *param = TargetCameraParam::create(up.0.clone(), target.0.clone());
        });
    }

    pub fn sys_update_camera_renderer(
        mut renderercmds: ResMut<ActionListRendererModify>,
        cameras: Query<
            (&Camera, &ViewerRenderersInfo, &CameraToScreen),
            Or<(Changed<Camera>, Changed<DirtyViewerRenderersInfo>)>
        >,
    ) {
        cameras.iter().for_each(|(enable, renderers, toscreen)| {
            let enable = enable.0;
            renderers.map.iter().for_each(|(_k, v)| {
                let id_render = v.1.0;
                renderercmds.push(OpsRendererCommand::Active(id_render, enable));
                renderercmds.push(OpsRendererCommand::RenderToFinal(id_render, toscreen.0));
            });
        });

    }

