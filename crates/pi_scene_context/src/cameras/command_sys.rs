
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    flags::{CullingFlag, Enable, GlobalEnable, RecordEnable}, layer_mask::prelude::*, prelude::{DirtyViewerRenderersInfo, SceneID, SceneMainCameraID, ViewerRenderersInfo}, transforms::command_sys::ActionTransformNode, viewer::{command_sys::ActionViewer, prelude::*}
};

use super::{
    camera::*, command::*, target_camera::*, AbsoluteTransform, GlobalMatrix, LocalEulerAngles, LocalMatrix, LocalPosition, LocalRotation, LocalRotationQuaternion, LocalScaling, RecordLocalEulerAngles, RecordLocalPosition, RecordLocalRotationQuaternion, RecordLocalScaling, TransformNodeDirty
};

pub fn sys_create_camera(
    mut cmds: ResMut<ActionListCameraCreate>,
    
    mut alter1: Alter<(), (), (DisposeReady, DisposeCan), ()>,
    mut alter2: Alter<(), (), (SceneID,), ()>,
    mut alter3: Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable), ()>,
    mut alter4: Alter<
        (),
        (),
        (
            TransformNodeDirty,
            LocalPosition,
            LocalScaling,
            LocalRotationQuaternion,
            LocalEulerAngles,
            RecordLocalPosition,
            RecordLocalScaling,
            RecordLocalRotationQuaternion,
            RecordLocalEulerAngles,
            LocalRotation,
            LocalMatrix,
            GlobalMatrix,
            AbsoluteTransform,
            FlagAnimationStartResetComp,
            CullingFlag
        ), 
        ()>,
    mut commands: Alter<
        (),
        (),
        (
            Camera,
            ViewerDistanceCompute,
            CameraFov,
            CameraOrthSize,
            RecordCameraFov,
            RecordCameraOrthSize,
            LayerMask,
            CameraUp,
            CameraTarget,
            TargetCameraParam,
            CameraParam,
        ), 
        ()>,
    mut commands1: Alter<
        (),
        (),
        (
            Camera,
            ViewerDistanceCompute,
            CameraFov,
            CameraOrthSize,
            RecordCameraFov,
            RecordCameraOrthSize,
            LayerMask,
            CameraUp,
            CameraTarget,
            TargetCameraParam,
            CameraParam,
        ), 
        ()>,
    mut commands2: Alter<
        (),
        (), 
        (
            ViewerAspect,
            ViewerViewMatrix,
            ViewerProjectionMatrix,
            ViewerTransformMatrix,
            ViewerGlobalPosition,
            ViewerDirection,
            ModelList,
            FlagModelList,
            ModelListAfterCulling,
            ViewerActive,
            ViewerRenderersInfo,
            DirtyViewerRenderersInfo,
            ForceIncludeModelList,
            FlagForceIncludeModelList
        ),
        ()>,
    mut commands3: Alter<(),(), (BindViewer, ), ()>,
    mut dynallocator: ResMut<ResBindBufferAllocator>,
    mut errors: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraCreation(scene, entity)| {
        if commands.get(entity).is_ok() {

            ActionCamera::init(entity, &mut alter1, &mut alter2, &mut alter3, &mut alter4, &mut commands, &mut commands1, &mut commands2,  scene);

            if let Some(bindviewer) = BindViewer::new(&mut dynallocator) {
                commands3.alter(entity,(bindviewer,));
            } else {
                errors.record(entity, ErrorRecord::ERROR_BIND_VIEWER_CREATE_FAIL);
            }
        }
    })
}


