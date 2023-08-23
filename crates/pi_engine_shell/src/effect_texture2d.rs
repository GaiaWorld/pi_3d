
use bevy::prelude::{Deref, Component};
use pi_assets::asset::Handle;

use pi_render::renderer::texture::*;

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D01Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D01);
impl From<ETextureViewUsage> for EffectBindTexture2D01Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D01::from(value) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D01Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D01::from(value) ) }
}
impl EffectBindTexture2D01Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D02Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D02);
impl From<ETextureViewUsage> for EffectBindTexture2D02Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D02::from(value) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D02Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D02::from(value) ) }
}
impl EffectBindTexture2D02Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D03Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D03);
impl From<ETextureViewUsage> for EffectBindTexture2D03Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D03::from(value) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D03Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D03::from(value) ) }
}
impl EffectBindTexture2D03Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D04Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D04);
impl From<ETextureViewUsage> for EffectBindTexture2D04Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D04::from(value) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D04Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D04::from(value) ) }
}
impl EffectBindTexture2D04Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D05Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D05);
impl From<ETextureViewUsage> for EffectBindTexture2D05Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D05::from(value) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D05Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D05::from(value) ) }
}
impl EffectBindTexture2D05Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D06Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D06);
impl From<ETextureViewUsage> for EffectBindTexture2D06Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D06::from(value) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D06Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D06::from(value) ) }
}
impl EffectBindTexture2D06Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D07Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D07);
impl From<ETextureViewUsage> for EffectBindTexture2D07Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D07::from(value) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D07Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D07::from(value) ) }
}
impl EffectBindTexture2D07Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D08Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D08);
impl From<ETextureViewUsage> for EffectBindTexture2D08Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D08::from(value) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D08Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D08::from(value) ) }
}
impl EffectBindTexture2D08Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}