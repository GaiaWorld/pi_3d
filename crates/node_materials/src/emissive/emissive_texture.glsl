
vec4 emissiveTexture(
    vec2 vUV,
    vec2 vUVOS,
    vec4 atlas
) {
    return texture(sampler2D(_EmissiveTex, sampler_EmissiveTex), uvAtlas(vUV * uEmissiveTilloff.xy + uEmissiveTilloff.zw + vUVOS, atlas));
}
vec4 emissiveTexture( vec2 vUV, vec2 vUVOS ) {
    return texture(sampler2D(_EmissiveTex, sampler_EmissiveTex), vUV * uEmissiveTilloff.xy + uEmissiveTilloff.zw + vUVOS );
}

vec3 emissiveColor() {
    return uEmissiveInfo.xyz;
}

float emissiveStrength() {
    return _EmissiveTexLevel;
}
