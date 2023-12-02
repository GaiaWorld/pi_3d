
float adjustRoughnessFromLightProperties(float roughness, float lightRadius, float lightDistance) {
    // #if defined(USEPHYSICALLIGHTFALLOFF) || defined(USEGLTFLIGHTFALLOFF)
        float lightRoughness = lightRadius / lightDistance;
        float totalRoughness = saturate(lightRoughness + roughness);
        return totalRoughness;
    // #else
    //     return roughness;
    // #endif
}

// ___________________________ Distance Light __________________________________

float computeDistanceLightFalloff_Standard(float3 lightOffset, float range) {
    return max(0., 1.0 - length(lightOffset) / range);
}

float computeDistanceLightFalloff_Physical(float lightDistanceSquared) {
    return 1.0 / maxEps(lightDistanceSquared);
}

float computeDistanceLightFalloff_GLTF(float lightDistanceSquared, float inverseSquaredRange) {
    float lightDistanceFalloff 	= 1.0 / maxEps(lightDistanceSquared);

    float factor 				= lightDistanceSquared*inverseSquaredRange;
    float attenuation 			= saturate(1.0 - factor*factor);
    attenuation 				*= attenuation;

    // Smooth attenuation of the falloff defined by the range.
    lightDistanceFalloff 		*= attenuation;

    return lightDistanceFalloff;
}

float computeDistanceLightFalloff(float3 lightOffset, float lightDistanceSquared, float range, float inverseSquaredRange) {
    // #ifdef USEPHYSICALLIGHTFALLOFF
    //     return computeDistanceLightFalloff_Physical(lightDistanceSquared);
    // #elif defined(USEGLTFLIGHTFALLOFF)
        return computeDistanceLightFalloff_GLTF(lightDistanceSquared, inverseSquaredRange);
    // #else
    //     return computeDistanceLightFalloff_Standard(lightOffset, range);
    // #endif
}

// ___________________________ Directional Light __________________________________

float computeDirectionalLightFalloff_Standard(float3 lightDirection, float3 directionToLightCenterW, float cosHalfAngle, float exponent) {
    float falloff = 0.0;

    float cosAngle = maxEps(dot(-lightDirection, directionToLightCenterW));
    // eg: 点光圆锥范围内才有光照
    if (cosAngle >= cosHalfAngle) {
        falloff = max(0., pow(cosAngle, exponent));
    }

    return falloff;
}

float computeDirectionalLightFalloff_Physical(float3 lightDirection, float3 directionToLightCenterW, float lightAngleScale, float lightAngleOffset) {
    // CPU 端计算
    // float lightAngleScale = 1.0 / max(0.001, (cosInner - cosOuter))
    // float lightAngleOffset = -cosOuter * angleScale

    float cd = dot(-lightDirection, directionToLightCenterW);
    float falloff = saturate(cd * lightAngleScale + lightAngleOffset);

    falloff *= falloff;
    return falloff;
}

float computeDirectionalLightFalloff_GLTF(float3 lightDirection, float3 directionToLightCenterW, float lightAngleScale, float lightAngleOffset) {
    float cd = dot(-lightDirection, directionToLightCenterW);
    float falloff = saturate(cd * lightAngleScale + lightAngleOffset);
    falloff *= falloff;
    return falloff;
}

float computeDirectionalLightFalloff(float3 lightDirection, float3 directionToLightCenterW, float cosHalfAngle, float exponent, float lightAngleScale, float lightAngleOffset) {
    // #ifdef USEPHYSICALLIGHTFALLOFF
    //     return computeDirectionalLightFalloff_Physical(lightDirection, directionToLightCenterW, lightAngleScale, lightAngleOffset);
    // #elif defined(USEGLTFLIGHTFALLOFF)
        return computeDirectionalLightFalloff_GLTF(lightDirection, directionToLightCenterW, lightAngleScale, lightAngleOffset);
    // #else
    //     return computeDirectionalLightFalloff_Standard(lightDirection, directionToLightCenterW, cosHalfAngle, exponent);
    // #endif
}
