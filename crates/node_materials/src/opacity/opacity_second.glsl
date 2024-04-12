vec4 opacity2Texture(
    vec2 vUV,
    vec2 vUVOS,
    vec4 atlas
) {
    return texture(sampler2D(_Opacity2Tex, sampler_Opacity2Tex), uvAtlas(vUV * uOpacity2Tilloff.xy + uOpacity2Tilloff.zw + vUVOS, atlas));
}
vec4 opacity2Texture( vec2 vUV, vec2 vUVOS ) {
    return texture(sampler2D(_Opacity2Tex, sampler_Opacity2Tex), vUV * uOpacity2Tilloff.xy + uOpacity2Tilloff.zw + vUVOS);
}

float opacity2Channel(vec4 data) {
    return valueByChannel(data, uOpacity2Channel) * uOpacity2Level;
}
