
vec4 emissiveTexture(
    vec2 vEmissiveUV,
    vec2 vUVOS
) {
    return texture(sampler2D(_EmissiveTex, sampler_EmissiveTex), vEmissiveUV * uEmissiveTilloff.xy + uEmissiveTilloff.zw + vUVOS);
}

vec3 emissiveColor() {
    return uEmissiveInfo.xyz;
}

float emissiveStrength() {
    return _EmissiveTexLevel;
}
