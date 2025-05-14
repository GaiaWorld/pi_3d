
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    layer_mask::prelude::*, prelude::SceneID, transforms::command_sys::{ActionTransformNode, TransformNodeBundle}, viewer::{command_sys::ActionViewer, prelude::*}
};

use super::{
    target_camera::*,
    camera::*,
    command::*,
};

pub fn sys_create_camera(
    mut cmds: ResMut<ActionListCameraCreate>,
    mut commands: Commands,
    // mut dynallocator: ResMut<ResBindBufferAllocator>,
    mut dynallocator: ResMut<ResBindBufferAllocatorStatic>,
    mut errors: ResMut<ErrorRecord>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_create_camera"));
    cmds.drain().for_each(|OpsCameraCreation(scene, entity)| {
        if let Some(mut commands) = commands.get_entity(entity) {

            let bindviewer = BindViewer::new(&mut dynallocator);
            let bundle = (bindviewer, ActionCamera::init(scene));
            commands.insert(bundle);
        }
    })
}


pub fn sys_act_camera_mode(
    mut cmds: ResMut<ActionListCameraModify>,
    mut active_cameras: Query<(&SceneID, &mut Camera, &mut ViewerActive)>,
    mut cameras: Query<(&mut CameraParam, &mut ViewerDistanceCompute)>,
    mut fov_cameras: Query<&mut CameraFov>,
    mut orth_cameras: Query<&mut CameraOrthSize>,
    mut aspect_cameras: Query<&mut ViewerAspect>,
    mut target_cameras: Query<&mut CameraTarget>,
    mut target_cmds: ResMut<ActionListCameraTarget>,
    mut recordfovs: ResMut<AnimeTargetRecordValues<CameraFov>>,
    mut recordorths: ResMut<AnimeTargetRecordValues<CameraOrthSize>>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_camera_mode"));
    cmds.drain().for_each(|OpsCameraModify(entity, mode)| {
        match mode {
            ECameraModify::FreeMode(val) => if let Ok((mut camera, mut distance)) = cameras.get_mut(entity) {
                if camera.mode != val {
                    camera.mode = val;
                }
                match val {
                    EFreeCameraMode::Perspective => *distance = ViewerDistanceCompute::new(EViewerDistanceCompute::Base),
                    EFreeCameraMode::Orthograhic => *distance = ViewerDistanceCompute::new(EViewerDistanceCompute::Direction),
                }
            },
            ECameraModify::Active(val) => if let Ok((idscene, mut camera, mut viewer)) = active_cameras.get_mut(entity) {
                // log::warn!("CameraActive {:?}, New {:?}", viewer, mode);
                if camera.0 != val {
                    *camera = Camera(val);
                    *viewer = ViewerActive(val);
                    // log::warn!("CameraActive Ok");
                }
            },
            ECameraModify::FixMode(val) => if let Ok((mut camera, _)) = cameras.get_mut(entity) {
                if camera.fixed_mode != val {
                    camera.fixed_mode = val;
                }
            },
            ECameraModify::Fov(val) => if let Ok(mut camera) = fov_cameras.get_mut(entity) {
                recordfovs.insert(entity, CameraFov(val));
                *camera = CameraFov(val);
            },
            ECameraModify::OrthSize(val) => if let Ok(mut camera) = orth_cameras.get_mut(entity) {
                recordorths.insert(entity, CameraOrthSize(val));
                *camera = CameraOrthSize(val);
            },
            ECameraModify::Aspect(val) => if let Ok(mut camera) = aspect_cameras.get_mut(entity) {
                camera.0 = val;
            },
            ECameraModify::NearFar(near, far) => if let Ok((mut camera, _)) = cameras.get_mut(entity) {
                camera.nearfar = CameraNearFar(near, far);
            },
        }
    });
    target_cmds.drain().for_each(|OpsCameraTarget(entity, target)| {
        if let Ok(mut camera) = target_cameras.get_mut(entity) {
            *camera = CameraTarget(target);
        }
    });
}

pub type CameraBaseBundle = (
    Camera,
    ViewerDistanceCompute,
    CameraFov,
    CameraOrthSize,
    LayerMask,
    CameraUp,
    CameraTarget,
    TargetCameraParam,
    CameraParam,
);

pub type CameraBundle = (
    TransformNodeBundle,
    CameraBaseBundle,
    ViewerBundle
);

pub struct ActionCamera;
impl ActionCamera {
    pub fn init(
        scene: Entity,
    ) -> CameraBundle {
        (
            ActionTransformNode::init(scene),
            ActionCamera::as_camera(),
            ActionViewer::as_viewer(false),
        )
    }
    pub(crate) fn as_camera() -> CameraBaseBundle {
        (
            Camera(false),
            ViewerDistanceCompute::default(),
            CameraFov::default(),
            CameraOrthSize::default(),
            LayerMask::default(),
            CameraUp(CoordinateSytem3::up()),
            CameraTarget(Vector3::new(0., 0., 1.)),
            TargetCameraParam::default(),
            CameraParam::default(),
        )
    }
}

    pub fn sys_update_target_camera_modify(
        mut cameras: Query<(&CameraUp, &CameraTarget, &mut TargetCameraParam), Or<(Changed<CameraUp>, Changed<CameraTarget>)>>,

        // mut performance: ResMut<Performance>,
    ) {
        // performance.systems.push(String::from("sys_update_target_camera_modify"));
        cameras.iter_mut().for_each(|(up, target, mut param)| {
            *param = TargetCameraParam::create(up.0.clone(), target.0.clone());
        });
    }


