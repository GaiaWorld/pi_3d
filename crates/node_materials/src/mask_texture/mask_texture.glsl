vec4 maskTexture(
    vec2 vUV,
    vec2 vUVOS,
    vec4 atlas
) {
    return texture(sampler2D(_MaskTex, sampler_MaskTex), uvAtlas(vUV * uMaskTilloff.xy + uMaskTilloff.zw + vUVOS, atlas));
}
vec4 maskTexture( vec2 vUV, vec2 vUVOS ) {
    return texture(sampler2D(_MaskTex, sampler_MaskTex), vUV * uMaskTilloff.xy + uMaskTilloff.zw + vUVOS);
}

