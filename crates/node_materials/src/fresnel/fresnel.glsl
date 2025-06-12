
float computeFresnelTerm(const float absVN, const float bias, const float power) {
    float fresnelTerm = pow(bias + absVN, power);
    return clamp(fresnelTerm, 0., 1.);
}

vec3 fresnelSchlickGGX(const float VdotH, const vec3 reflectance0, const vec3 reflectance90) {
	float t = pow5(1.0 - VdotH);
    return reflectance0 + (reflectance90 - reflectance0) * t;
}

vec3 fresnelSchlickGGX(const float VdotH, const float reflectance0, const float reflectance90) {
	float t = pow5(1.0 - VdotH);
	float f = reflectance0 + (reflectance90 - reflectance0) * t;
    return vec3(f, f, f);
}
