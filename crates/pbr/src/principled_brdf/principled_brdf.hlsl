// #ifndef PRINCIPLED_BRDF
// #define PRINCIPLED_BRDF

// #include "../Color/ColorAbout.hlsl"
// #include "../BRDF.hlsl"
// #include "./DirectLighting.hlsl"
// #include "./PointAndSpotLighting.hlsl"
// #include "./Reflectivity.hlsl"
// #include "./Reflection.hlsl"

struct PrincipledBRDFInput {
    float3 baseColor;

    float subsurface;
    float3 subsurfaceRadius;
    float3 subsurfaceColor;
    float subsurfaceIOR;
    float subsurfaceAnisotropy;

    float metallic;

    float specular;
    float3 specularTint;

    float roughness;

    float anisotropic;
    float anisotropicRotation;

    float sheen;
    float3 sheenTint;

    float clearCoat;
    float clearcoatRoughness;

    float IOR;
    // float metallicF0Factor;
    float4 metallicReflectanceColor;

    float transmission;
    float transmissionRoughness;

    float3 emission;
    float emissionStrength;
    
    float alpha;

    float3 normal;
    float3 clearcoatNormal;
    float3 tangent;
};

struct PrincipledBRDFOutput {
    float3 ambient;
    float3 diffuse;
    float3 specular;
    float3 irradiance;
    float3 radiance;
    float3 irradianceColor;
    float3 radianceColor;
    float3 emission;
    float3 sheen;
    float3 sheenRadiance;
    float3 clearcoat;
    float3 clearcoatRadiance;
    float3 refraction;
    float3 environmentBrdf;
    float3 surfaceAlbedo;
    float  alpha;
    float  microSurface;
};

#define MINIMUMVARIANCE 0.0005
float convertRoughnessToAverageSlope(float roughness) {
    return Square(roughness)+0.0005;
}
float fresnelGrazingReflectance(float reflectance0) {
    // typical incident reflectance range (between 4% to 100%)
    // reflectance0 -> 4%
    // reflectance90 -> 100%
    float reflectance90 = saturate(reflectance0 * 25.0);
    return reflectance90;
}
float2 getAARoughnessFactors(float3 normal) {
    // #ifdef SPECULARAA
        float3 nDfdx = ddx(normal);
        float3 nDfdy = ddy(normal);
        float slopeSquare = max(dot(nDfdx, nDfdx), dot(nDfdy, nDfdy));

        // Vive analytical lights roughness factor
        float geometricRoughnessFactor = pow(saturate(slopeSquare), 0.333);

        // Adapt linear roughness(alphaG) to geometric curvature of the current pixel.
        float geometryAlphaGFactor = sqrt(slopeSquare);
        // 
        geometryAlphaGFactor *= 0.75;

        return float2(geometricRoughnessFactor, geometryAlphaGFactor);
    // #else
    //     return float2(0., 0.);
    // #endif
}
// http://www.jcgt.org/published/0008/01/03/
// http://advances.realtimerendering.com/s2018/Siggraph%202018%20HDRP%20talk_with%20notes.pdf
float3 getEnergyConservationFactor(const float3 specularEnvironmentR0, const float3 environmentBrdf) {
    return 1.0 + specularEnvironmentR0 * (1.0 / environmentBrdf.y - 1.0);
}


// 这里把计算留在Shader中了 ior = 1.5 时 计算结果 f0 = 0.04
float4 computeMetallicReflectanceFactors(float ior, float metallicF0Factor, float4 metallicReflectanceColor) {
	float outside_ior = 1.;

	float f0 = pow((ior - outside_ior) / (ior + outside_ior), 2);

	float metallicF90		= metallicF0Factor;
	metallicReflectanceColor *= f0 * metallicF0Factor;

	return float4(
		metallicReflectanceColor.rgb,
		metallicF90
	);
}

