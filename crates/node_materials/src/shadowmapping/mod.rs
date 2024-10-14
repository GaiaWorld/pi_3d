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
        vec4 P,
        const float NdotL,
        out float shadow,
    ) {
        shadow = 1.;
        uint idshadow       = _ShadowLightIdxs[idxlight].x;
        
        mat4 depthVP        = _ShadowMapMatrix[idshadow];
        vec4 biasAndScale   = _BiasAndScaleSM[idshadow];
        vec4 depthTilloff   = _ShadowMapTilloff[idshadow];

        vec4 ShadowCoord    = depthVP * P;
        float depthMetric   = (ShadowCoord.z + biasAndScale.z) / biasAndScale.w;
        depthMetric         = clamp(depthMetric, 0., 1.);

        vec2 clipSpace = ShadowCoord.xy/ShadowCoord.w;
        vec2 uv = 0.5*clipSpace+vec2(0.5);
        uv.y = 1.0 - uv.y;
        float inAtlas = step(uv.x, 1.) * step(uv.y, 1.) * step(0., uv.x) * step(0., uv.y);

        // vec2 pixels = textureSize(sampler2D(_ShadowMap, sampler_ShadowMap), 0);
        // uv = uv * pixels + 0.5;
        // vec2 iuv = floor(uv);
        // vec2 fuv = fract(uv);
        // uv = iuv + fuv*fuv*(3.0 - 2.0 * fuv);
        // uv = (uv - 0.5) / pixels;
        float depthMap = texture(sampler2D(_ShadowMap, sampler_ShadowMap), uv).r;
        shadow =  mix(1., step(depthMetric, depthMap), inAtlas);

        // float depthMap = texture(sampler2D(_ShadowMap, sampler_ShadowMap), (ShadowCoord.xy * 0.5 + 0.5)).r;
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
            let nodematblocks = app.world.get_resource_mut::<NodeMaterialBlocks>().unwrap();
            nodematblocks.regist::<BlockShadowMapping>();
        // }

        // app.add_systems(Startup, setup);
    }
}