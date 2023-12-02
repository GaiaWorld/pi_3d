// #ifndef CUSTOM_FRESNEL
// #define CUSTOM_FRESNEL


float computeFresnelTerm(float absVN, float bias, float power) {
    float fresnelTerm = pow(bias + absVN, power);
    return clamp(fresnelTerm, 0., 1.);
}

float3 emissiveFresnel(
    float absVN,
    float3 emissiveColor,
    float3 emissiveLeftColor,
    float3 emissiveRightColor,
    float bias,
    float power
) {
    float emissiveFresnelTerm = computeFresnelTerm(absVN, bias, power);
    emissiveColor *= emissiveLeftColor.rgb * (1.0 - emissiveFresnelTerm) + emissiveFresnelTerm * emissiveRightColor.rgb;
    return emissiveColor;
}

float3 fresnelSchlickGGX(float VdotH, float3 reflectance0, float3 reflectance90) {
	float t = pow5(1.0 - VdotH);
    return reflectance0 + (reflectance90 - reflectance0) * t;
}

float3 fresnelSchlickGGX(float VdotH, float reflectance0, float reflectance90) {
	float t = pow5(1.0 - VdotH);
	float f = reflectance0 + (reflectance90 - reflectance0) * t;
    return float3(f, f, f);
}

// #endif