PrincipledBRDFOutput ApplyPrincipledBRDF(
    PrincipledBRDFInput input
    , float4 position
    , float3 ambientColor
    , float3 lightmapColor
    , float3 V
    , float3 geometricNormal
    , out float lightmapUsed
) {
    PrincipledBRDFOutput output;

    output.ambient          = float3(0., 0., 0.);
    output.diffuse          = float3(0., 0., 0.);
    output.specular         = float3(0., 0., 0.);
    output.irradiance       = float3(0., 0., 0.);
    output.irradianceColor  = float3(0., 0., 0.);
    output.radiance         = float3(0., 0., 0.);
    output.radianceColor    = float3(0., 0., 0.);
    output.emission         = input.emission * input.emissionStrength;
    output.sheen            = float3(0., 0., 0.);
    output.sheenRadiance    = float3(0., 0., 0.);
    output.clearcoat        = float3(0., 0., 0.);
    output.clearcoatRadiance= float3(0., 0., 0.);
    output.refraction       = float3(0., 0., 0.);
    output.alpha            = 1.0;

// Geometry
    float NdotVUnClamped        = dot(input.normal, V);
    float NdotV                 = saturate(absEps(NdotVUnClamped));
    float alphaG                = convertRoughnessToAverageSlope(input.roughness);
    float2 AARoughnessFactors   = getAARoughnessFactors(input.normal);
    // #ifdef SPECULARAA
        // Adapt linear roughness (alphaG) to geometric curvature of the current pixel.
        // alphaG += AARoughnessFactors.y;
    // #endif

    float3 environmentBrdf      = getBRDFLookup(NdotV, input.roughness);

    // 就是反射因子 - F0 = metallicReflectanceFactors.rgb, F90 = metallicReflectanceFactors.a
    float4 metallicReflectanceFactors = computeMetallicReflectanceFactors(
        input.IOR, 
        1.0, // input.metallicF0Factor, 
        input.metallicReflectanceColor
    );

    // 即 glossing 光泽度 与 粗糙的刚好互补 0~1
    float microSurface;
    // 漫射
    float3 surfaceAlbedo;
    // 反射率 - 金属 1.0 - 非金属 0.04
    float3 surfaceReflectivityColor;
    float3 metallicF0;

// Reflectivity - 由材质反照率、金属度、粗糙度、材质反照率控制因子 计算 - 漫反射、反射、光泽度
    Reflectivity(
        input.baseColor
        , metallicReflectanceFactors
        , input.metallic
        , input.roughness
        , microSurface
        , surfaceAlbedo
        , surfaceReflectivityColor
        , metallicF0
    );

    float4 finalRadiance = float4(0., 0., 0., 0.);
    float3 finalIrradiance = float3(0., 0., 0.);
// Reflection - 环境辐照度
    ReflectionBlock(
        position,
        V,
        input.normal,
        alphaG,
        finalRadiance,
        finalIrradiance
    );
    output.irradiance       = finalIrradiance.rgb;
    output.radiance         = finalRadiance.rgb;

// ___________________ Compute Reflectance aka R0 F0 info _________________________
    float reflectance = max(
            max(
                surfaceReflectivityColor.r,
                surfaceReflectivityColor.g
            ),
            surfaceReflectivityColor.b
        );
    float3 specularEnvironmentR0    = surfaceReflectivityColor.rgb;
    float3 specularEnvironmentR90   = float3(metallicReflectanceFactors.a, metallicReflectanceFactors.a, metallicReflectanceFactors.a);


    float3 specularEnvironmentReflectance = getReflectanceFromBRDFLookup(
        specularEnvironmentR0,
        specularEnvironmentR90,
        environmentBrdf
    );
    output.radiance                 *= specularEnvironmentReflectance;

// _____________________________ Energy Conservation  ___________________________
    float energyConservationFactor  = 1.0;
#ifdef _ENERGY_CONSERVATION
    energyConservationFactor        = getEnergyConservationFactor(specularEnvironmentR0, environmentBrdf);
#endif
    output.radiance                 *= energyConservationFactor;

// Lighting
    float totalAttention;
    float3 diffuseResult;
    float3 specularResult;
    lightmapUsed = 0.0;
    computeLighting(
        // depth,
        // dither,
        NdotV,
        NdotVUnClamped,
        input.normal,
        V,
        position,
        specularEnvironmentR0,
        specularEnvironmentR90,
        input.roughness,
        alphaG,
        lightmapColor,
        diffuseResult,
        specularResult,
        totalAttention
    );
    // lightmapUsed = DirectLighting(
    //     renderingLayerMask
    //     , shadowData
    //     , lightmapColor
    //     , position.xyz
    //     , input.normal
    //     , V
    //     , specularEnvironmentR0
    //     , specularEnvironmentR90
    //     , NdotV
    //     , microSurface
    //     , alphaG
    //     , diffuseResult
    //     , specularResult
    // );
    // output.diffuse      += diffuseResult;
    // output.specular     += specularResult;
    // PointAndSpotLighting(
    //     renderingLayerMask
    //     , shadowData
    //     , lightmapColor
    //     , position.xyz
    //     , input.normal
    //     , V
    //     , specularEnvironmentR0
    //     , specularEnvironmentR90
    //     , NdotV
    //     , microSurface
    //     , alphaG
    //     , diffuseResult
    //     , specularResult
    // );
    output.diffuse          = diffuseResult;
    output.specular         = specularResult;

    output.diffuse          *= surfaceAlbedo;
    output.diffuse          = max(output.diffuse, float3(0.0, 0., 0.));

    output.specular         = max(output.specular, float3(0.0, 0., 0.));
    // output.specular         *= energyConservationFactor;

    output.ambient          = ambientColor * surfaceAlbedo;

    output.environmentBrdf  = environmentBrdf;
    output.surfaceAlbedo    = surfaceAlbedo;
    
    output.irradianceColor  = output.irradiance * surfaceAlbedo;
    output.microSurface     = microSurface;

    return output;
}

// #endif
