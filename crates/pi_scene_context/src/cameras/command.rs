
use pi_engine_shell::prelude::*;
use pi_scene_math::{Number, Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    viewer::{
        BindViewer, ViewerActive,
        command::{Viewport, ActionViewer},
    },
    renderers::{
        ViewerRenderersInfo, graphic::RendererGraphicDesc, render_object::RendererID, renderer::*, DirtyViewerRenderersInfo, command::{ActionRenderer, ERendererCommand}
    }, 
    flags::{Enable, UniqueName}, 
    animation::command::ActionAnime,
    scene::command::ActionScene,
    transforms::command::ActionTransformNode,
    layer_mask::LayerMask, prelude::ActionListTransformNodeParent,
};

use super::{target_camera::{TargetCameraParam, CameraUp, CameraTarget}, camera::{EFreeCameraMode, EFixedMode, CameraViewport, CameraFov, CameraNearFar, CameraOrthSize, Camera}};

pub struct OpsCameraCreation(pub Entity, pub Entity, pub String);
impl OpsCameraCreation {
    pub fn ops(scene: Entity, entity: Entity, name: String) -> Self {
        Self(scene, entity, name)
    }
}
pub type ActionListCameraCreate = ActionList<OpsCameraCreation>;
pub fn sys_camera_create(
    mut cmds: ResMut<ActionListCameraCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    mut dynallocator: ResMut<ResBindBufferAllocator>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraCreation(scene, entity, name)| {
        let mut commands = commands.entity(entity);

        ActionScene::add_to_scene(&mut commands, &mut tree, scene);
        ActionTransformNode::init_for_tree(&mut commands);
        ActionTransformNode::as_transform_node(&mut commands, name);
        ActionCamera::as_camera(&mut commands);
        ActionAnime::as_anime_group_target(&mut commands);

        if let Some(bindviewer) = BindViewer::new(&mut dynallocator) {
            commands.insert(bindviewer);
        }
    })
}

pub struct OpsCameraMode(Entity, EFreeCameraMode);
impl OpsCameraMode {
    pub fn ops(camera: Entity, as_orthograhic: bool) -> Self {
        if as_orthograhic {
            Self(camera, EFreeCameraMode::Orthograhic)
        } else {
            Self(camera, EFreeCameraMode::Perspective)
        }
    }
}
pub type ActionListCameraMode = ActionList<OpsCameraMode>;
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


pub struct OpsCameraActive(Entity, bool);
impl OpsCameraActive {
    pub fn ops(camera: Entity, active: bool) -> Self {
        Self(camera, active)
    }
}
pub type ActionListCameraActive = ActionList<OpsCameraActive>;
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


pub struct OpsCameraFixedMode(Entity, EFixedMode);
impl OpsCameraFixedMode {
    pub fn ops(camera: Entity, as_horizontal: bool) -> Self {
        if as_horizontal {
            Self(camera, EFixedMode::HorizontalFixed)
        } else {
            Self(camera, EFixedMode::VerticalFixed)
        }
    }
}
pub type ActionListCameraFixedMode = ActionList<OpsCameraFixedMode>;
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

pub struct OpsCameraNearFar(Entity, CameraNearFar);
impl OpsCameraNearFar {
    pub fn ops(camera: Entity, near: Number, far: Number) -> Self {
        Self(camera, CameraNearFar(near, far))
    }
}
pub type ActionListCameraNearFar = ActionList<OpsCameraNearFar>;
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

pub struct OpsCameraFov(Entity, CameraFov);
impl OpsCameraFov {
    pub fn ops(camera: Entity, fov: Number) -> Self {
        Self(camera, CameraFov(fov))
    }
}
pub type ActionListCameraFov = ActionList<OpsCameraFov>;
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

pub struct OpsCameraOrthSize(Entity, CameraOrthSize);
impl OpsCameraOrthSize {
    pub fn ops(camera: Entity, size: Number) -> Self {
        Self(camera, CameraOrthSize(size))
    }
}
pub type ActionListCameraOrthSize = ActionList<OpsCameraOrthSize>;
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

pub struct OpsCameraTarget(Entity, Vector3);
impl OpsCameraTarget {
    pub fn ops(camera: Entity, x: Number, y: Number, z: Number) -> Self {
        Self(camera, Vector3::new(x, y, z))
    }
}
pub type ActionListCameraTarget = ActionList<OpsCameraTarget>;
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

