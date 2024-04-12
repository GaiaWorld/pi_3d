vec4 mainTexture(
    vec2 vUV,
    vec2 vUVOS,
    vec4 atlas
) {
    return texture(sampler2D(_MainTex, sampler_MainTex), uvAtlas(vUV * uMainTilloff.xy + uMainTilloff.zw + vUVOS, atlas));
}

vec4 mainTexture( vec2 vUV, vec2 vUVOS ) {
    return texture(sampler2D(_MainTex, sampler_MainTex), vUV * uMainTilloff.xy + uMainTilloff.zw + vUVOS);
}

vec3 mainColor() {
    return uMainInfo.xyz;
}

float mainStrength() {
    return _MainTexLevel;
}
