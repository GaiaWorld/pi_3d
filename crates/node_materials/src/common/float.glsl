
#define E 2.71828
#define CUSTOM_PI 3.1415926535897932384626433832795
#define Epsilon 0.0000001

#define lerp mix
#define ddx dFdx
#define ddy dFdy

float saturate(const float x)  {
	return clamp(x, 0., 1.0);
}
vec2 saturate(const vec2 x)  {
	return clamp(x, vec2(0.), vec2(1.0));
}
vec3 saturate(const vec3 x)  {
	return clamp(x, vec3(0.), vec3(1.0));
}

float Square (const float x) {
	return x * x;
}

float pow5(const float v) {
	float s = v*v;
	return s*s*v;
}

float saturateEps(const float x) {
	return clamp(x, Epsilon, 1.0);
}
float absEps(const float x) {
	return abs(x) + Epsilon;
}
float maxEps(const float x) {
	return max(x, Epsilon);
}
