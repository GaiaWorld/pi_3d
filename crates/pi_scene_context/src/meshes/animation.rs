use pi_engine_shell::prelude::*;

use crate::geometry::instance::instance_boneoffset::*;
use super::model::{IndiceRenderRange, RecordIndiceRenderRange};


pub type PluginAnimeBoneOffset          = PluginTypeAnime<InstanceBoneoffset, RecordInstanceBoneoffset>;
pub type PluginAnimeRenderIndiceRange   = PluginTypeAnime<IndiceRenderRange, RecordIndiceRenderRange>;