
    vec2 uv = gl_FragCoord.xy / vec2(800, 600).xy;
    float o = n1noise(200.0 * uv);

    float alpha = 1.0;

    vec3 pixel_color = vec3(emissive.rgb * o);

    vec3 fog_color = vec3(1.0, 1.0, 1.0); // 雾颜色。

    pixel_color = mix(fog_color, pixel_color, v_dist);

    gl_FragColor = vec4(pixel_color, alpha);
