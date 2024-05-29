


use derive_deref::Deref;

use pi_render::renderer::sampler::{BindDataSampler};
use pi_world_macros::Component;

use crate::assets::texture::TEXTURE_SLOT_COUNT;

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2DList(pub [Option<BindDataSampler>; TEXTURE_SLOT_COUNT]);
