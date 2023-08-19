vec3 WorldSpaceViewDir(vec3 positionWS) {
	return normalize(
		PI_MATRIX_P[3][3] * PI_MATRIX_P[2].xyz
		+
		(1.0 - PI_MATRIX_P[3][3]) * (PI_CAMERA_POSITION.xyz - positionWS)
	);
}
