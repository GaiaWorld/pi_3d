use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, EntityDelete, Commands, Res}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
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
        ViewerRenderersInfo, graphic::RendererGraphicDesc, render_object::RendererID, renderer::{RenderColorFormat, RenderColorClear, RenderDepthFormat, RenderDepthClear}
    },
};

use super::{free_camera::FreeCameraParam, target_camera::TargetCameraParam, camera::{CameraParam, EFreeCameraMode, EFixedMode, CameraViewport, CameraFov, CameraNearFar, CameraOrthSize, CameraOrthograhicParam}, arc_rotate_camera::ArcRotateCamera};

#[derive(Debug, Clone)]
pub enum ECameraCreateCommand {
    Active(ObjectID, bool),
    FreeCamera(ObjectID),
    ArcRotateCamera(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleCameraCreateList {
    pub list: Vec<ECameraCreateCommand>,
}
pub struct SysCameraCreate;
impl TSystemStageInfo for SysCameraCreate {

}
#[setup]
impl SysCameraCreate {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleCameraCreateList>,
        mut mode_cmd: Commands<GameObject, EFreeCameraMode>,
        mut fixmode_cmd: Commands<GameObject, EFixedMode>,
        mut fov_cmd: Commands<GameObject, CameraFov>,
        mut near_far_cmd: Commands<GameObject, CameraNearFar>,
        mut orth_size_cmd: Commands<GameObject, CameraOrthSize>,
        mut viewport_cmd: Commands<GameObject, CameraViewport>,
        mut view_cmd: Commands<GameObject, ViewerViewMatrix>,
        mut proj_cmd: Commands<GameObject, ViewerProjectionMatrix>,
        mut tran_cmd: Commands<GameObject, ViewerTransformMatrix>,
        mut gpos_cmd: Commands<GameObject, ViewerGlobalPosition>,
        mut vdir_cmd: Commands<GameObject, ViewerDirection>,
        mut list_model_cmd: Commands<GameObject, ModelList>,
        mut flag_list_model_cmd: Commands<GameObject, FlagModelList>,
        mut list_culling_cmd: Commands<GameObject, ModelListAfterCulling>,
        mut viewer_active_cmd: Commands<GameObject, ViewerActive>,
        mut viewer_bind_cmd: Commands<GameObject, BindViewer>,
        mut viewer_render_cmd: Commands<GameObject, ViewerRenderersInfo>,
        mut freecamera_cmd: Commands<GameObject, TargetCameraParam>,
        mut arccamera_cmd: Commands<GameObject, ArcRotateCamera>,
        device: Res<RenderDevice>,
        mut dynallocator: ResMut<BindBufferAllocator>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECameraCreateCommand::FreeCamera(obj) => {
                    
                    mode_cmd.insert(obj.clone(), EFreeCameraMode::Orthograhic);
                    fixmode_cmd.insert(obj.clone(), EFixedMode::HorizontalFixed);
                    fov_cmd.insert(obj.clone(), CameraFov(0.75));
                    near_far_cmd.insert(obj.clone(), CameraNearFar(0.1, 1000.0));
                    orth_size_cmd.insert(obj.clone(), CameraOrthSize(4.));
                    viewport_cmd.insert(obj.clone(), CameraViewport::default());

                    view_cmd.insert(obj.clone(), ViewerViewMatrix::default());
                    proj_cmd.insert(obj.clone(), ViewerProjectionMatrix::default());
                    tran_cmd.insert(obj.clone(), ViewerTransformMatrix::default());
                    gpos_cmd.insert(obj.clone(), ViewerGlobalPosition::default());
                    vdir_cmd.insert(obj.clone(), ViewerDirection::default());
                    list_model_cmd.insert(obj.clone(), ModelList::default());
                    flag_list_model_cmd.insert(obj.clone(), FlagModelList::default());
                    list_culling_cmd.insert(obj.clone(), ModelListAfterCulling::default());
                    freecamera_cmd.insert(obj.clone(), TargetCameraParam::default());
                    viewer_active_cmd.insert(obj.clone(), ViewerActive(false));
                    viewer_render_cmd.insert(obj.clone(), ViewerRenderersInfo::default());

                    if let Some(data) = BindViewer::new(&mut dynallocator) {
                        viewer_bind_cmd.insert(obj.clone(), data);
                    };
                },
                ECameraCreateCommand::ArcRotateCamera(obj) => {

                },
                ECameraCreateCommand::Active(entity, value) => {
                    viewer_active_cmd.insert(entity, ViewerActive(value));
                },
            }
        });
    }
}

#[derive(Debug)]
pub enum ECameraCommand {
    Renderer(ObjectID, ObjectID, RendererGraphicDesc),
    ModifyMode(ObjectID, EFreeCameraMode),
    ModifyNearFar(ObjectID, Number, Number),
    ModifyFov(ObjectID, Number),
    ModifyFixedMode(ObjectID, EFixedMode),
    ModifyOrthSize(ObjectID, Number),
}


#[derive(Debug, Default)]
pub struct SingleCameraCommandList {
    pub list: Vec<ECameraCommand>,
}

pub struct SysCameraParamCommand;
impl TSystemStageInfo for SysCameraParamCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![

        ]
    }
}
#[setup]
impl SysCameraParamCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleCameraCommandList>,
        mut cameras: Query<GameObject, &CameraViewport>,
        mut render_cmds: ResMut<SingleRendererCommandList>,

        mut viewer_active_cmd: Commands<GameObject, ViewerActive>,
        mut mode_cmd: Commands<GameObject, EFreeCameraMode>,
        mut fixmode_cmd: Commands<GameObject, EFixedMode>,
        mut fov_cmd: Commands<GameObject, CameraFov>,
        mut near_far_cmd: Commands<GameObject, CameraNearFar>,
        mut orth_size_cmd: Commands<GameObject, CameraOrthSize>,
        mut viewport_cmd: Commands<GameObject, CameraViewport>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECameraCommand::ModifyMode(entity, value) => {
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
                    if let Some(viewport) = cameras.get_mut(entity) {
                        render_cmds.list.push(
                            ERendererCommand::Active(entity, RendererID(id_render), value)
                        );
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
                },
            }
        });

    }
}

// TaqrgetCamera
#[derive(Debug)]
pub enum TargetCameraCommand {
    Target(ObjectID, Vector3),
}


#[derive(Debug, Default)]
pub struct SingleTargetCameraCommandList {
    pub list: Vec<TargetCameraCommand>,
}

pub struct SysTargetCameraCommand;
impl TSystemStageInfo for SysTargetCameraCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
        ]
    }
}
#[setup]
impl SysTargetCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleTargetCameraCommandList>,
        mut entity_delete: EntityDelete<GameObject>,
        mut cameras: Query<GameObject, &TargetCameraParam>,
        mut targetcamera_cmd: Commands<GameObject, TargetCameraParam>,
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
}

// FreeCamera
#[derive(Debug)]
pub enum FreeCameraCommand {
    Create(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleFreeCameraCommandList {
    pub list: Vec<FreeCameraCommand>,
}

pub struct SysFreeCameraCommand;
impl TSystemStageInfo for SysFreeCameraCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
        ]
    }
}
#[setup]
impl SysFreeCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleFreeCameraCommandList>,
        mut cameras: Query<GameObject, &mut FreeCameraParam>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                FreeCameraCommand::Create(entity) => {
                    // cameras.insert(entity, FreeCameraParam::default());
                },
            }
        });

    }
}
