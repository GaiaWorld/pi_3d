vec4 opacityTexture(
    vec2 vUV,
    vec2 vUVOS,
    vec4 atlas
) {
    return texture(sampler2D(_OpacityTex, sampler_OpacityTex), uvAtlas(vUV * uOpacityTilloff.xy + uOpacityTilloff.zw + vUVOS, atlas));
}
vec4 opacityTexture( vec2 vUV, vec2 vUVOS ) {
    return texture(sampler2D(_OpacityTex, sampler_OpacityTex), vUV * uOpacityTilloff.xy + uOpacityTilloff.zw + vUVOS);
}

float opacityChannel(vec4 data) {
    return valueByChannel(data, uOpacityChannel) * uOpacityLevel;
}
