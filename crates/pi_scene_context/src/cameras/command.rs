use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, EntityDelete, Commands}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{Number};

use crate::{bytes_write_to_memory, object::{ObjectID, GameObject}, viewer::{ViewerViewMatrix, ViewerProjectionMatrix, ViewerTransformMatrix, ViewerGlobalPosition, ViewerDirection}};

use super::{free_camera::FreeCameraParam, target_camera::TargetCameraParam, camera::{CameraParam, EFreeCameraMode, EFixedMode}};


#[derive(Debug, Default)]
pub struct SingleCameraCreateList {
    pub list: Vec<ObjectID>,
}
pub struct SysCameraCreate;
impl TSystemStageInfo for SysCameraCreate {

}
#[setup]
impl SysCameraCreate {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleCameraCreateList>,
        mut entity_delete: EntityDelete<GameObject>,
        mut param_cmd: Commands<GameObject, CameraParam>,
        mut view_cmd: Commands<GameObject, ViewerViewMatrix>,
        mut proj_cmd: Commands<GameObject, ViewerProjectionMatrix>,
        mut tran_cmd: Commands<GameObject, ViewerTransformMatrix>,
        mut gpos_cmd: Commands<GameObject, ViewerGlobalPosition>,
        mut vdir_cmd: Commands<GameObject, ViewerDirection>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|obj| {
            param_cmd.insert(obj.clone(), CameraParam::default());
            view_cmd.insert(obj.clone(), ViewerViewMatrix::default());
            proj_cmd.insert(obj.clone(), ViewerProjectionMatrix::default());
            tran_cmd.insert(obj.clone(), ViewerTransformMatrix::default());
            gpos_cmd.insert(obj.clone(), ViewerGlobalPosition::default());
            vdir_cmd.insert(obj.clone(), ViewerDirection::default());
        });
    }
}

#[derive(Debug)]
pub enum CameraCommand {
    Destroy(ObjectID),
    ModifyMode(ObjectID, EFreeCameraMode),
    ModifyFov(ObjectID, Number),
    ModifyFixedMode(ObjectID, EFixedMode),
    ModifyOrthSize(ObjectID, Number),
}


#[derive(Debug, Default)]
pub struct SingleCameraCommandList {
    pub list: Vec<CameraCommand>,
}

pub struct SysCameraCommand;
impl TSystemStageInfo for SysCameraCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysCameraCreate::key()
        ]
    }
}
#[setup]
impl SysCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleCameraCommandList>,
        mut cameras: Query<GameObject, &mut CameraParam>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
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
                            camera.orth_size = value;
                            camera.dirty = true;
                        },
                        None => {
                            cmds.list.push(cmd);
                        },
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
impl TSystemStageInfo for SysTargetCameraCommand {
}
#[setup]
impl SysTargetCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleTargetCameraCommandList>,
        mut entity_delete: EntityDelete<GameObject>,
        mut camera_cmd: Commands<GameObject, TargetCameraParam>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TargetCameraCommand::Create(entity) => {
                    camera_cmd.insert(entity, TargetCameraParam::default());
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
impl TSystemStageInfo for SysFreeCameraCommand {
}
#[setup]
impl SysFreeCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleFreeCameraCommandList>,
        mut cameras: Commands<GameObject, FreeCameraParam>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                FreeCameraCommand::Create(entity) => {
                    cameras.insert(entity, FreeCameraParam::default());
                },
            }
        });

    }
}
