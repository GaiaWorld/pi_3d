

float3 computeDiffuseLighting(
    float NdotL,
    float NdotV,
    float VdotH,
    float attenuation,
    float roughness
) {
    float diffuseTerm = diffuseBRDF_Burley(NdotL, NdotV, VdotH, roughness);
    
    // #if defined(_USE_DIFFUSE_RAMP)
    // return diffuseTerm * attenuation * GetDiffuseRamp(NdotL) * diffuseColor;
    // #else
    return float3(diffuseTerm * attenuation * NdotL);
    // #endif
}

float3 computeSpecularLighting(
    float NdotL,
    float NdotV,
    float NdotH,
    float attenuation,
    float roughness,
    float3 fresnel,
    float alphaG
) {
//     float alphaG            = convertRoughnessToAverageSlope(roughness);
    float distribution      = normalDistributionFunction_TrowbridgeReitzGGX(NdotH, alphaG);
    float smithVisibility   = smithVisibility_GGXCorrelated(NdotL, NdotV, alphaG);
    float3 specTerm         = fresnel * float3(distribution, distribution, distribution) * float3(smithVisibility, smithVisibility, smithVisibility);

    // #if defined(_USE_SPECULAR_RAMP)
    // return specTerm * attenuation * GetSpecularRamp(NdotH) * NdotL * lightColor;
    // #else
    return specTerm * attenuation * NdotL;
    // #endif
}

void BRDFLighting(
    float3 fresnel
    , float roughness
    , float attenuation
    , float alphaG
    , float NdotV
    , float VdotH
    , float NdotH
    // , float NdotLUnclamped
    , float NdotL
    // , float NdotVD
    // , float NdotHD
    // , float NdotLD
    , out float3 diffuse
    , out float3 specular
) {
    diffuse = computeDiffuseLighting(
        NdotL,
        NdotV,
        VdotH,
        attenuation,
        roughness
    );
    
    specular = computeSpecularLighting(
        NdotL,
        NdotV,
        NdotH,
        attenuation,
        roughness,
        fresnel,
        alphaG
    );
}

void conputeLightingDirect(
    float3 fresnel
    , float roughness
    , float attenuation
    , float alphaG
    , float NdotV
    , float VdotH
    , float NdotH
    // , float NdotLUnclamped
    , float NdotL
    // , float NdotVD
    // , float NdotHD
    // , float NdotLD
    , out float3 diffuseBase
    , out float3 specularBase
) {
    BRDFLighting(
            fresnel
            , roughness
            , attenuation
            , alphaG
            , NdotV
            , VdotH
            , NdotH
            // , NdotLUnclamped
            , NdotL
            // , NdotVD
            // , NdotHD
            // , NdotLD
            , diffuseBase
            , specularBase
    );
}

void computeLightingSpot(
    const float angleOffset,
    const float glossiness,
    out float attenuation,
    const float NdotV,
    const float3 N,
    const float3 V,
    const float3 L,
    const float3 D,
    const float3 color,
    out float3 diffuseBase,
    out float3 specularBase
) {
    float cosAngle = max(0., dot(D, -L));
    if (cosAngle >= 0.0001) {
        cosAngle = max(0., pow(cosAngle, 1.));
        attenuation *= cosAngle;

        float3 H        = normalize(V + L);
        float NdotL     = dot(N, L);
        float NdotH     = dot(N, H);
        
        diffuseBase     = NdotL * color * attenuation;

        float specComp  = pow(max(0., NdotH), max(1., glossiness));
        specularBase    = specComp * color * attenuation;

        return;
    }
    diffuseBase = float3(0., 0., 0.);
    specularBase = float3(0., 0., 0.);
}

