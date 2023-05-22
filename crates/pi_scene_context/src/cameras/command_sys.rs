
use pi_engine_shell::prelude::*;
use pi_scene_math::{Number, Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    viewer::{prelude::*, command_sys::ActionViewer},
    renderers::{prelude::*, command_sys::*}, 
    flags::{Enable, UniqueName}, 
    animation::{command_sys::*},
    scene::command_sys::ActionScene,
    transforms::{command_sys::ActionTransformNode, prelude::*},
    layer_mask::prelude::*,
};

use super::{
    target_camera::*,
    camera::*,
    command::*,
};

pub fn sys_camera_create(
    mut cmds: ResMut<ActionListCameraCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    mut dynallocator: ResMut<ResBindBufferAllocator>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraCreation(scene, entity, name, toscreen)| {
        let mut commands = commands.entity(entity);

        ActionScene::add_to_scene(&mut commands, &mut tree, scene);
        ActionTransformNode::init_for_tree(&mut commands);
        ActionTransformNode::as_transform_node(&mut commands, name);
        ActionCamera::as_camera(&mut commands, toscreen);
        ActionAnime::as_anime_group_target(&mut commands);

        if let Some(bindviewer) = BindViewer::new(&mut dynallocator) {
            commands.insert(bindviewer);
        }
    })
}


pub fn sys_camera_mode(
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


pub fn sys_camera_active(
    mut cmds: ResMut<ActionListCameraActive>,
    mut cameras: Query<(&mut Camera, &mut ViewerActive)>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraActive(entity, mode)| {
        log::warn!("CameraActive ");
        if let Ok((mut camera, mut viewer)) = cameras.get_mut(entity) {
            log::warn!("CameraActive {:?}, New {:?}", viewer, mode);
            if camera.0 != mode {
                *camera = Camera(mode);
                *viewer = ViewerActive(mode);
                log::warn!("CameraActive Ok");
            }
        } else {
            cmds.push(OpsCameraActive(entity, mode))
        }
    });
}

pub fn sys_camera_fixed_mode(
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

pub fn sys_camera_nearfar(
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

pub fn sys_camera_fov(
    mut cmds: ResMut<ActionListCameraFov>,
    mut cameras: Query<&mut CameraFov>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraFov(entity, mode)| {
        if let Ok(mut camera) = cameras.get_mut(entity) {
            *camera = mode;
        } else {
            cmds.push(OpsCameraFov(entity, mode))
        }
    });
}

pub fn sys_camera_orth_size(
    mut cmds: ResMut<ActionListCameraOrthSize>,
    mut cameras: Query<&mut CameraOrthSize>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraOrthSize(entity, mode)| {
        if let Ok(mut camera) = cameras.get_mut(entity) {
            *camera = mode;
        } else {
            cmds.push(OpsCameraOrthSize(entity, mode))
        }
    });
}

