use pi_ecs::{prelude::Query, query::{Write, With}};
use pi_ecs_macros::setup;

use crate::object::GameObject;
use super::{camera::{CameraParam, CameraViewMatrix, CameraGlobalPosition, CameraProjectionMatrix, CameraTransformMatrix, CameraDirection}};



#[derive(Debug)]
pub struct DirtyCamera;

#[derive(Debug)]
pub struct DirtyTargetCamera;

pub struct SysDirtyCameraParamTick;
#[setup]
impl SysDirtyCameraParamTick {
    #[system]
    pub fn tick(
        mut query_cameras: Query<GameObject, (Write<DirtyCamera>), With<DirtyCamera>>,
    ) {
        query_cameras.iter_mut().for_each(|(mut camera)| {
            camera.remove();
        });
    }
}

pub struct SysDirtyTargetCameraTick;
#[setup]
impl SysDirtyTargetCameraTick {
    #[system]
    pub fn tick(
        mut query_cameras: Query<GameObject, (Write<DirtyTargetCamera>), With<DirtyTargetCamera>>,
    ) {
        query_cameras.iter_mut().for_each(|(mut camera)| {
            camera.remove();
        });
    }
}

pub struct SysDirtyCameraRenderDataTick;
#[setup]
impl SysDirtyCameraRenderDataTick {
    #[system]
    pub fn tick(
        mut query_cameras: Query<GameObject, (&mut CameraViewMatrix, &mut CameraProjectionMatrix, &mut CameraTransformMatrix, &mut CameraGlobalPosition, &mut CameraDirection, With<DirtyCamera>)>,
    ) {
        // query_cameras.iter_mut().for_each(|(mut view_matrix, mut project_matrix, mut transform_matrix, mut global, mut direction, _)| {
        //     view_matrix.1 = false;
        //     project_matrix.1 = false;
        //     transform_matrix.1 = false;
        //     global.1 = false;
        //     direction.1 = false;
        // });
    }
}