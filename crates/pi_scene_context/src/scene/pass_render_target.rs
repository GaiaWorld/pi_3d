use std::sync::Arc;
use pi_scene_shell::prelude::*;

#[derive(Component, Default)]
pub struct MainCameraOpaqueTarget(pub Option<CustomRenderTarget>);
impl MainCameraOpaqueTarget {
    pub fn binds(&self) -> Option<(Arc<ShaderBindMainCameraOpaqueTexture>, Arc<ShaderBindMainCameraOpaqueSampler>)> {
        if let Some(target) = &self.0 {
            let tex = ETextureViewUsage::SRT(target.rt.clone());
            Some((
                Arc::new(ShaderBindMainCameraOpaqueTexture(BindDataTexture2D(tex))),
                Arc::new(ShaderBindMainCameraOpaqueSampler(target.sampler.clone()))
            ))
        } else {
            None
        }
    }
}

#[derive(Component, Default)]
pub struct MainCameraDepthTarget(pub Option<CustomRenderTarget>);
impl MainCameraDepthTarget {
    pub fn binds(&self) -> Option<(Arc<ShaderBindMainCameraDepthTexture>, Arc<ShaderBindMainCameraDepthSampler>)> {
        if let Some(target) = &self.0 {
            let tex = ETextureViewUsage::SRT(target.rt.clone());
            Some((
                Arc::new(ShaderBindMainCameraDepthTexture(BindDataTexture2D(tex))),
                Arc::new(ShaderBindMainCameraDepthSampler(target.sampler.clone()))
            ))
        } else {
            None
        }
    }
}