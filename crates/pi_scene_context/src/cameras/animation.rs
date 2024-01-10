
use pi_scene_shell::prelude::*;

use super::camera::*;

pub type PluginAnimeCameraFOV       = PluginTypeAnime<CameraFov, RecordCameraFov>;
pub type PluginAnimeCameraSize      = PluginTypeAnime<CameraOrthSize, RecordCameraOrthSize>;