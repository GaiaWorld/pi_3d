use std::mem::replace;

use pi_engine_shell::prelude::*;
use pi_render::{rhi::{device::RenderDevice}, renderer::bind_buffer::{BindBufferAllocator}};
use pi_scene_math::{Number, Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    object::{ObjectID, GameObject},
    viewer::{
        ViewerViewMatrix, ViewerProjectionMatrix, ViewerTransformMatrix, ViewerGlobalPosition, ViewerDirection,
        ModelList, ModelListAfterCulling, BindViewer, ViewerActive,
        command::{SingleRendererCommandList, ERendererCommand, Viewport, ActionViewer}, FlagModelList
    },
    renderers::{
        ViewerRenderersInfo, graphic::RendererGraphicDesc, render_object::RendererID, renderer::{RenderColorFormat, RenderColorClear, RenderDepthFormat, RenderDepthClear}, DirtyViewerRenderersInfo
    }, flags::{Enable}, commands::TCommandList,
};

use super::{free_camera::FreeCameraParam, target_camera::{TargetCameraParam, CameraUp, CameraTarget}, camera::{CameraParam, EFreeCameraMode, EFixedMode, CameraViewport, CameraFov, CameraNearFar, CameraOrthSize, CameraOrthograhicParam, Camera}, arc_rotate_camera::ArcRotateCamera};

pub struct ActionCamera;
impl ActionCamera {
    pub fn create_camera(
        commands: &mut EntityCommands,
        device: &PiRenderDevice,
        dynallocator: &mut BindBufferAllocator,
    ) {
        commands
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
            .insert(Enable(true))
            .insert(CameraUp(CoordinateSytem3::up()))
            .insert(CameraTarget(Vector3::new(0., 0., 1.)));
    
        if let Some(data) = BindViewer::new(dynallocator) {
            commands.insert(data);
        };
    }

    pub fn active_camera(
        commands: &mut EntityCommands,
        value: bool,
    ) {
        commands.insert(ViewerActive(value));
    }

    pub fn modify_camera(
        commands: &mut EntityCommands,
        action: ECameraModifyCommand,
    ) {
        match action {
            ECameraModifyCommand::ModifyMode(value) => {
                commands.insert(value);
            },
            ECameraModifyCommand::ModifyNearFar(near, far) => {
                commands.insert(CameraNearFar(near, far));
            },
            ECameraModifyCommand::ModifyFov(value) => {
                commands.insert(CameraFov(value));
            },
            ECameraModifyCommand::ModifyFixedMode(value) => {
                commands.insert(value);
            },
            ECameraModifyCommand::ModifyOrthSize(value) => {
                commands.insert(CameraOrthSize(value));
            }
            ECameraModifyCommand::Target(value) => {

            },
        }
    }

    pub fn modify_camera_render(
        commands: &mut Commands,
        camera: Entity,
        id_render: Entity,
        camera_data: (&CameraViewport, &mut ViewerRenderersInfo),
        desc: RendererGraphicDesc,
        render_graphic: &mut PiRenderGraph,
    ) {
        let (view_port, viewer_renderers) = camera_data;
        if let Some((render, id_render)) = viewer_renderers.map.get(&desc.curr) {
            commands.despawn(id_render.0);
            render_graphic.remove_node(render.curr.to_string());
        }
        viewer_renderers.map.insert(desc.curr.clone(), (desc.clone(), RendererID(id_render)));
        
        commands.enitty(camera).insert(DirtyViewerRenderersInfo);

        ActionViewer::create_viewer_renderer(commands, render_graphic, camera, id_render, desc);
    }

}

#[derive(Debug)]
pub enum ECameraModifyCommand {
    ModifyMode(EFreeCameraMode),
    ModifyNearFar(Number, Number),
    ModifyFov(Number),
    ModifyFixedMode(EFixedMode),
    ModifyOrthSize(Number),
    Target(Vector3),
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
        mut cameras: Query<(Entity, &CameraUp, &CameraTarget), Or<(Changed<CameraUp>, Changed<CameraTarget>)>>,
        mut commands: Commands,
    ) {
        cameras.iter().for_each(|(entity, up, target)| {
            commands.entity(entity).insert(
                TargetCameraParam::create(up.0.clone(), target.0.clone())
            );
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
        mut commands: Commands,
        mut render_cmds: ResMut<SingleRendererCommandList>,
    ) {
        cameras.iter().for_each(|(param, renderers, enable, view_active)| {
            let enable = enable.0 && view_active.0;
            renderers.map.iter().for_each(|(k, v)| {
                let id_render = v.1.0;
                let &mut entitycmd = commands.entity(id_render);
                entitycmd.insert(Enable(enable));

                if enable {
                    ActionViewer::modify_viewer_renderer(&mut entitycmd, ERendererCommand::RenderColorFormat(RenderColorFormat(wgpu::TextureFormat::Rgba8UnormSrgb)));
                    ActionViewer::modify_viewer_renderer(&mut entitycmd, ERendererCommand::RenderColorClear(RenderColorClear(wgpu::Color { r: 0., g: 0., b: 0., a: 0. })));
                    ActionViewer::modify_viewer_renderer(&mut entitycmd, ERendererCommand::RenderDepthFormat(RenderDepthFormat(Some(wgpu::TextureFormat::Depth32Float))));
                    ActionViewer::modify_viewer_renderer(&mut entitycmd, ERendererCommand::RenderDepthClear(RenderDepthClear(0.)));
                }
            });
        });

    }
// }
