
// varying vec2 vUV;

float rand(vec2 n) {
	return fract(cos(dot(n, vec2(12.9898, 4.1414))) * 43758.5453);
}

float noise(vec2 n) {
	const vec2 d = vec2(0.0, 1.0);
	vec2 b = floor(n), f = smoothstep(vec2(0.0), vec2(1.0), fract(n));
	return mix(mix(rand(b), rand(b + d.yx), f.x), mix(rand(b + d.xy), rand(b + d.yy), f.x), f.y);
}

float fbm(vec2 n) {
	float total = 0.0, ampl = amplitude;
	for (int i = 0; i < 4; i++) {
		total += noise(n) * ampl;
		n += n;
		ampl *= 0.5;
	}
	return total;
}

layout(location = 0) out vec4 gl_FragColor;

