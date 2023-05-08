
#define E 2.71828
#define CUSTOM_PI 3.1415926535897932384626433832795
#define Epsilon 0.0000001

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
