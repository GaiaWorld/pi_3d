#define sampleReflectionLod(s, c, l) textureLod(s, c, l)

float3 computeCubicCoords(
    float3 viewDirection,
    float3 normalW,
    float4x4 reflectionMatrix
) {
    float3 coords = (reflectionMatrix * float4(reflect(viewDirection, normalW), 0.0)).xyz;

    return coords;
}

float3 computeReflectionCoords() {
    return float3(0., 0., 0.);
}

void createReflectionCoords(
    float4 vPositionW,
    float3 viewDirection,
    float3 normalW,
    float4x4 reflectionMatrix,
    out float3 reflectionCoords
) {
    float3 reflectionVector = computeCubicCoords(-viewDirection, normalW, reflectionMatrix);

    reflectionCoords = reflectionVector;
}

void createReflectionCoords(
    float4 vPositionW,
    float3 viewDirection,
    float3 normalW,
    float4x4 reflectionMatrix,
    out float2 reflectionCoords
) {
    float3 reflectionVector = computeCubicCoords(-viewDirection, normalW, reflectionMatrix);

    reflectionCoords = reflectionVector.xy;
    reflectionCoords.y = 1.0 - reflectionCoords.y;
}

float getLODFromAlphaG(
    float cubeMapDimensionPixels,
    float microsurfaceAverageSlope
) {
    float microsurfaceAverageSlopeTexels = cubeMapDimensionPixels * microsurfaceAverageSlope;
    return log2(microsurfaceAverageSlopeTexels);
}

void sampleReflectionTexture(
    float alphG,
    float3 reflectionMicrosurfaceInfos,
    const float3 reflectionCoords,
    out float4 environmentRadiance
) {
    float reflectionLOD = getLODFromAlphaG(reflectionMicrosurfaceInfos.x, alphG);
    reflectionLOD = reflectionLOD * reflectionMicrosurfaceInfos.y + reflectionMicrosurfaceInfos.z;

    environmentRadiance = sampleReflectionLod(samplerCube(_EnvironmentTexture, sampler_EnvironmentTexture), reflectionCoords, reflectionLOD);
    environmentRadiance.rgb = toLinearSpace(environmentRadiance.rgb);
    environmentRadiance.rgb = environmentRadiance.rgb / environmentRadiance.a;
}

float3 computeEnvironmentIrradianceNoSH(
    float3 normal
) {
    // Fast method for evaluating a fixed spherical harmonics function on the sphere (e.g. irradiance or radiance).
    // Cost: 24 scalar operations on modern GPU "scalar" shader core, or 8 multiply-adds of 3D vectors:
    // "Function Cost 24	24x mad"

    // Note: the lower operation count compared to other methods (e.g. Sloan) is by further
    // taking advantage of the input 'normal' being normalised, which affords some further algebraic simplification.
    // Namely, the SH coefficients are first converted to spherical polynomial (SP) basis, then 
    // a substitution is performed using Z^2 = (1 - X^2 - Y^2).

    // As with other methods for evaluation spherical harmonic, the input 'normal' is assumed to be normalised (or near normalised).
    // This isn't as critical as it is with other calculations (e.g. specular highlight), but the result will be slightly incorrect nonetheless.
    float Nx = normal.x;
    float Ny = normal.y;
    float Nz = normal.z;

    vec3 C1     = _SphericalZZ.rgb;
    vec3 Cx     = _SphericalX.rgb;
    vec3 Cy     = _SphericalY.rgb;
    vec3 Cz     = _SphericalZ.rgb;
    vec3 Cxx_zz = _SphericalXX_ZZ.rgb;
    vec3 Cyy_zz = _SphericalYY_ZZ.rgb;
    vec3 Cxy    = _SphericalXY.rgb;
    vec3 Cyz    = _SphericalYZ.rgb;
    vec3 Czx    = _SphericalZX.rgb;

    vec3 a1 = Cyy_zz * Ny + Cy;
    vec3 a2 = Cyz * Nz + a1;
    vec3 b1 = Czx * Nz + Cx;
    vec3 b2 = Cxy * Ny + b1;
    vec3 b3 = Cxx_zz * Nx + b2;
    vec3 t1 = Cz  * Nz + C1;
    vec3 t2 = a2  * Ny + t1;
    vec3 t3 = b3  * Nx + t2;

    return t3;
}

void reflectionBlockCUBENoSH(
    float4 vPositionW,
    float3 viewDirection,
    float3 normalW,
    float4x4 reflectionMatrix,
    float alphaG,
    float3 vReflectionMicrosurfaceInfos,
    out float4 environmentRadiance, 
    out float3 environmentIrradiance, 
    out float3 reflectionCoords
) {
    createReflectionCoords(vPositionW, viewDirection, normalW, reflectionMatrix, reflectionCoords);
    sampleReflectionTexture(alphaG, vReflectionMicrosurfaceInfos, reflectionCoords, environmentRadiance);

    float3 irradianceVector = (reflectionMatrix * float4(normalW, 1.)).xyz;
    // environmentIrradiance = ShadeSH9(half4(normalW, 1.));
    environmentIrradiance = computeEnvironmentIrradianceNoSH(irradianceVector);
    // environmentIrradiance *= vReflectionColor.rgb;
}

void ReflectionBlock(
    float4 vPositionW,
    float3 viewDirection,
    float3 normalW,
    float alphaG,
    out float4 environmentRadiance, 
    out float3 environmentIrradiance
) {

    float3 vReflectionMicrosurfaceInfos = float3(_SphericalX.w, _SphericalY.w, 0.0);
    float2 vReflectionInfos = float2(1.0, 0.0);
    float3 vReflectionColor = float3(1., 1., 1.);
    float4x4 reflectionMatrix = float4x4(float4(1., 0., 0., 0.), float4(0., 1., 0., 0.), float4(0., 0., 1., 0.), float4(0., 0., 0., 1.));

    environmentRadiance     = float4(0., 0., 0., 0.);
    environmentIrradiance   = float3(0., 0., 0.);
    float3 reflectionCoords = float3(0., 0., 0.);

    reflectionBlockCUBENoSH(
        vPositionW,
        viewDirection,
        normalW,
        reflectionMatrix,
        alphaG,
        vReflectionMicrosurfaceInfos,
        environmentRadiance,
        environmentIrradiance,
        reflectionCoords
    );

    environmentRadiance.rgb *= vReflectionInfos.x;
    environmentRadiance.rgb *= vReflectionColor.rgb;

    environmentIrradiance *= vReflectionColor.rgb;
}