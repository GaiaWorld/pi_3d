
use crate::ecs::*;

use derive_deref::Deref;

use pi_render::renderer::sampler::*;

use crate::assets::texture::TEXTURE_SLOT_COUNT;

#[derive(Clone, Deref, Hash, PartialEq, Eq, Component, Default)]
pub struct EffectBindSampler2DList(pub [Option<BindDataSampler>; TEXTURE_SLOT_COUNT]);
