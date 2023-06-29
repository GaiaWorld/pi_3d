
use pi_engine_shell::prelude::*;

use super::{transform_node::{LocalPosition, LocalEulerAngles, LocalRotationQuaternion, LocalScaling}, AssetCapacityAnimeTransformNode};

pub type PluginAnimeLocalPosition   = PluginTypeAnime<LocalPosition, AssetCapacityAnimeTransformNode>;
pub type PluginAnimeLocalEuler      = PluginTypeAnime<LocalEulerAngles, AssetCapacityAnimeTransformNode>;
pub type PluginAnimeLocalQuaternion = PluginTypeAnime<LocalRotationQuaternion, AssetCapacityAnimeTransformNode>;
pub type PluginAnimeLocalScaling    = PluginTypeAnime<LocalScaling, AssetCapacityAnimeTransformNode>;