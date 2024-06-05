use pi_scene_shell::prelude::*;
use crate::{prelude::TNodeMaterialBlock, NodeMaterialBlocks};

pub struct BlockShadowMapping;
impl TNodeMaterialBlock for BlockShadowMapping {
    const KEY: &'static str = "ShadowMapping";

    const FS_DEFINED: &'static str = "
    bool hasDirectShadow(const uint idxlight) {
        uint idshadow = _ShadowLightIdxs[idxlight].x;
        return idshadow < MAX_SHADOW;
    }
    void computeDirectShadow(
        const uint idxlight,
        const vec4 P,
        const float NdotL,
        out float shadow,
    ) {
        shadow = 1.;
        uint idshadow       = _ShadowLightIdxs[idxlight].x;
        
        mat4 depthVP        = _ShadowMapMatrix[idshadow];
        vec4 depthBias      = _BiasAndScaleSM[idshadow];
        vec4 depthTilloff   = _ShadowMapTilloff[idshadow];
        vec4 ShadowCoord    = depthVP * P;

        float bias          = depthBias.x; // clamp(depthBias.x * tan(acos(NdotL)), 0., 0.01);
        float depthScale    = depthBias.z;
    
        vec2 uv = (ShadowCoord.xy * 0.5 + 0.5);
        uv.y = 1.0 - uv.y;
        float depthMap = texture(sampler2D(_ShadowMap, sampler_ShadowMap), uv).r;
        // float depthMap = texture(sampler2D(_ShadowMap, sampler_ShadowMap), (ShadowCoord.xy * 0.5 + 0.5)).r;

        shadow = mix(1., step((ShadowCoord.z - bias) * depthScale, depthMap), step(abs(ShadowCoord.x), 1.) * step(abs(ShadowCoord.y), 1.) );
        // shadow = depthMap;
    }
    ";

    const VS_DEFINED: &'static str = "
    ";

    const BIND_DEFINES: pi_scene_shell::prelude::BindDefine = pi_scene_shell::prelude::BindDefines::SHADOWMAP;
}


fn _setup(
    mut nodematblocks: ResMut<NodeMaterialBlocks>,
) {
    nodematblocks.regist::<BlockShadowMapping>();
}

pub struct PluginShadowMapping;
impl Plugin for PluginShadowMapping {
    fn build(&self, app: &mut App) {
        
        // #[cfg(not(target_arch="wasm32"))]
        // {
            let mut nodematblocks = app.world.get_resource_mut::<NodeMaterialBlocks>().unwrap();
            nodematblocks.regist::<BlockShadowMapping>();
        // }

        // app.add_systems(Startup, setup);
    }
}