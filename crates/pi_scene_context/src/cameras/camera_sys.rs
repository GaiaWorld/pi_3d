use pi_ecs::{prelude::{Query, Commands, Res}, query::{Or, Changed, With}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Rotation3};
use pi_share::Share;

use crate::{transforms::{transform_node::{LocalPosition, LocalRotation}}, object::{GameObject, ObjectID}, cameras::{target_camera::TargetCameraParam}, renderers::{renderer::RenderSize, render_object::RendererID}};

use super::{camera::{EFreeCameraMode, CameraFov, CameraNearFar, CameraOrthSize, EFixedMode, CameraParam, CameraViewport}, command::SysCameraParamCommand};


pub struct TargetCameraEffectLocalRotation;
impl TSystemStageInfo for TargetCameraEffectLocalRotation {
}
#[setup]
impl TargetCameraEffectLocalRotation {
    #[system]
    pub fn calc(
        query_cameras: Query<GameObject, (ObjectID, &TargetCameraParam, &LocalPosition)>,
        mut rot_cmd: Commands<GameObject, LocalRotation>,
    ) {
        //  log::debug!("Target Camera Control Calc:");
        let coordsys = CoordinateSytem3::left();
        query_cameras.iter().for_each(|(obj, target_camera, lposition)| {
            let mut rotation = Rotation3::identity();
            target_camera.calc_rotation(&coordsys, &lposition.0, &mut rotation);
            rot_cmd.insert(obj, LocalRotation(rotation));
        });
    }
}

pub struct SysCameraParamUpdate;
impl TSystemStageInfo for SysCameraParamUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysCameraParamCommand::key()
        ]
    }
}
#[setup]
impl SysCameraParamUpdate {
    #[system]
    fn sys(
        cameras: Query<
            GameObject,
            (
                ObjectID,
                &EFreeCameraMode, &CameraFov, &CameraNearFar, &CameraOrthSize, &EFixedMode, &CameraViewport
            ),
            Or<(Changed<EFreeCameraMode>, Changed<CameraFov>, Changed<CameraNearFar>, Changed<CameraOrthSize>, Changed<EFixedMode>, Changed<CameraViewport>)>
        >,
        mut param_cmd: Commands<GameObject, CameraParam>,
    ) {
        cameras.iter().for_each(|(id_camera, mode, fov, nearfar, size, fixmode, viewport)| {
            let param = CameraParam::create(mode, fixmode, fov, nearfar, size, viewport);
            param_cmd.insert(id_camera, param);
        });
    }
}

pub struct SyeCameraRenderSizeUpdate;
impl TSystemStageInfo for SyeCameraRenderSizeUpdate {
}
#[setup]
impl SyeCameraRenderSizeUpdate {
    #[system]
    fn sys(
        window: Res<Share<winit::window::Window>>,
        cameras: Query<GameObject, &RendererID, With<CameraParam>>,
        mut rendersize_cmd: Commands<GameObject, RenderSize>,
    ) {
        let size = window.inner_size();
        cameras.iter().for_each(|id_renderer| {
            rendersize_cmd.insert(id_renderer.0, RenderSize::new(size.width, size.height));
        });
    }
}