void computeLighting(
    // const float depth,
    // const float dither,
    const float NdotV,
    const float NdotVUnClamped,
    const float3 N,
    const float3 V,
    const float4 P,
    const float3 F0,
    const float3 F90,
    const float roughness,
    const float alphaG,
    const float3 lightmapColor,
    out float3 lightDiffuse,
    out float3 lightSpecular,
    out float totalAttention
) {
    totalAttention = 0.;
    float shadow = 1.0;
    float3 diffuseBase = float3(0., 0., 0.);
    float3 specularBase = float3(0., 0., 0.);

    lightDiffuse = float3(0., 0., 0.);
    lightSpecular = float3(0., 0., 0.);

    uint _MDirectLightCount = _MLightsCount.x;

    float attenuation = 1.0;
    for (uint i = 0; i < _MDirectLightCount; i++) {
        uint lid        = _MLightsIndexs[i].x;
        if (_DirectLightDirection[lid].w < 0. && !hasDirectShadow(lid)) { continue; }

        float llayer                = _DirectLightColor[lid].w;
        float3 L                    = -normalize(_DirectLightDirection[lid].xyz);
        float3 H                    = normalize(V + L);
        float VdotH                 = saturate(dot(V, H));
        float NdotH                 = saturateEps(dot(N, H));
        float NdotLUnclamped        = dot(N, L);
        float NdotL                 = saturateEps(NdotLUnclamped);
        float attenuation           = 1.0;
        float3 color                = _DirectLightColor[lid].xyz;
        float3 fresnel              = fresnelSchlickGGX(VdotH, F0, F90);

        // float lightRadius           = 64.0;
        // float lightDistance         = length(-_DirectLightDirection[lid].xyz);
        // float lightDistanceSquared  = Square(lightDistance);
        // float lightRoughness        = adjustRoughnessFromLightProperties(roughness, lightRadius, lightDistance);
        float lightRoughness        = roughness;

        conputeLightingDirect(
            fresnel,
            lightRoughness,
            attenuation,
            alphaG,
            NdotV,
            VdotH,
            NdotH,
            NdotL,
            diffuseBase,
            specularBase
        );

        totalAttention += attenuation;

        computeDirectShadow(lid, P, NdotLUnclamped, shadow);

        // lightDiffuse    += float3(shadow);
        lightDiffuse    += color * diffuseBase * shadow;
        lightSpecular   += color * specularBase * shadow;
    }
    shadow = 1.0;
    for (uint i = 0; i < _MDirectLightCount; i++) {
        uint lid        = _MLightsIndexs[i].x;
        if (_DirectLightDirection[lid].w > 0. || hasDirectShadow(lid)) { continue; }

        float llayer                = _DirectLightColor[lid].w;
        float3 L                    = -normalize(_DirectLightDirection[lid].xyz);
        float3 H                    = normalize(V + L);
        float VdotH                 = saturate(dot(V, H));
        float NdotH                 = saturateEps(dot(N, H));
        float NdotLUnclamped        = dot(N, L);
        float NdotL                 = saturateEps(NdotLUnclamped);
        float attenuation           = 1.0;
        float3 color                = _DirectLightColor[lid].xyz;
        float3 fresnel              = fresnelSchlickGGX(VdotH, F0, F90);

        // float lightRadius           = 64.0;
        // float lightDistance         = length(-_DirectLightDirection[lid].xyz);
        // float lightDistanceSquared  = Square(lightDistance);
        // float lightRoughness        = adjustRoughnessFromLightProperties(roughness, lightRadius, lightDistance);
        float lightRoughness        = roughness;

        conputeLightingDirect(
            fresnel,
            lightRoughness,
            attenuation,
            alphaG,
            NdotV,
            VdotH,
            NdotH,
            NdotL,
            diffuseBase,
            specularBase
        );

        totalAttention += attenuation;

        lightDiffuse    += color * diffuseBase * shadow;
        lightSpecular   += color * specularBase * shadow;
    }

    uint _MPointLightCount = _MLightsCount.y;
    for (uint i = 0; i < _MPointLightCount; i++) {
        uint lid        = _MLightsIndexs[i].y;
        if (_PointLightPosition[lid].w < 0.) { continue; }

        float llayer                    = _PointLightColor[lid].w;
        float range                     = _PointLightData[lid].z;
        float inverseSquaredRange       = _PointLightData[lid].w;
        float3 lightOffset              = _PointLightPosition[lid].xyz - P.xyz;
        float3 L                        = normalize(lightOffset);
        float  lightDistanceSquared     = dot(lightOffset, lightOffset);
        float  lightDistance            = sqrt(lightDistanceSquared);

        float3 H                        = normalize(V + L);
        float  VdotH                    = saturate(dot(V, H));
        float  NdotH                    = saturateEps(dot(N, H));
        float  NdotLUnclamped           = dot(N, L);
        float  NdotL                    = saturateEps(NdotLUnclamped);
        float3 color                    = _PointLightColor[lid].xyz;

        float  attenuation              = computeDistanceLightFalloff(lightOffset, lightDistanceSquared, range, inverseSquaredRange);
        
        float lightRadius               = 64.0;
        float  lightRoughness           = adjustRoughnessFromLightProperties(roughness, lightRadius, lightDistance);
        float3 fresnel                  = fresnelSchlickGGX(VdotH, F0, F90);

        BRDFLighting(
            fresnel
            , lightRoughness
            , attenuation
            , alphaG
            , NdotV
            , VdotH
            , NdotH
            // , NdotLUnclamped
            , NdotL
            // , NdotVD
            // , NdotHD
            // , NdotLD
            , diffuseBase
            , specularBase
        );

        totalAttention += attenuation;

        lightDiffuse    += color * diffuseBase * shadow;
        lightSpecular   += color * specularBase * shadow;
    }

    uint _MSpotLightCount = _MLightsCount.z;
    for (uint i = 0; i < _MSpotLightCount; i++) {
        uint lid        = _MLightsIndexs[i].z;
        if (_SpotLightPosition[lid].w < 0.) { continue; }

        float llayer                    = _SpotLightColor[lid].w;
        float angleScale                = _SpotLightData[lid].x;
        float angleOffset               = _SpotLightData[lid].y;
        float range                     = _SpotLightData[lid].z;
        float inverseSquaredRange       = _SpotLightData[lid].w;
        float cosHalfAngle              = angleScale;
        float exponent                  = angleOffset;
        float3 lightOffset              = _SpotLightPosition[lid].xyz - P.xyz;
        float3 L                        = normalize(lightOffset);
        float  lightDistanceSquared     = dot(lightOffset, lightOffset);
        float  lightDistance            = sqrt(lightDistanceSquared);

        float3 H                        = normalize(V + L);
        float  VdotH                    = saturate(dot(V, H));
        float  NdotH                    = saturateEps(dot(N, H));
        float  NdotLUnclamped           = dot(N, L);
        float  NdotL                    = saturateEps(NdotLUnclamped);
        float3 color                    = _SpotLightColor[lid].xyz;

        float3 lightDirection           = normalize(_SpotLightDirection[lid].xyz);

        float  attenuation              = computeDistanceLightFalloff(lightOffset, lightDistanceSquared, range, inverseSquaredRange);
        attenuation                     *= computeDirectionalLightFalloff(lightDirection, L, cosHalfAngle, exponent, angleScale, angleOffset);
        
        float lightRadius               = 64.0;
        float  lightRoughness           = adjustRoughnessFromLightProperties(roughness, lightRadius, lightDistance);

        float3 fresnel                  = fresnelSchlickGGX(VdotH, F0, F90);

        BRDFLighting(
            fresnel
            , lightRoughness
            , attenuation
            , alphaG
            , NdotV
            , VdotH
            , NdotH
            // , NdotLUnclamped
            , NdotL
            // , NdotVD
            // , NdotHD
            // , NdotLD
            , diffuseBase
            , specularBase
        );

        totalAttention += attenuation;

        lightDiffuse    += color * diffuseBase * shadow;
        lightSpecular   += color * specularBase * shadow;
    }
}
