use std::mem::replace;

use pi_engine_shell::prelude::*;
use pi_render::{rhi::{device::RenderDevice}, renderer::bind_buffer::{BindBufferAllocator}};
use pi_scene_math::{Number, Vector3};

use crate::{
    object::{ObjectID, GameObject},
    viewer::{
        ViewerViewMatrix, ViewerProjectionMatrix, ViewerTransformMatrix, ViewerGlobalPosition, ViewerDirection,
        ModelList, ModelListAfterCulling, BindViewer, ViewerActive,
        command::{SingleRendererCommandList, ERendererCommand, Viewport}, FlagModelList
    },
    renderers::{
        ViewerRenderersInfo, graphic::RendererGraphicDesc, render_object::RendererID, renderer::{RenderColorFormat, RenderColorClear, RenderDepthFormat, RenderDepthClear}, DirtyViewerRenderersInfo
    }, flags::{Enable, enable::SingleEnableCommands}, commands::TCommandList,
};

use super::{free_camera::FreeCameraParam, target_camera::TargetCameraParam, camera::{CameraParam, EFreeCameraMode, EFixedMode, CameraViewport, CameraFov, CameraNearFar, CameraOrthSize, CameraOrthograhicParam, Camera}, arc_rotate_camera::ArcRotateCamera};

#[derive(Debug, Clone)]
pub enum ECameraCreateCommand {
    Active(ObjectID, bool),
    FreeCamera(ObjectID),
    ArcRotateCamera(ObjectID),
}

#[derive(Debug, Default, Resource)]
pub struct SingleCameraCreateList {
    pub list: Vec<ECameraCreateCommand>,
}
// pub struct SysCameraCreate;
// impl TSystemStageInfo for SysCameraCreate {

// }
// #[setup]
// impl SysCameraCreate {
//     #[system]
    pub fn sys_cmds_camera_create(
        mut cmds: ResMut<SingleCameraCreateList>,
        mut commands: Commands,
        device: Res<PiRenderDevice>,
        mut dynallocator: ResMut<BindBufferAllocator>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECameraCreateCommand::FreeCamera(obj) => {
                    let bundle = commands.entity(obj)
                        .insert(Camera)
                        .insert(EFreeCameraMode::Orthograhic)
                        .insert(EFixedMode::HorizontalFixed)
                        .insert(CameraFov(0.75))
                        .insert(CameraNearFar(0.1, 1000.0))
                        .insert(CameraOrthSize(4.))
                        .insert(CameraViewport::default())
                        .insert(ViewerViewMatrix::default())
                        .insert(ViewerProjectionMatrix::default())
                        .insert(ViewerTransformMatrix::default())
                        .insert(ViewerGlobalPosition::default())
                        .insert(ViewerDirection::default())
                        .insert(ModelList::default())
                        .insert(FlagModelList::default())
                        .insert(ModelListAfterCulling::default())
                        .insert(TargetCameraParam::default())
                        .insert(ViewerActive(false))
                        .insert(ViewerRenderersInfo::default())
                        .insert(DirtyViewerRenderersInfo)
                        .insert(Enable(true));

                    if let Some(data) = BindViewer::new(&mut dynallocator) {
                        bundle.insert(data);
                    };
                },
                ECameraCreateCommand::ArcRotateCamera(obj) => {

                },
                ECameraCreateCommand::Active(entity, value) => {
                    commands.entity(entity).insert(ViewerActive(value));
                },
            }
        });
    }
// }

#[derive(Debug)]
pub enum ECameraCommand {
    Renderer(ObjectID, ObjectID, RendererGraphicDesc),
    ModifyMode(ObjectID, EFreeCameraMode),
    ModifyNearFar(ObjectID, Number, Number),
    ModifyFov(ObjectID, Number),
    ModifyFixedMode(ObjectID, EFixedMode),
    ModifyOrthSize(ObjectID, Number),
}


#[derive(Debug, Default, Resource)]
pub struct SingleCameraCommandList {
    pub list: Vec<ECameraCommand>,
}

// pub struct SysCameraParamCommand;
// impl TSystemStageInfo for SysCameraParamCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![

