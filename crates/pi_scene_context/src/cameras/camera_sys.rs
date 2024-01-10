
use pi_scene_shell::prelude::*;

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
        mut cameras: Query<
            (
                &EFreeCameraMode, &CameraFov, &CameraNearFar, &CameraOrthSize, &EFixedMode, &mut CameraParam,
            ),
            Or<(Changed<EFreeCameraMode>, Changed<CameraFov>, Changed<CameraNearFar>, Changed<CameraOrthSize>, Changed<EFixedMode>)>
        >,
    ) {
        // log::debug!("CameraParam :");
        cameras.iter_mut().for_each(|(mode, fov, nearfar, size, fixmode, mut param)| {
            // log::debug!("CameraParam : 0");
            *param = CameraParam::create(mode, fixmode, fov, nearfar, size);
        });
    }
