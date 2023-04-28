
use bevy::prelude::{Deref, Component};
use pi_assets::asset::Handle;

use pi_render::{
    renderer::{
        texture::*,
    },
    rhi::{asset::TextureRes},
};


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D01Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D01);
impl From<Handle<TextureRes>> for EffectBindTexture2D01Comp {
    fn from(value: Handle<TextureRes>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D01(BindDataTexture2D(ETextureViewUsage::Tex(value))) ) }
}
impl EffectBindTexture2D01Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D02Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D02);
impl From<Handle<TextureRes>> for EffectBindTexture2D02Comp {
    fn from(value: Handle<TextureRes>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D02(BindDataTexture2D(ETextureViewUsage::Tex(value))) ) }
}
impl EffectBindTexture2D02Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D03Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D03);
impl From<Handle<TextureRes>> for EffectBindTexture2D03Comp {
    fn from(value: Handle<TextureRes>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D03(BindDataTexture2D(ETextureViewUsage::Tex(value))) ) }
}
impl EffectBindTexture2D03Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D04Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D04);
impl From<Handle<TextureRes>> for EffectBindTexture2D04Comp {
    fn from(value: Handle<TextureRes>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D04(BindDataTexture2D(ETextureViewUsage::Tex(value))) ) }
}
impl EffectBindTexture2D04Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D05Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D05);
impl From<Handle<TextureRes>> for EffectBindTexture2D05Comp {
    fn from(value: Handle<TextureRes>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D05(BindDataTexture2D(ETextureViewUsage::Tex(value))) ) }
}
impl EffectBindTexture2D05Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}


#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D06Comp(pub pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D06);
impl From<Handle<TextureRes>> for EffectBindTexture2D06Comp {
    fn from(value: Handle<TextureRes>) -> Self { Self( pi_render::render_3d::binds::effect_texture2d::EffectBindTexture2D06(BindDataTexture2D(ETextureViewUsage::Tex(value)) )) }
}
impl EffectBindTexture2D06Comp {
    pub fn data(&self) -> &BindDataTexture2D { &self.0 }
}