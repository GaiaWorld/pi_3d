
vec2 applyUVOffsetSpeed(vec2 uvSpeed) {
	vec2 result = vec2(0., 0.);
	if (abs(uvSpeed.x) > 0.0001) {
		result.x = PI_Time.y / uvSpeed.x;
	}
	if (abs(uvSpeed.y) > 0.0001) {
		result.y = PI_Time.y / uvSpeed.y;
	}

	return result;
}
