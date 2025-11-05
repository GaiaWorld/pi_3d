
#define LinearEncodePowerApprox 2.2
#define GammaEncodePowerApprox 1.0 / 2.2

float toLinearSpace(const float v) {
return pow(v, LinearEncodePowerApprox);
}
vec3 toLinearSpace(const vec3 v) {
return vec3(
pow(v.r, LinearEncodePowerApprox),
pow(v.g, LinearEncodePowerApprox),
pow(v.b, LinearEncodePowerApprox)
);
}
vec4 toLinearSpace(const vec4 v) {
return vec4(
pow(v.r, LinearEncodePowerApprox),
pow(v.g, LinearEncodePowerApprox),
pow(v.b, LinearEncodePowerApprox),
v.a
);
}

float toGammaSpace(const float v) {
return pow(v, GammaEncodePowerApprox);
}
vec3 toGammaSpace(const vec3 v) {
return vec3(
pow(v.r, GammaEncodePowerApprox),
pow(v.g, GammaEncodePowerApprox),
pow(v.b, GammaEncodePowerApprox)
);
}
vec4 toGammaSpace(const vec4 v) {
return vec4(
pow(v.r, GammaEncodePowerApprox),
pow(v.g, GammaEncodePowerApprox),
pow(v.b, GammaEncodePowerApprox),
v.a
);
}
