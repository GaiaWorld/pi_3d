vec4 opacityTexture(
    vec2 vOpacityUV,
    vec2 vUVOS
) {
    return texture(sampler2D(_OpacityTex, sampler_OpacityTex), vOpacityUV * uOpacityTilloff.xy + uOpacityTilloff.zw + vUVOS);
}

float opacityStrength() {
    return uOpacity;
}

float opacityChannel(vec4 data) {
    return valueByChannel(data, uOpacityChannel);
}
