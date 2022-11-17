use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, EntityDelete}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::{Number};

use crate::{bytes_write_to_memory, object::{ObjectID, GameObject}};

use super::{free_camera::FreeCameraParam, target_camera::TargetCameraParam, camera::{CameraTransformMatrix, CameraGlobalPosition, CameraDirection, CameraProjectionMatrix, CameraViewMatrix, CameraParam, EFreeCameraMode, EFixedMode}};

#[derive(Debug)]
enum CameraCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    ModifyMode(ObjectID, EFreeCameraMode),
    ModifyFov(ObjectID, Number),
    ModifyFixedMode(ObjectID, EFixedMode),
    ModifyOrthSize(ObjectID, Number),
}


#[derive(Debug, Default)]
struct SingleCameraCommandList {
    pub list: Vec<CameraCommand>,
}

struct SysCameraCommand;
#[setup]
impl SysCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleCameraCommandList>,
        mut cameras: Query<GameObject, (Write<CameraParam>, Write<CameraViewMatrix>, Write<CameraProjectionMatrix>, Write<CameraTransformMatrix>, Write<CameraGlobalPosition>, Write<CameraDirection>)>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                CameraCommand::Create(entity) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            camera.0.write(CameraParam::default());
                            camera.1.write(CameraViewMatrix::default());
                            camera.2.write(CameraProjectionMatrix::default());
                            camera.3.write(CameraTransformMatrix::default());
                            camera.4.write(CameraGlobalPosition::default());
                            camera.5.write(CameraDirection::default());
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::Destroy(entity) => {
                    entity_delete.despawn(entity);
                },
                CameraCommand::ModifyMode(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(camera) => {
                            // camera.get_or_default().mode = value;
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::ModifyFov(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            // camera.get_or_default().fov = value;
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::ModifyFixedMode(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            // camera.get_or_default().fixed_mode = value;
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::ModifyOrthSize(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            match camera.0.get_mut() {
                                Some(camera) => {
                                    camera.orth_size = value;
                                    camera.dirty = true;
                                },
                                None => todo!(),
                            }
                            camera.0.notify_modify();
                        },
                        None => todo!(),
                    }
                },
            }
        });

    }
}

// TaqrgetCamera
#[derive(Debug)]
pub enum TargetCameraCommand {
    Create(ObjectID),
    Destroy(ObjectID),
}


#[derive(Debug, Default)]
pub struct SingleTargetCameraCommandList {
    pub list: Vec<TargetCameraCommand>,
}

pub struct SysTargetCameraCommand;
#[setup]
impl SysTargetCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleTargetCameraCommandList>,
        mut cameras: Query<GameObject, Write<TargetCameraParam>>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TargetCameraCommand::Create(entity) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            camera.write(TargetCameraParam::default());
                        },
                        None => todo!(),
                    }
                },
                TargetCameraCommand::Destroy(entity) => {
                    entity_delete.despawn(entity);
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
#[setup]
impl SysFreeCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleFreeCameraCommandList>,
        mut cameras: Query<GameObject, Write<FreeCameraParam>>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                FreeCameraCommand::Create(entity) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            camera.write(FreeCameraParam::default());
                        },
                        None => todo!(),
                    }
                },
            }
        });

    }
}
