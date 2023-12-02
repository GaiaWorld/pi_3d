
float3 getBRDFLookup(float NdotV, float perceptualRoughness) {
    float2 UV         = float2(NdotV, perceptualRoughness);
    float4 brdfLookup = GetEnvironmentBRDFTexture(UV);

    return brdfLookup.rgb / brdfLookup.a;
}

float diffuseBRDF_Burley(float NdotL, float NdotV, float VdotH, float roughness) {
    float diffuseFresnelNV = pow5(saturateEps(1.0-NdotL));
    float diffuseFresnelNL = pow5(saturateEps(1.0-NdotV));
    float diffuseFresnel90 = 0.5+2.0*VdotH*VdotH*roughness;
    float fresnel = (1.0+(diffuseFresnel90-1.0)*diffuseFresnelNL) *
    (1.0+(diffuseFresnel90-1.0)*diffuseFresnelNV);
    return fresnel/CUSTOM_PI;
}

float3 getReflectanceFromBRDFLookup(
	const float3 specularEnvironmentR0,
	const float3 specularEnvironmentR90,
	const float3 environmentBrdf
) {
	float3 reflectance = (specularEnvironmentR90 - specularEnvironmentR0) * environmentBrdf.x + specularEnvironmentR0 * environmentBrdf.y;
	return reflectance;
}

float3 getReflectanceFromBRDFLookup(
	const float3 specularEnvironmentR0,
	const float3 environmentBrdf
) {
	float3 reflectance = lerp(environmentBrdf.xxx, environmentBrdf.yyy, specularEnvironmentR0);
	return reflectance;
}

float environmentRadianceOcclusion(float ambientOcclusion, float NdotVUnclamped) {
    float temp = NdotVUnclamped + ambientOcclusion;
    return saturate(Square(temp) - 1.0 + ambientOcclusion);
}
float environmentHorizonOcclusion(float3 view, float3 normal, float3 geometricNormal) {
	// http://marmosetco.tumblr.com/post/81245981087
	float3 reflection = reflect(view, normal);
	float temp = saturate(1.0 + 1.1 * dot(reflection, geometricNormal));
	return Square(temp);
}

// #ifdef SS_TRANSLUCENCY
//     // Pixar diffusion profile
//     // http://graphics.pixar.com/library/ApproxBSSRDF/paper.pdf
//     float3 transmittanceBRDF_Burley(const float3 tintColor, const float3 diffusionDistance, float thickness) {
//         float3 S = 1. / maxEps(diffusionDistance);
//         float3 temp = exp((-0.333333333 * thickness) * S);
//         return tintColor.rgb * 0.25 * (temp * temp * temp + 3.0 * temp);
//     }

//     // Extends the dark area to prevent seams
//     // Keep it energy conserving by using McCauley solution: https://blog.selfshadow.com/2011/12/31/righting-wrap-part-1/
//     float computeWrappedDiffuseNdotL(float NdotL, float w) {
//         float t = 1.0 + w;
//         float invt2 = 1.0 / square(t);
//         return saturate((NdotL + w) * invt2);
//     }
// #endif

float normalDistributionFunction_TrowbridgeReitzGGX(float NdotH, float alphaG) {
    float a2 = Square(alphaG);
    float d = NdotH * NdotH * (a2-1.0) + 1.0;
    return a2 / (3.1415926535897932384626433832795 * d * d);
}
float smithVisibility_GGXCorrelated(float NdotL, float NdotV, float alphaG) {
    float a2 = alphaG*alphaG;
    float GGXV = NdotL*sqrt(NdotV*(NdotV-a2*NdotV)+a2);
    float GGXL = NdotV*sqrt(NdotL*(NdotL-a2*NdotL)+a2);
    return 0.5/(GGXV+GGXL);
}
