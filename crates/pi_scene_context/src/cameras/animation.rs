
use pi_engine_shell::prelude::*;

use super::{camera::*, AssetCapacityAnimeCamera};

pub type PluginAnimeCameraFOV       = PluginTypeAnime<CameraFov, AssetCapacityAnimeCamera>;
pub type PluginAnimeCameraSize      = PluginTypeAnime<CameraOrthSize, AssetCapacityAnimeCamera>;