pub fn sys_act_camera_mode(
    mut mode_cmds: ResMut<ActionListCameraMode>,
    mut active_cmds: ResMut<ActionListCameraActive>,
    mut active_cameras: Query<(&SceneID, &mut Camera, &mut ViewerActive)>,
    mut scenes: Query<&mut SceneMainCameraID>,
    mut fix_cmds: ResMut<ActionListCameraFixedMode>,
    mut nearfar_cmds: ResMut<ActionListCameraNearFar>,
    mut cameras: Query<(&mut CameraParam, &mut ViewerDistanceCompute)>,
) {
    mode_cmds.drain().drain(..).for_each(|OpsCameraMode(entity, mode)| {
        if let Ok((mut camera, mut distance)) = cameras.get_mut(entity) {
            if camera.mode != mode {
                camera.mode = mode;
            }
            match mode {
                EFreeCameraMode::Perspective => *distance = ViewerDistanceCompute::Base,
                EFreeCameraMode::Orthograhic => *distance = ViewerDistanceCompute::Direction,
            }
        } else {
            mode_cmds.push(OpsCameraMode(entity, mode))
        }
    });
    active_cmds.drain().drain(..).for_each(|OpsCameraActive(entity, mode)| {
        // log::warn!("CameraActive ");
        if let Ok((idscene, mut camera, mut viewer)) = active_cameras.get_mut(entity) {
            // log::warn!("CameraActive {:?}, New {:?}", viewer, mode);
            if camera.0 != mode {
                *camera = Camera(mode);
                *viewer = ViewerActive(mode);
                // log::warn!("CameraActive Ok");
            }
            if mode {
                if let Ok(mut maincamera) = scenes.get_mut(idscene.0) {
                    *maincamera = SceneMainCameraID(Some(entity));
                }
            }
        } else {
            active_cmds.push(OpsCameraActive(entity, mode))
        }
    });
    fix_cmds.drain().drain(..).for_each(|OpsCameraFixedMode(entity, mode)| {
        if let Ok((mut camera, _)) = cameras.get_mut(entity) {
            if camera.fixed_mode != mode {
                camera.fixed_mode = mode;
            }
        } else {
            fix_cmds.push(OpsCameraFixedMode(entity, mode))
        }
    });
    nearfar_cmds.drain().drain(..).for_each(|OpsCameraNearFar(entity, mode)| {
        if let Ok((mut camera, _)) = cameras.get_mut(entity) {
            camera.nearfar = mode;
        } else {
            nearfar_cmds.push(OpsCameraNearFar(entity, mode))
        }
    });
}


