

	vec2 fragCoord = gl_FragCoord.xy;

    float time = iTime * 0.03 + 200.0 * 0.01;
	
// #ifdef AA
//     vec3 color = vec3(0.0);
//     for(int i = -1; i <= 1; i++) {
//         for(int j = -1; j <= 1; j++) {
//         	vec2 uv = fragCoord+vec2(i,j)/3.0;
//     		color += getPixel(uv, time);
//         }
//     }
//     color /= 9.0;
// #else
    vec3 color = getPixel(fragCoord, time);
// #endif
    
    // post
	gl_FragColor = vec4(pow(color,vec3(0.65)), 1.0);
