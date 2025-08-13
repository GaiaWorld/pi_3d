vec3 emissiveFresnel(
    const float absVN,
    const vec3 emissiveColor
) {
    float emissiveFresnelTerm = computeFresnelTerm(absVN, matParam.fresnelParam.x, matParam.fresnelParam.y);
    return emissiveColor * matParam.fresnelLeft.rgb * (1.0 - emissiveFresnelTerm) + emissiveFresnelTerm * matParam.fresnelRight.rgb;
}
