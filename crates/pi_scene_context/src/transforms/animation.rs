
use pi_engine_shell::prelude::*;

use super::{transform_node::*, AssetCapacityAnimeTransformNode};

pub type PluginAnimeLocalPosition   = PluginTypeAnime<LocalPosition, RecordLocalPosition, AssetCapacityAnimeTransformNode>;
pub type PluginAnimeLocalEuler      = PluginTypeAnime<LocalEulerAngles, RecordLocalEulerAngles, AssetCapacityAnimeTransformNode>;
pub type PluginAnimeLocalQuaternion = PluginTypeAnime<LocalRotationQuaternion, RecordLocalRotationQuaternion, AssetCapacityAnimeTransformNode>;
pub type PluginAnimeLocalScaling    = PluginTypeAnime<LocalScaling, RecordLocalScaling, AssetCapacityAnimeTransformNode>;