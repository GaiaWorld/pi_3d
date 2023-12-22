vec4 mainTexture(
    vec2 vMainUV,
    vec2 vUVOS
) {
    return texture(sampler2D(_MainTex, sampler_MainTex), vMainUV * uMainTilloff.xy + uMainTilloff.zw + vUVOS);
}

vec3 mainColor() {
    return uMainInfo.xyz;
}

float mainStrength() {
    return _MainTexLevel;
}
