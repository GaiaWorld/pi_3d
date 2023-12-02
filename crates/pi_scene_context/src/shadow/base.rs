
use pi_engine_shell::prelude::*;

use crate::{viewer::prelude::TCullingPerformance, layer_mask::prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageShadowGenerator {
    Create,
    CreateApply,
    Command,
    CommandApply,
    Renderer,
    CalcMatrix,
    Culling,
}

#[derive(Deref, Component)]
pub struct ShadowLayerMask(pub LayerMask);
impl TViewerLayerMask for ShadowLayerMask {
    fn include(&self, other: u32) -> bool {
        return self.0.0 & other > 0;
    }
}


#[derive(Deref, Component)]
pub struct ShadowCastPassTag(pub PassTag);

#[derive(Deref, Component)]
pub struct ShadowLinkedLightID(pub Entity);

#[derive(Deref, Component)]
pub struct LightLinkedShadowID(pub Option<Entity>);


#[derive(Deref, Component)]
pub struct ShadowIndex(pub u32);

#[derive(Deref, Component)]
pub struct ShadowMinZ(pub f32);
impl Default for ShadowMinZ {
    fn default() -> Self {
        Self(0.)
    }
}

#[derive(Deref, Component)]
pub struct ShadowMaxZ(pub f32);
impl Default for ShadowMaxZ {
    fn default() -> Self {
        Self(20.)
    }
}

#[derive(Deref, Component)]
pub struct ShadowFrustumSize(pub f32);
impl Default for ShadowFrustumSize {
    fn default() -> Self {
        Self(10.)
    }
}

// #[derive(Deref, Component)]
// pub struct ShadowAtlasSize(pub u32);
// impl ShadowAtlasSize {
//     pub const DEFAULT: u32 = 1024;
// }
// impl Default for ShadowAtlasSize {
//     fn default() -> Self {
//         Self(Self::DEFAULT)
//     }
// }

#[derive(Deref, Component)]
pub struct ShadowBias(pub f32);
impl Default for ShadowBias {
    fn default() -> Self {
        Self(0.001)
    }
}

#[derive(Deref, Component)]
pub struct ShadowNormalBias(pub f32);
impl Default for ShadowNormalBias {
    fn default() -> Self {
        Self(0.001)
    }
}

#[derive(Deref, Component)]
pub struct ShadowDepthScale(pub f32);
impl Default for ShadowDepthScale {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Deref, Component)]
pub struct ShadowGeneratorID(pub ObjectID);

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
