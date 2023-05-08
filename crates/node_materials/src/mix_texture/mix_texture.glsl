vec4 mixTexture(
    vec2 vUV,
    vec2 vUVOS
) {
    return texture(sampler2D(_MixTex, sampler_MixTex), vUV * uMixTilloff.xy + uMixTilloff.zw + vUVOS);
}

