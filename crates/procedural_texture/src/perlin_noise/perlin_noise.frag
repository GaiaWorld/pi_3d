
    vec2 uv = gl_FragCoord.xy / vec2(width, height);
    
    vec2 pos = vec2(uv * size);

    // Use the noise function
    float n = noise(pos);

    gl_FragColor = vec4(vec3(n), 1.0);
