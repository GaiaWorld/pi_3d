

use bevy::prelude::{Deref, Component};
use pi_assets::asset::Handle;

use pi_render::renderer::sampler::{BindDataSampler, SamplerRes};
use crate::binds::*;


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2D01Comp(pub Option<BindDataSampler>);
impl From<Handle<SamplerRes>> for EffectBindSampler2D01Comp {
    fn from(value: Handle<SamplerRes>) -> Self { Self( Some(BindDataSampler(value)) ) }
}
impl EffectBindSampler2D01Comp {
    // pub fn data(&self) -> &BindDataSampler { &self.0 }
}

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2D02Comp(pub Option<BindDataSampler>);
impl From<Handle<SamplerRes>> for EffectBindSampler2D02Comp {
    fn from(value: Handle<SamplerRes>) -> Self { Self( Some(BindDataSampler(value)) ) }
}
impl EffectBindSampler2D02Comp {
    // pub fn data(&self) -> &BindDataSampler { &self.0 }
}


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2D03Comp(pub Option<BindDataSampler>);
impl From<Handle<SamplerRes>> for EffectBindSampler2D03Comp {
    fn from(value: Handle<SamplerRes>) -> Self { Self( Some(BindDataSampler(value)) ) }
}
impl EffectBindSampler2D03Comp {
    // pub fn data(&self) -> &BindDataSampler { &self.0 }
}


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2D04Comp(pub Option<BindDataSampler>);
impl From<Handle<SamplerRes>> for EffectBindSampler2D04Comp {
    fn from(value: Handle<SamplerRes>) -> Self { Self( Some(BindDataSampler(value)) ) }
}
impl EffectBindSampler2D04Comp {
    // pub fn data(&self) -> &BindDataSampler { &self.0 }
}


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2D05Comp(pub Option<BindDataSampler>);
impl From<Handle<SamplerRes>> for EffectBindSampler2D05Comp {
    fn from(value: Handle<SamplerRes>) -> Self { Self( Some(BindDataSampler(value)) ) }
}
impl EffectBindSampler2D05Comp {
    // pub fn data(&self) -> &BindDataSampler { &self.0 }
}


#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2D06Comp(pub Option<BindDataSampler>);
impl From<Handle<SamplerRes>> for EffectBindSampler2D06Comp {
    fn from(value: Handle<SamplerRes>) -> Self { Self( Some(BindDataSampler(value)) ) }
}
impl EffectBindSampler2D06Comp {
    // pub fn data(&self) -> &BindDataSampler { &self.0 }
}

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2D07Comp(pub Option<BindDataSampler>);
impl From<Handle<SamplerRes>> for EffectBindSampler2D07Comp {
    fn from(value: Handle<SamplerRes>) -> Self { Self( Some(BindDataSampler(value)) ) }
}
impl EffectBindSampler2D07Comp {
    // pub fn data(&self) -> &BindDataSampler { &self.0 }
}

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EffectBindSampler2D08Comp(pub Option<BindDataSampler>);
impl From<Handle<SamplerRes>> for EffectBindSampler2D08Comp {
    fn from(value: Handle<SamplerRes>) -> Self { Self( Some(BindDataSampler(value)) ) }
}
impl EffectBindSampler2D08Comp {
    // pub fn data(&self) -> &BindDataSampler { &self.0 }
}