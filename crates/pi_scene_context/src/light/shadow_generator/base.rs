
use pi_engine_shell::prelude::*;


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

#[derive(Deref, Component)]
pub struct ShadowAtlasSize(pub u32);
impl ShadowAtlasSize {
    pub const DEFAULT: u32 = 1024;
}
impl Default for ShadowAtlasSize {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

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
        Self(1000.)
    }
}

#[derive(Deref, Component)]
pub struct ShadowEnable(pub bool);

#[derive(Deref, Component)]
pub struct ShadowGeneratorID(pub ObjectID);

pub const KEY_SHADOW_DEPTH_BIAS: &'static str = "uShadowDepthBias";
pub const KEY_SHADOW_NORMAL_BIAS: &'static str = "uShadowNormalBias";
pub const KEY_SHADOW_DEPTH_SCALE: &'static str = "uShadowDepthScale";
pub const KEY_SHADOW_MINZ: &'static str = "uShadowMinZ";
pub const KEY_SHADOW_MAXZ: &'static str = "uShadowMaxZ";