pub fn sys_act_camera_aspect(
    mut fov_cmds: ResMut<ActionListCameraFov>,
    mut fov_cameras: Query<(&mut CameraFov, &mut RecordCameraFov)>,
    mut orth_cmds: ResMut<ActionListCameraOrthSize>,
    mut orth_cameras: Query<(&mut CameraOrthSize, &mut RecordCameraOrthSize)>,
    mut aspect_cmds: ResMut<ActionListCameraAspect>,
    mut aspect_cameras: Query<&mut ViewerAspect>,
    mut target_cameras: Query<&mut CameraTarget>,
    mut target_cmds: ResMut<ActionListCameraTarget>,
) {
    fov_cmds.drain().drain(..).for_each(|OpsCameraFov(entity, mode)| {
        if let Ok((mut camera, mut record)) = fov_cameras.get_mut(entity) {
            record.0 = mode.clone();
            *camera = mode;
        } else {
            fov_cmds.push(OpsCameraFov(entity, mode))
        }
    });
    orth_cmds.drain().drain(..).for_each(|OpsCameraOrthSize(entity, mode)| {
        if let Ok((mut camera, mut record)) = orth_cameras.get_mut(entity) {
            record.0 = mode.clone();
            *camera = mode;
        } else {
            orth_cmds.push(OpsCameraOrthSize(entity, mode))
        }
    });
    aspect_cmds.drain().drain(..).for_each(|OpsCameraAspect(entity, val)| {
        if let Ok(mut camera) = aspect_cameras.get_mut(entity) {
            camera.0 = val;
        } else {
            aspect_cmds.push(OpsCameraAspect(entity, val))
        }
    });
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
        entity: Entity,
        alter1: &mut Alter<(), (), (DisposeReady, DisposeCan), ()>,
        alter2: &mut Alter<(), (), (SceneID,), ()>,
        alter3: &mut Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable), ()>,
        alter4: &mut Alter<
        (),
        (),
        (
            TransformNodeDirty,
            LocalPosition,
            LocalScaling,
            LocalRotationQuaternion,
            LocalEulerAngles,
            RecordLocalPosition,
            RecordLocalScaling,
            RecordLocalRotationQuaternion,
            RecordLocalEulerAngles,
            LocalRotation,
            LocalMatrix,
            GlobalMatrix,
            AbsoluteTransform,
            FlagAnimationStartResetComp,
            CullingFlag
        ), 
        ()>,
        commands: &mut Alter<
        (),
        (),
        (
            Camera,
            ViewerDistanceCompute,
            CameraFov,
            CameraOrthSize,
            RecordCameraFov,
            RecordCameraOrthSize,
            LayerMask,
            CameraUp,
            CameraTarget,
            TargetCameraParam,
            CameraParam,
        ), 
        ()>,
        commands1: &mut Alter<
        (),
        (),
        (
            Camera,
            ViewerDistanceCompute,
            CameraFov,
            CameraOrthSize,
            RecordCameraFov,
            RecordCameraOrthSize,
            LayerMask,
            CameraUp,
            CameraTarget,
            TargetCameraParam,
            CameraParam,
        ), 
        ()>,
        commands2: &mut Alter<
        (),
        (), 
        (
            ViewerAspect,
            ViewerViewMatrix,
            ViewerProjectionMatrix,
            ViewerTransformMatrix,
            ViewerGlobalPosition,
            ViewerDirection,
            ModelList,
            FlagModelList,
            ModelListAfterCulling,
            ViewerActive,
            ViewerRenderersInfo,
            DirtyViewerRenderersInfo,
            ForceIncludeModelList,
            FlagForceIncludeModelList
        ),
        ()>,
        scene: Entity,
    ) {
        ActionTransformNode::init(entity, alter1, alter2, alter3, alter4, scene);
        ActionCamera::as_camera(entity, commands1, commands2);
    }
    pub(crate) fn as_camera(
        entity: Entity,
        commands: &mut Alter<
        (),
        (),
        (
            Camera,
            ViewerDistanceCompute,
            CameraFov,
            CameraOrthSize,
            RecordCameraFov,
            RecordCameraOrthSize,
            LayerMask,
            CameraUp,
            CameraTarget,
            TargetCameraParam,
            CameraParam,
        ), 
        ()>,
        commands2: &mut Alter<
        (),
        (), 
        (
            ViewerAspect,
            ViewerViewMatrix,
            ViewerProjectionMatrix,
            ViewerTransformMatrix,
            ViewerGlobalPosition,
            ViewerDirection,
            ModelList,
            FlagModelList,
            ModelListAfterCulling,
            ViewerActive,
            ViewerRenderersInfo,
            DirtyViewerRenderersInfo,
            ForceIncludeModelList,
            FlagForceIncludeModelList
        ),
        ()>,
    ) {
        commands.alter(entity, (
            Camera(false),
            ViewerDistanceCompute::default(),
            CameraFov::default(),
            CameraOrthSize::default(),
            RecordCameraFov::default(),
            RecordCameraOrthSize::default(), 
            LayerMask::default(),
            CameraUp(CoordinateSytem3::up()),
            CameraTarget(Vector3::new(0., 0., 1.)),
            TargetCameraParam::default(),
            CameraParam::default()
        ));
    
        
        ActionViewer::as_viewer(entity, commands2, false);
    }
}

    pub fn sys_update_target_camera_modify(
        mut cameras: Query<(&CameraUp, &CameraTarget, &mut TargetCameraParam), (Changed<CameraUp>, Changed<CameraTarget>)>,
    ) {
        cameras.iter_mut().for_each(|(up, target, mut param)| {
            *param = TargetCameraParam::create(up.0.clone(), target.0.clone());
        });
    }


