
use pi_scene_shell::prelude::{pi_world::editor::EntityEditor, *};
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
        // *editor.get_component_unchecked_mut_by_id(entity, components[1]) =    ViewerDistanceCompute::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[2]) =    CameraFov::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[3]) =    CameraOrthSize::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[4]) =    RecordCameraFov::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[5]) =    RecordCameraOrthSize::default(); 
        // *editor.get_component_unchecked_mut_by_id(entity, components[6]) =    LayerMask::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[7]) =    CameraUp(CoordinateSytem3::up());
        *editor.get_component_unchecked_mut_by_id(entity, components[8]) =    CameraTarget(Vector3::new(0., 0., 1.));
        // *editor.get_component_unchecked_mut_by_id(entity, components[9]) =    TargetCameraParam::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[10]) =   CameraParam::default();
   
    
        
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


