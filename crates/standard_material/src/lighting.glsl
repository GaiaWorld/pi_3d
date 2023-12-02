
#define P v_pos
#define N v_normal

void conputeLightingDirect(
    const float glossiness,
    const float attenuation,
    const float NdotV,
    const vec3 N,
    const vec3 V,
    const vec3 L,
    const vec3 color,
    out vec3 diffuseBase,
    out vec3 specularBase
) {
        vec3 H          = normalize(V + L);
        float NdotL     = dot(N, L);
        float NdotH     = dot(N, H);

        diffuseBase     = NdotL * color * attenuation;

        float specComp  = pow(max(0., NdotH), max(1., glossiness));
        specularBase    = specComp * color * attenuation;
}

void computeLightingSpot(
    const float angleOffset,
    const float glossiness,
    out float attenuation,
    const float NdotV,
    const vec3 N,
    const vec3 V,
    const vec3 L,
    const vec3 D,
    const vec3 color,
    out vec3 diffuseBase,
    out vec3 specularBase
) {
    float cosAngle = max(0., dot(D, -L));
    if (cosAngle >= 0.0001) {
        cosAngle = max(0., pow(cosAngle, 1.));
        attenuation *= cosAngle;

        vec3 H          = normalize(V + L);
        float NdotL     = dot(N, L);
        float NdotH     = dot(N, H);
        
        diffuseBase     = NdotL * color * attenuation;

        float specComp  = pow(max(0., NdotH), max(1., glossiness));
        specularBase    = specComp * color * attenuation;

        return;
    }
    diffuseBase = vec3(0., 0., 0.);
    specularBase = vec3(0., 0., 0.);
}

void computeLighting(
    const float depth,
    const float dither,
    const float NdotV,
    const vec3 N,
    const vec3 V,
    const vec4 P,
    const float glossiness,
    const vec3 lightmapColor,
    out vec3 lightDiffuse,
    out vec3 lightSpecular,
    out float totalAttention
) {
    totalAttention = 0.;
    float shadow = 1.0;
    vec3 diffuseBase = vec3(0., 0., 0.);
    vec3 specularBase = vec3(0., 0., 0.);

    lightDiffuse = vec3(0., 0., 0.);
    lightSpecular = vec3(0., 0., 0.);

    uint _MDirectLightCount = _MLightsCount.x;

    float attenuation = 1.0;
    for (uint i = 0; i < _MDirectLightCount; i++) {
        uint lid        = _MLightsIndexs[i].x;
        if (_DirectLightDirection[lid].w < 0. && !hasDirectShadow(lid)) { continue; }

        float llayer    = _DirectLightColor[lid].w;
        vec3 L          = -normalize(_DirectLightDirection[lid].xyz);
        vec3 color      = _DirectLightColor[lid].xyz;

        conputeLightingDirect(glossiness, attenuation, NdotV, N, V, L, color, diffuseBase, specularBase);
        totalAttention += attenuation;

        computeDirectShadow(lid, P, dot(N, L), shadow);

        // lightDiffuse    += vec3(shadow);
        lightDiffuse    += diffuseBase * shadow;
        lightSpecular   += specularBase * shadow;
    }
    shadow = 1.0;
    for (uint i = 0; i < _MDirectLightCount; i++) {
        uint lid        = _MLightsIndexs[i].x;
        if (_DirectLightDirection[lid].w > 0. || hasDirectShadow(lid)) { continue; }

        float llayer    = _DirectLightColor[lid].w;
        vec3 L          = -normalize(_DirectLightDirection[lid].xyz);
        vec3 color      = _DirectLightColor[lid].xyz;

        conputeLightingDirect(glossiness, attenuation, NdotV, N, V, L, color, diffuseBase, specularBase);
        totalAttention += attenuation;

        lightDiffuse    += diffuseBase * shadow;
        lightSpecular   += specularBase * shadow;
    }

    uint _MPointLightCount = _MLightsCount.y;
    for (uint i = 0; i < _MPointLightCount; i++) {
        uint lid        = _MLightsIndexs[i].y;
        if (_PointLightPosition[lid].w < 0.) { continue; }

        float llayer            = _PointLightColor[lid].w;
        float range             = _PointLightData[lid].z;
        float invSquaredRange   = _PointLightData[lid].w;
        vec3 L                  = _PointLightPosition[lid].xyz - P.xyz;
        attenuation             = max(0., 1.0 - length(L) / range);
        L                       = normalize(L);
        vec3 color              = _PointLightColor[lid].xyz;

        conputeLightingDirect(glossiness, attenuation, NdotV, N, V, L, color, diffuseBase, specularBase);
        totalAttention += attenuation;


        lightDiffuse    += diffuseBase * shadow;
        lightSpecular   += specularBase * shadow;
    }

    uint _MSpotLightCount = _MLightsCount.z;
    for (uint i = 0; i < _MSpotLightCount; i++) {
        uint lid        = _MLightsIndexs[i].z;
        if (_SpotLightPosition[lid].w < 0.) { continue; }

        float llayer            = _SpotLightColor[lid].w;
        float angleScale        = _SpotLightData[lid].x;
        float angleOffset       = _SpotLightData[lid].y;
        float range             = _SpotLightData[lid].z;
        float invSquaredRange   = _SpotLightData[lid].w;
        vec3 L                  = _SpotLightPosition[lid].xyz - P.xyz;
        attenuation             = max(0., 1.0 - length(L) / range);
        L                       = normalize(L);
        vec3 color              = _SpotLightColor[lid].xyz;
        vec3 D                  = normalize(_SpotLightDirection[lid].xyz);

        // conputeLightingDirect(glossiness, attenuation, NdotV, N, V, L, color, diffuseBase, specularBase);
        computeLightingSpot(angleOffset, glossiness, attenuation, NdotV, N, V, L, D, color, diffuseBase, specularBase);
        totalAttention += attenuation;


        lightDiffuse    += diffuseBase * shadow;
        lightSpecular   += specularBase * shadow;
    }
}
