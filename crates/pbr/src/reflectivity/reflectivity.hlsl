// #ifndef BABYLON_REFLECTIVITY
// #define BABYLON_REFLECTIVITY

void Reflectivity(
    const float3 baseColor
    , const float4 metallicReflectanceFactors
    , float metallic
    , float roughness
    , out float microSurface
    , out float3 surfaceAlbedo
    , out float3 surfaceReflectivityColor
    , out float3 metallicF0
) {
    microSurface                    = 1.0 - roughness;

    microSurface                    = saturate(microSurface);
    metallicF0                      = metallicReflectanceFactors.rgb;
    surfaceAlbedo                   = lerp(baseColor.rgb * (float3(1., 1., 1.) - metallicF0), float3(0., 0., 0.), float3(metallic));
    surfaceReflectivityColor        = lerp(metallicF0, baseColor.rgb, float3(metallic));
}

// #endif