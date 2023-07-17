
use pi_engine_shell::prelude::*;

use super::{camera::*, AssetCapacityAnimeCamera};

pub type PluginAnimeCameraFOV       = PluginTypeAnime<CameraFov, RecordCameraFov>;
pub type PluginAnimeCameraSize      = PluginTypeAnime<CameraOrthSize, RecordCameraOrthSize>;