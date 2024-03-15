use std::sync::Arc;

use bevy_ecs::prelude::Component;
use pi_scene_shell::prelude::*;

#[derive(Component)]
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

#[derive(Component)]
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

// impl ITexture for OpaqueRenderTarget {
//     fn format(&self) -> wgpu::TextureFormat {
//         if let Some(rt) = &self.rt {
//             rt.target().colors[0].1.format()
//         } else {
//             self.default.format
//         }
//     }
//     fn key(&self) -> KeyTextureViewUsage {
//         self.data.key()
//     }
//     fn view(&self) -> &wgpu::TextureView {
//         self.data.view()
//     }
// }
