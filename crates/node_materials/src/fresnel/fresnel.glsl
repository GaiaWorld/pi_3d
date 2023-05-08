
float computeFresnelTerm(float absVN, float bias, float power) {
    float fresnelTerm = pow(bias + absVN, power);
    return clamp(fresnelTerm, 0., 1.);
}

vec3 fresnelSchlickGGX(float VdotH, vec3 reflectance0, vec3 reflectance90) {
	float t = pow5(1.0 - VdotH);
    return reflectance0 + (reflectance90 - reflectance0) * t;
}

vec3 fresnelSchlickGGX(float VdotH, float reflectance0, float reflectance90) {
	float t = pow5(1.0 - VdotH);
	float f = reflectance0 + (reflectance90 - reflectance0) * t;
    return vec3(f, f, f);
}