pub fn sys_camera_aspect(
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


pub fn sys_camera_pixel_size(
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

pub fn sys_camera_toscreen(
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

pub fn sys_camera_target(
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

pub fn sys_camera_renderer_action(
    mut cmds: ResMut<ActionListCameraRenderer>,
    mut commands: Commands,
    mut viewers: Query<
        (
            &Camera, &mut ViewerRenderersInfo, &UniqueName, &CameraToScreen
        )
    >,
    mut render_graphic: ResMut<PiRenderGraph> ,
    mut renderercmds: ResMut<ActionListRendererModify>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraRendererInit(id_viewer, id_renderer, graphic_desc, color_format, depth_stencil_format)| {
        
        if let Ok((enable, mut viewer_renderers, name, toscreen)) = viewers.get_mut(id_viewer) {

            if let Some((desc, id_render)) = viewer_renderers.map.get(&graphic_desc.curr) {
                // viewer_renderers.map.insert(graphic_desc.curr.clone(), (graphic_desc.clone(), RendererID(id_render.0)));
                let mut commands = commands.entity(id_render.0);
                // ActionRenderer::modify(&mut commands, ERendererCommand::Active(enable.0));
                // ActionRenderer::modify(&mut commands, ERendererCommand::ColorFormat(RenderColorFormat(color_format)));
                // ActionRenderer::modify(&mut commands, ERendererCommand::DepthFormat(RenderDepthFormat(depth_stencil_format)));
                commands.despawn();
            }

            match ActionRenderer::create_graphic_node(&mut render_graphic, name.0.to_string(), id_viewer, RendererID(id_renderer), &graphic_desc) {
                Ok(node) => {
                    log::warn!("CameraRenderer: {:?}", toscreen.0);
                    if toscreen.0 {
                        render_graphic.add_depend(WindowRenderer::CLEAR_KEY, name.0.to_string());
                        render_graphic.add_depend(name.0.to_string(), WindowRenderer::KEY);
                    }
                    commands.entity(id_viewer).insert(DirtyViewerRenderersInfo);

                    viewer_renderers.map.insert(graphic_desc.curr.clone(), (graphic_desc.clone(), RendererID(id_renderer)));
                    
                    let mut commands = commands.entity(id_renderer);
                    ActionRenderer::as_renderer(
                        &mut commands, node, id_viewer, graphic_desc, ViewerSize::DEFAULT_WIDTH, ViewerSize::DEFAULT_HEIGHT
                    );
                    renderercmds.push(OpsRendererCommand::ColorFormat(id_renderer, RenderColorFormat(color_format)));
                    renderercmds.push(OpsRendererCommand::DepthFormat(id_renderer, RenderDepthFormat(depth_stencil_format)));
                    log::warn!("Camera Renderer Init!!");
                },
                Err(_) => {},
            }
        } else {
            cmds.push(OpsCameraRendererInit(id_viewer, id_renderer, graphic_desc, color_format, depth_stencil_format));
        }
    });
}

pub struct ActionCamera;
impl ActionCamera {
    pub(crate) fn as_camera(
        commands: &mut EntityCommands,
        toscreen: bool,
    ) {
        commands.insert(Camera(false))
            .insert(EFreeCameraMode::Orthograhic)
            .insert(EFixedMode::HorizontalFixed)
            .insert(CameraFov(0.75))
            .insert(CameraNearFar(0.1, 1000.0))
            .insert(CameraOrthSize(4.))
            .insert(CameraToScreen(toscreen))
            .insert(CameraViewport::default())
            .insert(LayerMask::default())
            .insert(CameraUp(CoordinateSytem3::up()))
            .insert(CameraTarget(Vector3::new(0., 0., 1.)));
        
        ActionViewer::as_viewer(commands);
        commands.insert(ViewerSize::default());
    }
    pub fn create(
        app: &mut App,
        scene: Entity,
        name: String,
        toscreen: bool,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);
        let entity = commands.spawn_empty().id();

        queue.apply(&mut app.world);

        let mut cmds = app.world.get_resource_mut::<ActionListCameraCreate>().unwrap();
        cmds.push(OpsCameraCreation(scene, entity, name, toscreen));

        entity
    }

    pub fn active_camera(
        commands: &mut EntityCommands,
        value: bool,
    ) {
        commands.insert(ViewerActive(value));
    }

}

    pub fn sys_cmds_target_camera_modify(
        cameras: Query<(Entity, &CameraUp, &CameraTarget), Or<(Changed<CameraUp>, Changed<CameraTarget>)>>,
        mut commands: Commands,
    ) {
        cameras.iter().for_each(|(entity, up, target)| {
            log::debug!("TargetCameraParam : 0");
            commands.entity(entity).insert(
                TargetCameraParam::create(up.0.clone(), target.0.clone())
            );
        });
    }

    pub fn sys_camera_renderer_modify(
        cameras: Query<
            (&Camera, &ViewerRenderersInfo, &CameraToScreen),
            Or<(Changed<Camera>, Changed<DirtyViewerRenderersInfo>)>
        >,
        mut renderercmds: ResMut<ActionListRendererModify>,
    ) {
        cameras.iter().for_each(|(enable, renderers, toscreen)| {
            let enable = enable.0;
            renderers.map.iter().for_each(|(k, v)| {
                let id_render = v.1.0;
                renderercmds.push(OpsRendererCommand::Active(id_render, enable));
                renderercmds.push(OpsRendererCommand::RenderToFinal(id_render, toscreen.0));
            });
        });

    }

