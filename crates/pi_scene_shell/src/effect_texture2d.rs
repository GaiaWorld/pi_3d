
use bevy_ecs::prelude::Component;
use derive_deref::Deref;
use pi_assets::asset::Handle;

use pi_render::renderer::texture::*;

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D01Comp(pub Option<ETextureViewUsage>);
impl From<ETextureViewUsage> for EffectBindTexture2D01Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( Some( value ) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D01Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some( ETextureViewUsage::Image(value) ) ) }
}
impl EffectBindTexture2D01Comp {
    // pub fn data(&self) -> Option<&BindDataTexture2D> { if let Some(data) = &self.0 { Some(data) } else { None } }
}

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D02Comp(pub Option<ETextureViewUsage>);
impl From<ETextureViewUsage> for EffectBindTexture2D02Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( Some( value ) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D02Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some( ETextureViewUsage::Image(value) ) ) }
}
impl EffectBindTexture2D02Comp {
    // pub fn data(&self) -> Option<&BindDataTexture2D> { if let Some(data) = &self.0 { Some(data) } else { None } }
}


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D03Comp(pub Option<ETextureViewUsage>);
impl From<ETextureViewUsage> for EffectBindTexture2D03Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( Some( value ) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D03Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some( ETextureViewUsage::Image(value) ) ) }
}
impl EffectBindTexture2D03Comp {
    // pub fn data(&self) -> Option<&BindDataTexture2D> { if let Some(data) = &self.0 { Some(data) } else { None } }
}


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D04Comp(pub Option<ETextureViewUsage>);
impl From<ETextureViewUsage> for EffectBindTexture2D04Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( Some( value ) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D04Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some( ETextureViewUsage::Image(value) ) ) }
}
impl EffectBindTexture2D04Comp {
    // pub fn data(&self) -> Option<&BindDataTexture2D> { if let Some(data) = &self.0 { Some(data) } else { None } }
}


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D05Comp(pub Option<ETextureViewUsage>);
impl From<ETextureViewUsage> for EffectBindTexture2D05Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( Some( value ) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D05Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some( ETextureViewUsage::Image(value) ) ) }
}
impl EffectBindTexture2D05Comp {
    // pub fn data(&self) -> Option<&BindDataTexture2D> { if let Some(data) = &self.0 { Some(data) } else { None } }
}


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D06Comp(pub Option<ETextureViewUsage>);
impl From<ETextureViewUsage> for EffectBindTexture2D06Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( Some( value ) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D06Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some( ETextureViewUsage::Image(value) ) ) }
}
impl EffectBindTexture2D06Comp {
    // pub fn data(&self) -> Option<&BindDataTexture2D> { if let Some(data) = &self.0 { Some(data) } else { None } }
}

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D07Comp(pub Option<ETextureViewUsage>);
impl From<ETextureViewUsage> for EffectBindTexture2D07Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( Some( value ) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D07Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some( ETextureViewUsage::Image(value) ) ) }
}
impl EffectBindTexture2D07Comp {
    // pub fn data(&self) -> Option<&BindDataTexture2D> { if let Some(data) = &self.0 { Some(data) } else { None } }
}

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2D08Comp(pub Option<ETextureViewUsage>);
impl From<ETextureViewUsage> for EffectBindTexture2D08Comp {
    fn from(value: ETextureViewUsage) -> Self { Self( Some( value ) ) }
}
impl From<Handle<ImageTextureView>> for EffectBindTexture2D08Comp {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some( ETextureViewUsage::Image(value) ) ) }
}
impl EffectBindTexture2D08Comp {
    // pub fn data(&self) -> Option<&BindDataTexture2D> { if let Some(data) = &self.0 { Some(data) } else { None } }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, Component)]
pub struct EffectBindTexture2DComp {
    pub tex01: Option<BindDataTexture2D>,
    pub tex02: Option<BindDataTexture2D>,
    pub tex03: Option<BindDataTexture2D>,
    pub tex04: Option<BindDataTexture2D>,
    pub tex05: Option<BindDataTexture2D>,
    pub tex06: Option<BindDataTexture2D>,
    pub tex07: Option<BindDataTexture2D>,
    pub tex08: Option<BindDataTexture2D>,
}
// impl EffectBindTexture2DComp {
//     pub fn append(slot: usize, tex: BindDataTexture2D) {

//     }
// }

// #[derive(Debug, Clone, Hash, PartialEq, Eq, Component)]
// pub struct EffectBindTexture2DKey {
//     pub tex01: Option<KeyTexture>,
//     pub tex02: Option<BindDataTexture2D>,
//     pub tex03: Option<BindDataTexture2D>,
//     pub tex04: Option<BindDataTexture2D>,
//     pub tex05: Option<BindDataTexture2D>,
//     pub tex06: Option<BindDataTexture2D>,
//     pub tex07: Option<BindDataTexture2D>,
//     pub tex08: Option<BindDataTexture2D>,
// }