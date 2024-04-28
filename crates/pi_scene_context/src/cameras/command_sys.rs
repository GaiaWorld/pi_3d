
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    viewer::{prelude::*, command_sys::ActionViewer},
    transforms::command_sys::ActionTransformNode,
    layer_mask::prelude::*, prelude::{SceneMainCameraID, SceneID},
};

use super::{
    target_camera::*,
    camera::*,
    command::*,
};

pub fn sys_create_camera(
    mut cmds: ResMut<ActionListCameraCreate>,
    mut commands: Commands,
    mut dynallocator: ResMut<ResBindBufferAllocator>,
    mut errors: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraCreation(scene, entity)| {
        if let Some(mut commands) = commands.get_entity(entity) {

            ActionCamera::init(&mut commands, scene);

            if let Some(bindviewer) = BindViewer::new(&mut dynallocator) {
                commands.insert(bindviewer);
            } else {
                errors.record(entity, ErrorRecord::ERROR_BIND_VIEWER_CREATE_FAIL);
            }
        }
    })
}


pub fn sys_act_camera_mode(
    mut cmds: ResMut<ActionListCameraModify>,
    mut active_cameras: Query<(&SceneID, &mut Camera, &mut ViewerActive)>,
    mut scenes: Query<&mut SceneMainCameraID>,
    mut cameras: Query<(&mut CameraParam, &mut ViewerDistanceCompute)>,
    mut fov_cameras: Query<(&mut CameraFov, &mut RecordCameraFov)>,
    mut orth_cameras: Query<(&mut CameraOrthSize, &mut RecordCameraOrthSize)>,
    mut aspect_cameras: Query<&mut ViewerAspect>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraModify(entity, mode)| {
        match mode {
            ECameraModify::FreeMode(val) => if let Ok((mut camera, mut distance)) = cameras.get_mut(entity) {
                if camera.mode != val {
                    camera.mode = val;
                }
                match val {
                    EFreeCameraMode::Perspective => *distance = ViewerDistanceCompute::Base,
                    EFreeCameraMode::Orthograhic => *distance = ViewerDistanceCompute::Direction,
                }
            } else {
                cmds.push(OpsCameraModify(entity, ECameraModify::FreeMode(val)));
            },
            ECameraModify::Active(val) => if let Ok((idscene, mut camera, mut viewer)) = active_cameras.get_mut(entity) {
                // log::warn!("CameraActive {:?}, New {:?}", viewer, mode);
                if camera.0 != val {
                    *camera = Camera(val);
                    *viewer = ViewerActive(val);
                    // log::warn!("CameraActive Ok");
                }
                if val {
                    if let Ok(mut maincamera) = scenes.get_mut(idscene.0) {
                        *maincamera = SceneMainCameraID(Some(entity));
                    }
                }
            } else {
                cmds.push(OpsCameraModify(entity, ECameraModify::Active(val)));
            },
            ECameraModify::FixMode(val) => if let Ok((mut camera, _)) = cameras.get_mut(entity) {
                if camera.fixed_mode != val {
                    camera.fixed_mode = val;
                }
            } else {
                cmds.push(OpsCameraModify(entity, ECameraModify::FixMode(val)));
            },
            ECameraModify::Fov(val) => if let Ok((mut camera, mut record)) = fov_cameras.get_mut(entity) {
                record.0 = CameraFov(val);
                *camera = CameraFov(val);
            } else {
                cmds.push(OpsCameraModify(entity, ECameraModify::Fov(val)));
            },
            ECameraModify::OrthSize(val) => if let Ok((mut camera, mut record)) = orth_cameras.get_mut(entity) {
                record.0 = CameraOrthSize(val);
                *camera = CameraOrthSize(val);
            } else {
                cmds.push(OpsCameraModify(entity, ECameraModify::OrthSize(val)));
            },
            ECameraModify::Aspect(val) => if let Ok(mut camera) = aspect_cameras.get_mut(entity) {
                camera.0 = val;
            } else {
                cmds.push(OpsCameraModify(entity, ECameraModify::Aspect(val)));
            },
            ECameraModify::NearFar(near, far) => if let Ok((mut camera, _)) = cameras.get_mut(entity) {
                camera.nearfar = CameraNearFar(near, far);
            } else {
                cmds.push(OpsCameraModify(entity, ECameraModify::NearFar(near, far)));
            },
        }
    });
}


pub fn sys_act_camera_aspect(
    mut target_cameras: Query<&mut CameraTarget>,
    mut target_cmds: ResMut<ActionListCameraTarget>,
) {
    target_cmds.drain().drain(..).for_each(|OpsCameraTarget(entity, target)| {
        if let Ok(mut camera) = target_cameras.get_mut(entity) {
            *camera = CameraTarget(target);
        } else {
            target_cmds.push(OpsCameraTarget(entity, target))
        }
    });
}


pub struct ActionCamera;
impl ActionCamera {
    pub fn init(
        commands: &mut EntityCommands,
        scene: Entity,
    ) {
        ActionTransformNode::init(commands, scene);
        ActionCamera::as_camera(commands);
    }
    pub(crate) fn as_camera(
        commands: &mut EntityCommands,
    ) {
        commands.insert(Camera(false))
            .insert(ViewerDistanceCompute::default())
            .insert(CameraFov::default())
            .insert(CameraOrthSize::default())
            .insert(RecordCameraFov::default())
            .insert(RecordCameraOrthSize::default()) 
            .insert(LayerMask::default())
            .insert(CameraUp(CoordinateSytem3::up()))
            .insert(CameraTarget(Vector3::new(0., 0., 1.)))
            .insert(TargetCameraParam::default())
            .insert(CameraParam::default())
            ;
        
        ActionViewer::as_viewer(commands, false);
    }
}

    pub fn sys_update_target_camera_modify(
        mut cameras: Query<(&CameraUp, &CameraTarget, &mut TargetCameraParam), Or<(Changed<CameraUp>, Changed<CameraTarget>)>>,
    ) {
        cameras.iter_mut().for_each(|(up, target, mut param)| {
            *param = TargetCameraParam::create(up.0.clone(), target.0.clone());
        });
    }


