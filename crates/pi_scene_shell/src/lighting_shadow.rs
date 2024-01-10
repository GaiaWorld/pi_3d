use bevy::prelude::Resource;
use pi_bevy_render_plugin::constant::texture_sampler::{ColorFormat, DepthStencilFormat};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LightLimitInfo {
    pub max_direct_light_count: u32,
    pub max_point_light_count: u32,
    pub max_spot_light_count: u32,
    pub max_hemi_light_count: u32,
}

#[derive(Resource)]
pub struct ModelLightLimit(pub LightLimitInfo);

#[derive(Resource)]
pub struct SceneLightLimit(pub LightLimitInfo);

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShadowLimitInfo {
    pub max_count: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub color_format: ColorFormat,
    pub depth_stencil_format: DepthStencilFormat,
}

#[derive(Resource)]
pub struct SceneShadowLimit(pub ShadowLimitInfo);
