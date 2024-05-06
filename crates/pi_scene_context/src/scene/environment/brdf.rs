use std::sync::Arc;

use pi_scene_shell::prelude::*;

#[derive(Clone, Hash, )]
pub struct BRDFTextureSlot(pub EKeyTexture);
impl BRDFTextureSlot {
    pub fn key(&self) -> &EKeyTexture {
        &self.0
    }
}
impl std::ops::Deref for BRDFTextureSlot {
    type Target = EKeyTexture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, )]
pub struct BRDFTexture(pub Option<Arc<ShaderBindBRDFTexture>>);
impl From<ETextureViewUsage> for BRDFTexture {
    fn from(value: ETextureViewUsage) -> Self { Self( Some(Arc::new(ShaderBindBRDFTexture(BindDataTexture2D(value)))) ) }
}
impl From<Handle<ImageTextureView>> for BRDFTexture {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some(Arc::new(ShaderBindBRDFTexture(BindDataTexture2D(ETextureViewUsage::Image(value))))) ) }
}

// 
pub struct BRDFSampler(pub Option<Arc<ShaderBindBRDFSampler>>);
impl BRDFSampler {
    pub fn new(device: &RenderDevice, asset: &Share<AssetMgr<SamplerRes>>) -> Self {
        let desc = SamplerDesc::linear_clamp();
        if let Some(sampler) = BindDataSampler::create(desc, device, asset) {
            Self(Some(Arc::new(ShaderBindBRDFSampler(sampler))))
        } else {
            Self(None)
        }
    }
}
