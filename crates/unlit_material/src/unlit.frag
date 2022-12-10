
    vec4 baseColor = vec4(1., 1., 1., 1.);
    float alpha = 1.0;

    vec4 mainTextureColor = texture(sampler2D(_MainTex, sampler_MainTex), v_uv);
    baseColor.rgb = mainTextureColor.rgb;
    alpha *= mainTextureColor.a;

    baseColor.rgb *= emissive.rgb * emissive.a;

    // float level = dot(v_normal, vec3(0., 0., -1.));
    // baseColor.rgb = mix(baseColor.rgb, v_normal, 0.5);
    // baseColor.rgb = (v_pos + vec3(1., 1., 1.)) / 2.;

    gl_FragColor = vec4(baseColor.rgb, alpha);
