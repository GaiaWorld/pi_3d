vec4 mixTexture(
    vec2 vUV,
    vec2 vUVOS,
    vec4 atlas
) {
    return texture(sampler2D(_MixTex, sampler_MixTex), uvAtlas(vUV * uMixTilloff.xy + uMixTilloff.zw + vUVOS, atlas));
}

vec4 mixTexture( vec2 vUV, vec2 vUVOS ) {
    return texture(sampler2D(_MixTex, sampler_MixTex), vUV * uMixTilloff.xy + uMixTilloff.zw + vUVOS);
}
