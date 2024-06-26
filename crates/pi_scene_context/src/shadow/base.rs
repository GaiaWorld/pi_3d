
use pi_scene_shell::prelude::*;

use crate::{viewer::prelude::TCullingPerformance, layer_mask::prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageShadowGenerator {
    Create,
    _Create,
    Command,
    Renderer,
    CalcMatrix,
    ViewerUpdate,
    BindUpdate,
    Culling,
}

#[derive(Deref, Component, Default)]
pub struct ShadowLayerMask(pub LayerMask);
impl TViewerLayerMask for ShadowLayerMask {
    fn include(&self, other: u32) -> bool {
        return self.0.0 & other > 0;
    }
}


#[derive(Deref, Component, Default)]
pub struct ShadowCastPassTag(pub PassTag);

#[derive(Deref, Component)]
pub struct ShadowLinkedLightID(pub Entity);
impl Default for ShadowLinkedLightID {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}

#[derive(Deref, Component, Default)]
pub struct LightLinkedShadowID(pub Option<Entity>);


#[derive(Deref, Component, Default)]
pub struct ShadowIndex(pub u32);

#[derive(Component)]
pub struct ShadowParam {
    pub minz: f32,
    pub maxz: f32,
    pub frustum: f32,
    pub bias: f32,
    pub normalbias: f32,
    pub depthscale: f32,
}
impl Default for ShadowParam {
    fn default() -> Self {
        Self { minz: 0., maxz: 20., frustum: 10., bias: 0.001, normalbias: 0.001, depthscale: 1.0 }
    }
}

#[derive(Deref, Component)]
pub struct ShadowGeneratorID(pub Entity);
impl Default for ShadowGeneratorID {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}
#[derive(Component)]
pub struct ShadowAngle(pub f32);
impl Default for ShadowAngle {
    fn default() -> Self {
        Self(3.1415926 / 2.)
    }
}


pub const KEY_SHADOW_DEPTH_BIAS: &'static str = "uShadowDepthBias";
pub const KEY_SHADOW_NORMAL_BIAS: &'static str = "uShadowNormalBias";
pub const KEY_SHADOW_DEPTH_SCALE: &'static str = "uShadowDepthScale";
pub const KEY_SHADOW_MINZ: &'static str = "uShadowMinZ";
pub const KEY_SHADOW_MAXZ: &'static str = "uShadowMaxZ";

#[derive(Resource, Default)]
pub struct StateShadow {
    pub culling_time: u32,
}
impl TCullingPerformance for StateShadow {
    fn culling_time(&mut self, ms: u32) {
        self.culling_time = ms;
    }
}
