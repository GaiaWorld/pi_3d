vec3 emissiveFresnel(
    float absVN,
    vec3 emissiveColor,
    MatParam matParam
) {
    float emissiveFresnelTerm = computeFresnelTerm(absVN, matParam.fresnelParam.x, matParam.fresnelParam.y);
    emissiveColor *= matParam.fresnelLeft.rgb * (1.0 - emissiveFresnelTerm) + emissiveFresnelTerm * matParam.fresnelRight.rgb;
    return emissiveColor;
}
