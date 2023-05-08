vec3 emissiveFresnel(
    float absVN,
    vec3 emissiveColor
) {
    float emissiveFresnelTerm = computeFresnelTerm(absVN, fresnelParam.x, fresnelParam.y);
    emissiveColor *= fresnelLeft.rgb * (1.0 - emissiveFresnelTerm) + emissiveFresnelTerm * fresnelRight.rgb;
    return emissiveColor;
}
