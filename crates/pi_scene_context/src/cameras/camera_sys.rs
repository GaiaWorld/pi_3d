
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
                &CameraFov, &CameraOrthSize, &mut CameraParam,
            ),
            (Changed<CameraFov>, Changed<CameraOrthSize>)
        >,
    ) {
        // log::debug!("CameraParam :");
        cameras.iter_mut().for_each(|(fov, size, mut param)| {
            param.fov = fov.clone();
            param.orth = size.clone();
        });
    }
