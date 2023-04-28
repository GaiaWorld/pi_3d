
use pi_engine_shell::prelude::*;

use crate::{transforms::{transform_node::*}, cameras::{target_camera::*}, renderers::{renderer::RenderSize, ViewerRenderersInfo}};

use super::{camera::*};


    pub fn sys_calc_target_camera_local_rot(
        query_cameras: Query<(ObjectID, &TargetCameraParam, &LocalPosition), Changed<TargetCameraParam>>,
        mut rot_cmd: Commands,
    ) {
        // let coordsys = CoordinateSytem3::left();
        // query_cameras.iter().for_each(|(obj, target_camera, lposition)| {
        //     let mut rotation = Rotation3::identity();
        //     target_camera.calc_rotation(&coordsys, &lposition.0, &mut rotation);
        //     rot_cmd.entity(obj).insert(LocalRotation(rotation));
        // });
    }

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
        log::debug!("CameraParam :");
        cameras.iter().for_each(|(id_camera, mode, fov, nearfar, size, fixmode, viewport)| {
            log::debug!("CameraParam : 0");
            let param = CameraParam::create(mode, fixmode, fov, nearfar, size, viewport);
            commands.entity(id_camera).insert(param);
        });
    }

    pub fn sys_change_camera_render_size(
        window: Res<PiRenderWindow>,
        cameras: Query<&ViewerRenderersInfo, With<Camera>>,
        mut commands: Commands,
    ) {
        cameras.iter().for_each(|renderers| {
            renderers.map.iter().for_each(|(_, v)| {
                commands.entity(v.1.0).insert(RenderSize::new(window.width, window.height));
            });
        });
    }

