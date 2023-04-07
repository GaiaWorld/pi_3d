
use pi_engine_shell::prelude::*;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Rotation3};
use pi_share::Share;

use crate::{transforms::{transform_node::{LocalPosition, LocalRotation}}, object::{GameObject, ObjectID}, cameras::{target_camera::TargetCameraParam}, renderers::{renderer::RenderSize, render_object::RendererID, ViewerRenderersInfo}};

use super::{camera::{EFreeCameraMode, CameraFov, CameraNearFar, CameraOrthSize, EFixedMode, CameraParam, CameraViewport, Camera}};


// pub struct TargetCameraEffectLocalRotation;
// impl TSystemStageInfo for TargetCameraEffectLocalRotation {
// }
// #[setup]
// impl TargetCameraEffectLocalRotation {
//     #[system]
    pub fn sys_calc_target_camera_local_rot(
        query_cameras: Query<(ObjectID, &TargetCameraParam, &LocalPosition), Changed<TargetCameraParam>>,
        mut rot_cmd: Commands,
    ) {
        // let coordsys = CoordinateSytem3::left();
        // query_cameras.iter().for_each(|(obj, target_camera, lposition)| {
        //     let mut rotation = Rotation3::identity();
        //     target_camera.calc_rotation(&coordsys, &lposition.0, &mut rotation);
        //     rot_cmd.insert(obj, LocalRotation(rotation));
        // });
    }
// }

// pub struct SysCameraParamUpdate;
// impl TSystemStageInfo for SysCameraParamUpdate {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysCameraParamCommand::key()
//         ]
//     }
// }
// #[setup]
// impl SysCameraParamUpdate {
//     #[system]
    pub(crate) fn sys_update_camera_param(
        cameras: Query<
            (
                ObjectID,
                &EFreeCameraMode, &CameraFov, &CameraNearFar, &CameraOrthSize, &EFixedMode, &CameraViewport
            ),
            Or<(Changed<EFreeCameraMode>, Changed<CameraFov>, Changed<CameraNearFar>, Changed<CameraOrthSize>, Changed<EFixedMode>, Changed<CameraViewport>)>
        >,
        mut commands: Commands,
    ) {
        cameras.iter().for_each(|(id_camera, mode, fov, nearfar, size, fixmode, viewport)| {
            let param = CameraParam::create(mode, fixmode, fov, nearfar, size, viewport);
            commands.entity(id_camera).insert(param);
        });
    }
// }

// pub struct SyeCameraRenderSizeUpdate;
// impl TSystemStageInfo for SyeCameraRenderSizeUpdate {
// }
// #[setup]
// impl SyeCameraRenderSizeUpdate {
//     #[system]
    pub fn sys_change_camera_render_size(
        window: Res<PiRenderWindow>,
        cameras: Query<&ViewerRenderersInfo, With<Camera>>,
        mut commands: Commands,
    ) {
        let size = window.inner_size();
        cameras.iter().for_each(|renderers| {
            renderers.map.iter().for_each(|(k, v)| {
                commands.entity(v.1.0).insert(RenderSize::new(size.width, size.height));
            });
        });
    }
// }
