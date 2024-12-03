use std::sync::Arc;
use pi_scene_shell::prelude::*;

#[derive(Component, Default)]
pub struct MainCameraOpaqueTarget(pub Option<CustomRenderTarget>);
impl MainCameraOpaqueTarget {
    pub fn binds(&self) -> Option<(ShaderBindMainCameraOpaqueTexture, ShaderBindMainCameraOpaqueSampler)> {
        if let Some(target) = &self.0 {
            let tex = ETextureViewUsage::SRT(target.rt.clone());
            Some((
                ShaderBindMainCameraOpaqueTexture(BindDataTexture2D(tex)),
                ShaderBindMainCameraOpaqueSampler(target.sampler.clone())
            ))
        } else {
            None
        }
    }
}

#[derive(Component, Default)]
pub struct MainCameraDepthTarget(pub Option<CustomRenderTarget>);
impl MainCameraDepthTarget {
    pub fn binds(&self) -> Option<(ShaderBindMainCameraDepthTexture, ShaderBindMainCameraDepthSampler)> {
        if let Some(target) = &self.0 {
            let tex = ETextureViewUsage::SRT(target.rt.clone());
            Some((
                ShaderBindMainCameraDepthTexture(BindDataTexture2D(tex)),
                ShaderBindMainCameraDepthSampler(target.sampler.clone())
            ))
        } else {
            None
        }
    }
}