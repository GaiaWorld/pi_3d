
use pi_engine_shell::prelude::*;

use super::transform_node::{LocalPosition, LocalEulerAngles, LocalRotationQuaternion, LocalScaling};

pub type PluginAnimeLocalPosition   = PluginTypeAnime<LocalPosition>;
pub type PluginAnimeLocalEuler      = PluginTypeAnime<LocalEulerAngles>;
pub type PluginAnimeLocalQuaternion = PluginTypeAnime<LocalRotationQuaternion>;
pub type PluginAnimeLocalScaling    = PluginTypeAnime<LocalScaling>;