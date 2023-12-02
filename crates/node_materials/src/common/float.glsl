
#define E 2.71828
#define CUSTOM_PI 3.1415926535897932384626433832795
#define Epsilon 0.0000001

#define lerp mix
#define ddx dFdx
#define ddy dFdy


float saturate(float x)  {
	return clamp(x, 0., 1.0);
}
vec2 saturate(vec2 x)  {
	return clamp(x, vec2(0.), vec2(1.0));
}
vec3 saturate(vec3 x)  {
	return clamp(x, vec3(0.), vec3(1.0));
}

float Square (float x) {
	return x * x;
}

float pow5(float v) {
	float s = v*v;
	return s*s*v;
}


float saturateEps(float x) {
	return clamp(x, Epsilon, 1.0);
}
float absEps(float x) {
	return abs(x) + Epsilon;
}
float maxEps(float x) {
	return max(x, Epsilon);
}
