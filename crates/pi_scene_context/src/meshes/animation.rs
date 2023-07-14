use pi_engine_shell::prelude::{PluginTypeAnime, RenderIndices};

use crate::{geometry::instance::instance_boneoffset::{InstanceBoneoffset, RecordInstanceBoneoffset}};
use super::model::{IndiceRenderRange, RecordIndiceRenderRange};


pub type PluginAnimeBoneOffset          = PluginTypeAnime<InstanceBoneoffset, RecordInstanceBoneoffset>;
pub type PluginAnimeRenderIndiceRange   = PluginTypeAnime<IndiceRenderRange, RecordIndiceRenderRange>;