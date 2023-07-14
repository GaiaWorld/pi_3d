
use pi_engine_shell::prelude::*;

use super::{transform_node::*};

pub type PluginAnimeLocalPosition   = PluginTypeAnime<LocalPosition, RecordLocalPosition>;
pub type PluginAnimeLocalEuler      = PluginTypeAnime<LocalEulerAngles, RecordLocalEulerAngles>;
pub type PluginAnimeLocalQuaternion = PluginTypeAnime<LocalRotationQuaternion, RecordLocalRotationQuaternion>;
pub type PluginAnimeLocalScaling    = PluginTypeAnime<LocalScaling, RecordLocalScaling>;