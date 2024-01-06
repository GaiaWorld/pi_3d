
    vec4 baseColor = v_color;
    float alpha = opacity();

    float glossiness 			= 0.;
    vec3 diffuseColor 		    = vec3(0., 0., 0.);
    vec3 baseAmbientColor		= vec3(1., 1., 1.);

    // ----------------------------------------------------------

    vec4 mainTextureColor   = mainTexture(v_uv, applyUVOffsetSpeed(uMainUVOS));
    baseColor.rgb           *= mainTextureColor.rgb * mainStrength();
    alpha                   *= mainTextureColor.a;

    diffuseColor            = mainColor();

    vec4 opacityData        = opacityTexture(v_uv2, applyUVOffsetSpeed(uOpacityUVOS));
    alpha                   *= opacityChannel(opacityData);

    if (cutoff(alpha)) {
        discard;
    }

    vec3 finalDiffuse       = diffuseColor * baseColor.rgb;
    
    vec4 finalColor 		= vec4(
        finalDiffuse * baseAmbientColor
        ,
        alpha
    );

    finalColor				= max(finalColor, 0.0);

    // Premulty
    finalColor.rgb          *= finalColor.a;

    gl_FragColor = finalColor;
