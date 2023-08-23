
use pi_engine_shell::prelude::*;

pub use super::{
    target_camera::*,
    camera::*,
    command::*,
    animation::*,
};


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
    pub render: ResMut<'w, ActionListCameraRenderer>,
    pub aspect: ResMut<'w, ActionListCameraAspect>,
    pub pixelsize: ResMut<'w, ActionListCameraPixelSize>,
}
