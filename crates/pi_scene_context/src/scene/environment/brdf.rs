use pi_scene_shell::prelude::*;

#[derive(Clone, Hash, Component, Default)]
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

#[derive(Clone, Deref, Hash, PartialEq, Eq, Component, Default)]
pub struct BRDFTexture(pub Option<ShaderBindBRDFTexture>);
impl From<ETextureViewUsage> for BRDFTexture {
    fn from(value: ETextureViewUsage) -> Self { Self( Some(ShaderBindBRDFTexture(BindDataTexture2D(value))) ) }
}
impl From<Handle<ImageTextureView>> for BRDFTexture {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some(ShaderBindBRDFTexture(BindDataTexture2D(ETextureViewUsage::Image(value)))) ) }
}

#[derive(Component, Default)]
pub struct BRDFSampler(pub Option<ShaderBindBRDFSampler>);
impl BRDFSampler {
    pub fn new(device: &RenderDevice, asset: &ShareAssetMgr<SamplerRes>) -> Self {
        let desc = SamplerDesc::linear_clamp();
        Self(ShaderBindBRDFSampler::new(desc, device, asset))
    }
}
