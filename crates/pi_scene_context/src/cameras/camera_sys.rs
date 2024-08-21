
use pi_scene_shell::prelude::*;

use super::camera::*;

    pub fn sys_update_camera_param(
        mut cameras: Query<
            (
                &CameraFov, &CameraOrthSize, &mut CameraParam,
            ),
            Or<(Changed<CameraFov>, Changed<CameraOrthSize>)>
        >,
    ) {
        // log::debug!("CameraParam :");
        cameras.iter_mut().for_each(|(fov, size, mut param)| {
            param.fov = fov.clone();
            param.orth = size.clone();
        });
    }
