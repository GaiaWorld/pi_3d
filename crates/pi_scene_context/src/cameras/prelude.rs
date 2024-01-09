
use pi_engine_shell::prelude::*;

use crate::{viewer::prelude::{ModelListAfterCulling, ModelList, TCullingPerformance, ActionListViewerForceInclude}};

pub use super::{
    target_camera::*,
    camera::*,
    command::*,
    animation::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageCamera {
    CameraCreate,
    _CameraCreate,
    CameraCommand,
    CameraRenderer,
    CameraCalcMatrix,
    CameraCulling,
}


#[derive(Resource, Default)]
pub struct StateCamera {
    pub camera: Option<Entity>,
    pub includes: u32,
    pub culling: u32,
    pub culling_time: u32,
}
impl TCullingPerformance for StateCamera {
    fn culling_time(&mut self, ms: u32) {
        self.culling_time = ms;
    }
}

#[derive(SystemParam)]
pub struct ActionSetCamera<'w> {
    pub create: ResMut<'w, ActionListCameraCreate>,
    pub mode: ResMut<'w, ActionListCameraMode>,
    pub target: ResMut<'w, ActionListCameraTarget>,
    pub active: ResMut<'w, ActionListCameraActive>,
    pub fixmode: ResMut<'w, ActionListCameraFixedMode>,
    pub fov: ResMut<'w, ActionListCameraFov>,
    pub size: ResMut<'w, ActionListCameraOrthSize>,
    pub nearfar: ResMut<'w, ActionListCameraNearFar>,
    // pub render: ResMut<'w, ActionListCameraRenderer>,
    pub aspect: ResMut<'w, ActionListCameraAspect>,
    // pub pixelsize: ResMut<'w, ActionListCameraPixelSize>,
    pub forceinclude: ResMut<'w, ActionListViewerForceInclude>,
}

pub type StateCameraQuery = QueryState<(&'static Camera, &'static ModelList, &'static ModelListAfterCulling)>;

pub fn sys_state_camera(
    mut state: ResMut<StateCamera>,
    cameras: Query<(&Camera, &ModelList, &ModelListAfterCulling)>,
) {
    state.culling = 0;
    if let Some(camera) = state.camera {
        if let Ok((_camera, includes, culling)) = cameras.get(camera) {
            state.includes += includes.0.len() as u32;
            state.culling += culling.0.len() as u32;
        }
    }
}
