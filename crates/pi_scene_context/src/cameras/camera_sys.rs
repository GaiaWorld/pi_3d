
use pi_engine_shell::prelude::*;

use crate::viewer::prelude::*;

use super::camera::*;


    // pub fn sys_calc_target_camera_local_rot(
    //     query_cameras: Query<(ObjectID, &TargetCameraParam, &LocalPosition), Changed<TargetCameraParam>>,
    //     mut rot_cmd: Commands,
    // ) {
    //     // let coordsys = CoordinateSytem3::left();
    //     // query_cameras.iter().for_each(|(obj, target_camera, lposition)| {
    //     //     let mut rotation = Rotation3::identity();
    //     //     target_camera.calc_rotation(&coordsys, &lposition.0, &mut rotation);
    //     //     rot_cmd.entity(obj).insert(LocalRotation(rotation));
    //     // });
    // }

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
        // log::debug!("CameraParam :");
        cameras.iter().for_each(|(id_camera, mode, fov, nearfar, size, fixmode, viewport)| {
            // log::debug!("CameraParam : 0");
            let param = CameraParam::create(mode, fixmode, fov, nearfar, size, viewport);
            if let Some(mut cmd) = commands.get_entity(id_camera) {
                cmd.insert(param);
            }
        });
    }

    pub fn sys_change_camera_render_size(
        window: Res<PiRenderWindow>,
        mut cameras: Query<(&mut ViewerSize, &CameraToScreen), With<Camera>>,
    ) {
        cameras.iter_mut().for_each(|(mut viewersize, toscreen)| {
            if toscreen.0 {
                *viewersize = ViewerSize(window.width, window.height);
            }
        });
    }