//         ]
//     }
// }
// #[setup]
// impl SysCameraParamCommand {
//     #[system]
    pub fn sys_cmds_camera_modify(
        mut cmds: ResMut<SingleCameraCommandList>,
        mut cameras: Query<(&CameraViewport, &mut ViewerRenderersInfo)>,
        mut render_cmds: ResMut<SingleRendererCommandList>,

        mut commands: Commands,

        // mut mode_cmd: Commands<EFreeCameraMode>,
        // mut fixmode_cmd: Commands<EFixedMode>,
        // mut fov_cmd: Commands<CameraFov>,
        // mut near_far_cmd: Commands<CameraNearFar>,
        // mut orth_size_cmd: Commands<CameraOrthSize>,
        // mut viewport_cmd: Commands<CameraViewport>,
        // mut dirty_viewer_render_cmd: Commands<DirtyViewerRenderersInfo>,

        mut render_graphic: ResMut<PiRenderGraph>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECameraCommand::ModifyMode(entity, value) => {
                    commands.entity(entity).in
                    mode_cmd.insert(entity, value);
                },
                ECameraCommand::ModifyNearFar(entity, near, far) => {
                    near_far_cmd.insert(entity, CameraNearFar(near, far));
                },
                ECameraCommand::ModifyFov(entity, value) => {
                    fov_cmd.insert(entity, CameraFov(value));
                },
                ECameraCommand::ModifyFixedMode(entity, value) => {
                    fixmode_cmd.insert(entity, value);
                },
                ECameraCommand::ModifyOrthSize(entity, value) => {
                    orth_size_cmd.insert(entity, CameraOrthSize(value));
                },
                ECameraCommand::Renderer(entity, id_render, value) => {
                    if let Some((viewport, mut viewer_renderers)) = cameras.get_mut(entity) {

                        if let Some((render, id_render)) = viewer_renderers.map.get(&value.curr) {
                            entity_cmd.despawn(id_render.0);
                            render_graphic.remove_node(render.curr.to_string());
                        }
                        viewer_renderers.map.insert(value.curr.clone(), (value.clone(), RendererID(id_render)));
                        
                        render_cmds.list.push(
                            ERendererCommand::Active(entity, RendererID(id_render), value)
                        );
                        
                        dirty_viewer_render_cmd.insert(entity, DirtyViewerRenderersInfo);
                    }
                },
            }
        });

    }
// }

// TaqrgetCamera
#[derive(Debug)]
pub enum TargetCameraCommand {
    Target(ObjectID, Vector3),
}


#[derive(Debug, Default, Resource)]
pub struct SingleTargetCameraCommandList {
    pub list: Vec<TargetCameraCommand>,
}

// pub struct SysTargetCameraCommand;
// impl TSystemStageInfo for SysTargetCameraCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//         ]
//     }
// }
// #[setup]
// impl SysTargetCameraCommand {
//     #[system]
    pub fn sys_cmds_target_camera_modify(
        mut cmds: ResMut<SingleTargetCameraCommandList>,
        mut entity_delete: EntityDelete<GameObject>,
        mut cameras: Query<&TargetCameraParam>,
        mut targetcamera_cmd: Commands<TargetCameraParam>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TargetCameraCommand::Target(entity, target) => {
                    if let Some(camera) = cameras.get_mut(entity) {
                        let mut camera = camera.clone();
                        camera.target = target;
                        targetcamera_cmd.insert(entity, camera);
                    }
                },
            }
        });

    }
// }

// pub struct SysCameraRenderer;
// impl TSystemStageInfo for SysCameraRenderer {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysCameraParamCommand::key()
//         ]
//     }
// }
// #[setup]
// impl SysCameraRenderer {

    pub fn sys_cmds_camera_renderer_modify(
        mut cameras: Query<
            (&Camera, &ViewerRenderersInfo, &Enable, &ViewerActive),
            Or<(Changed<DirtyViewerRenderersInfo>, Changed<Enable>, Changed<ViewerActive>)>
        >,
        mut enable_cmds: Commands<Enable>,
        mut render_cmds: ResMut<SingleRendererCommandList>,
    ) {
        cameras.iter().for_each(|(param, renderers, enable, view_active)| {
            let enable = enable.0 && view_active.0;
            renderers.map.iter().for_each(|(k, v)| {
                let id_render = v.1.0;
                enable_cmds.insert(id_render, Enable(enable));
                if enable {
                    render_cmds.list.push(
                        ERendererCommand::RenderColorFormat(id_render, RenderColorFormat(wgpu::TextureFormat::Rgba8UnormSrgb))
                    );
                    render_cmds.list.push(
                        ERendererCommand::RenderColorClear(id_render, RenderColorClear(wgpu::Color { r: 0., g: 0., b: 0., a: 0. }))
                    );
                    render_cmds.list.push(
                        ERendererCommand::RenderDepthFormat(id_render, RenderDepthFormat(Some(wgpu::TextureFormat::Depth32Float)))
                    );
                    render_cmds.list.push(
                        ERendererCommand::RenderDepthClear(id_render, RenderDepthClear(0.))
                    );
                }
            });
        });

    }
// }
