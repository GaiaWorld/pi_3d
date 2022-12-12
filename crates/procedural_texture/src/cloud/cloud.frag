
	vec2 vUV = gl_FragCoord.xy / vec2(width, height);
	vec2 p = vUV * 12.0;
	vec4 c = mix(skyColor, cloudColor, fbm(p));
	gl_FragColor = c;