pub struct OpsCameraRendererInit(Entity, Entity, RendererGraphicDesc, wgpu::TextureFormat, Option<wgpu::TextureFormat>);
impl OpsCameraRendererInit {
    pub fn ops(
        camera: Entity,
        renderer: Entity,
        render_desc: RendererGraphicDesc,
        render_target_color_format: wgpu::TextureFormat,
        render_target_depth_stencil_format: Option<wgpu::TextureFormat>,
    ) -> Self {
        Self(camera, renderer, render_desc, render_target_color_format, render_target_depth_stencil_format)
    }
}
pub type ActionListCameraRenderer = ActionList<OpsCameraRendererInit>;
pub fn sys_camera_renderer_action(
    mut cmds: ResMut<ActionListCameraRenderer>,
    mut commands: Commands,
    mut viewers: Query<
        (
            &Camera, &mut ViewerRenderersInfo, &UniqueName
        )
    >,
    mut render_graphic: ResMut<PiRenderGraph> ,
) {
    cmds.drain().drain(..).for_each(|OpsCameraRendererInit(id_viewer, id_renderer, graphic_desc, color_format, depth_stencil_format)| {
        
        if let Ok((enable, mut viewer_renderers, name)) = viewers.get_mut(id_viewer) {

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
                    commands.entity(id_viewer).insert(DirtyViewerRenderersInfo);

                    viewer_renderers.map.insert(graphic_desc.curr.clone(), (graphic_desc.clone(), RendererID(id_renderer)));
                    
                    let mut commands = commands.entity(id_renderer);
                    ActionRenderer::as_renderer(
                        &mut commands, node, id_viewer, graphic_desc
                    );
                    ActionRenderer::modify(&mut commands, ERendererCommand::Active(true));
                    ActionRenderer::modify(&mut commands, ERendererCommand::ColorFormat(RenderColorFormat(color_format)));
                    ActionRenderer::modify(&mut commands, ERendererCommand::DepthFormat(RenderDepthFormat(depth_stencil_format)));
                },
                Err(_) => {},
            }
        } else {
            cmds.push(OpsCameraRendererInit(id_viewer, id_renderer, graphic_desc, color_format, depth_stencil_format));
        }
    });
}

// pub struct OpsCameraRendererModify(Entity, String, )

pub struct BundleCamera(
    Camera,
    EFreeCameraMode,
    EFixedMode,
    CameraNearFar,
    CameraOrthSize,
    CameraViewport,
    LayerMask,
    CameraUp,
    CameraTarget,
);

pub struct ActionCamera;
impl ActionCamera {
    pub(crate) fn as_camera(
        commands: &mut EntityCommands,
    ) {
        commands.insert(Camera(false))
            .insert(EFreeCameraMode::Orthograhic)
            .insert(EFixedMode::HorizontalFixed)
            .insert(CameraFov(0.75))
            .insert(CameraNearFar(0.1, 1000.0))
            .insert(CameraOrthSize(4.))
            .insert(CameraViewport::default())
            .insert(LayerMask::default())
            .insert(CameraUp(CoordinateSytem3::up()))
            .insert(CameraTarget(Vector3::new(0., 0., 1.)));
        
        ActionViewer::as_viewer(commands);
    }
    pub fn create(
        app: &mut App,
        scene: Entity,
        name: String,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);
        let entity = commands.spawn_empty().id();

        queue.apply(&mut app.world);

        let mut cmds = app.world.get_resource_mut::<ActionListCameraCreate>().unwrap();
        cmds.push(OpsCameraCreation(scene, entity, name));

        entity
    }

    pub fn active_camera(
        commands: &mut EntityCommands,
        value: bool,
    ) {
        commands.insert(ViewerActive(value));
    }

    pub fn modify_renderer(
        commands: &mut SingleCameraRendererCmds,
        camera: Entity,
        desc: RendererGraphicDesc,
    ) {
        commands.0.push((camera, desc));
    }

}

#[derive(Debug)]
pub enum ECameraModifyCommand {
    Enable(Entity, bool),
    ModifyMode(Entity, EFreeCameraMode),
    ModifyNearFar(Entity, Number, Number),
    ModifyFov(Entity, Number),
    ModifyFixedMode(Entity, EFixedMode),
    ModifyOrthSize(Entity, Number),
    // Target(Entity, Vector3),
}

#[derive(Default, Resource)]
pub struct SingleCameraRendererCmds(pub Vec<(Entity, RendererGraphicDesc)>);

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
            (&Camera, &ViewerRenderersInfo),
            Or<(Changed<Camera>, Changed<DirtyViewerRenderersInfo>)>
        >,
        mut commands: Commands,
    ) {
        cameras.iter().for_each(|(enable, renderers)| {
            let enable = enable.0;
            renderers.map.iter().for_each(|(k, v)| {
                let id_render = v.1.0;
                ActionRenderer::modify(&mut commands.entity(id_render), ERendererCommand::Active(enable));
            });
        });

    }

