use derive_deref::Deref;
use pi_engine_shell::object::ObjectID;


#[derive(Deref)]
pub struct ShadowMinZ(pub f32);
impl Default for ShadowMinZ {
    fn default() -> Self {
        Self(0.)
    }
}

#[derive(Deref)]
pub struct ShadowMaxZ(pub f32);
impl Default for ShadowMaxZ {
    fn default() -> Self {
        Self(20.)
    }
}

#[derive(Deref)]
pub struct ShadowFrustumSize(pub f32);
impl Default for ShadowFrustumSize {
    fn default() -> Self {
        Self(10.)
    }
}

#[derive(Deref)]
pub struct ShadowAtlasSize(pub u32);
impl Default for ShadowAtlasSize {
    fn default() -> Self {
        Self(1024)
    }
}

#[derive(Deref)]
pub struct ShadowBias(pub f32);
impl Default for ShadowBias {
    fn default() -> Self {
        Self(0.001)
    }
}

#[derive(Deref)]
pub struct ShadowNormalBias(pub f32);
impl Default for ShadowNormalBias {
    fn default() -> Self {
        Self(0.001)
    }
}

#[derive(Deref)]
pub struct ShadowDepthScale(pub f32);
impl Default for ShadowDepthScale {
    fn default() -> Self {
        Self(1000.)
    }
}

#[derive(Deref)]
pub struct ShadowEnable(pub bool);

#[derive(Deref)]
pub struct ShadowGeneratorID(pub ObjectID);
