
struct lightingInfo
{
    // Pre Falloff Info
    vec3 diffuse;
    vec3 specular;
    float NdotL;
};

lightingInfo computeLighting(const vec3 viewDirectionW, const vec3 vPositionW, const vec3 vNormal, const vec4 lightData, const vec3 diffuseColor, const vec3 specularColor, const float range, const float glossiness) {
    lightingInfo result;

    vec3 direction = mix(lightData.xyz - vPositionW, -lightData.xyz, lightData.w);
    float attenuation = mix(max(0.0, 1.0 - length(direction) / range), 1.0, lightData.w);

    vec3 lightVectorW = normalize(direction);

    result.ndl = max(0., dot(vNormal, lightVectorW));

    result.diffuse = ndl * diffuseColor * attenuation;

	// Specular
	vec3 angleW = normalize(viewDirectionW + lightVectorW);
	float specComp = max(0., dot(vNormal, angleW));
	specComp = pow(specComp, max(1., glossiness));

	result.specular = specComp * specularColor * attenuation;
}

