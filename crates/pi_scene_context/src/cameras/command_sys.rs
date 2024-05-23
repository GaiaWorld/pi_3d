
use pi_scene_shell::prelude::{pi_world::editor::EntityEditor, *};
use pi_scene_math::{Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    flags::{CullingFlag, Enable, GlobalEnable, RecordEnable}, layer_mask::prelude::*, prelude::{DirtyViewerRenderersInfo, SceneID, SceneMainCameraID, ViewerRenderersInfo}, transforms::command_sys::ActionTransformNode, viewer::{command_sys::ActionViewer, prelude::*}
};

use super::{
    camera::*, command::*, target_camera::*, AbsoluteTransform, GlobalMatrix, LocalEulerAngles, LocalMatrix, LocalPosition, LocalRotation, LocalRotationQuaternion, LocalScaling, RecordLocalEulerAngles, RecordLocalPosition, RecordLocalRotationQuaternion, RecordLocalScaling, TransformNodeDirty
};

pub fn sys_create_camera(
    mut cmds: ResMut<ActionListCameraCreate>,
    mut editor: EntityEditor,
    mut dynallocator: ResMut<ResBindBufferAllocator>,
    mut errors: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|OpsCameraCreation(scene, entity)| {
        if editor.contains_entity(entity) {

            ActionCamera::init(entity, &mut editor,  scene);

            if let Some(bindviewer) = BindViewer::new(&mut dynallocator) {
                let index = editor.init_component::<BindViewer>();
                editor.add_components(entity, &[index]);
                *editor.get_component_unchecked_mut_by_id(entity, index) = bindviewer;
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
        editor: &mut EntityEditor,
        scene: Entity,
    ) {
        ActionTransformNode::init(entity, editor, scene);
        ActionCamera::as_camera(entity, editor);
    }
    pub(crate) fn as_camera(
        entity: Entity,
        editor: &mut EntityEditor,
    ) {
        let components = [
            editor.init_component::<Camera>(),
            editor.init_component::<ViewerDistanceCompute>(),
            editor.init_component::<CameraFov>(),
            editor.init_component::<CameraOrthSize>(),
            editor.init_component::<RecordCameraFov>(),
            editor.init_component::<RecordCameraOrthSize>(),
            editor.init_component::<LayerMask>(),
            editor.init_component::<CameraUp>(),
            editor.init_component::<CameraTarget>(),
            editor.init_component::<TargetCameraParam>(),
            editor.init_component::<CameraParam>(),
        ];
        editor.add_components(entity, &components);


        *editor.get_component_unchecked_mut_by_id(entity, components[0]) =   Camera(false);
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) =    ViewerDistanceCompute::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) =    CameraFov::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) =    CameraOrthSize::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) =    RecordCameraFov::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[5]) =    RecordCameraOrthSize::default(); 
        *editor.get_component_unchecked_mut_by_id(entity, components[6]) =    LayerMask::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[7]) =    CameraUp(CoordinateSytem3::up());
        *editor.get_component_unchecked_mut_by_id(entity, components[8]) =    CameraTarget(Vector3::new(0., 0., 1.));
        *editor.get_component_unchecked_mut_by_id(entity, components[9]) =    TargetCameraParam::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[10]) =   CameraParam::default();
   
    
        
        ActionViewer::as_viewer(entity, editor, false);
    }
}

    pub fn sys_update_target_camera_modify(
        mut cameras: Query<(&CameraUp, &CameraTarget, &mut TargetCameraParam), (Changed<CameraUp>, Changed<CameraTarget>)>,
    ) {
        cameras.iter_mut().for_each(|(up, target, mut param)| {
            *param = TargetCameraParam::create(up.0.clone(), target.0.clone());
        });
    }


