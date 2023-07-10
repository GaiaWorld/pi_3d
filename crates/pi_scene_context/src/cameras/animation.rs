
use pi_engine_shell::prelude::*;

use super::{camera::*, AssetCapacityAnimeCamera};

pub type PluginAnimeCameraFOV       = PluginTypeAnime<CameraFov, RecordCameraFov, AssetCapacityAnimeCamera>;
pub type PluginAnimeCameraSize      = PluginTypeAnime<CameraOrthSize, RecordCameraOrthSize, AssetCapacityAnimeCamera>;