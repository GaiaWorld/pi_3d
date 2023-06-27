    vec4 baseColor = v_color;
    float alpha = opacity();

	float glossiness 			= 0.;
	vec3 specularColor 		    = vec3(0., 0., 0.);
	vec3 diffuseColor 		    = vec3(0., 0., 0.);
	vec3 baseAmbientColor		= vec3(1., 1., 1.);
	vec4 refractionColor		= vec4(0., 0., 0., 1.);
    vec4 reflectionColor 		= vec4(0., 0., 0., 1.);

	// ----------------------------------------------------------
	vec3 diffuseBase 					= vec3(0., 0., 0.);
	vec3 specularBase 					= vec3(0., 0., 0.);

    vec4 mainTextureColor   = mainTexture(v_uv, applyUVOffsetSpeed(uMainUVOS));
    baseColor.rgb           *= mainTextureColor.rgb * mainStrength() * mainColor();
    alpha                   *= mainTextureColor.a;

    vec4 opacityData        = opacityTexture(v_uv, applyUVOffsetSpeed(uOpacityUVOS));
    float opacityValue      = opacityChannel(opacityData);

    if (cutoff(opacityValue - 0.001)) {
        discard;
    }

    vec3 emissiveColor = emissiveColor();
    emissiveColor *= emissiveTexture(v_uv, applyUVOffsetSpeed(uEmissiveUVOS)).rgb * emissiveStrength();

	vec3 finalSpecular 		= specularBase * specularColor;
    vec3 finalDiffuse       = (diffuseBase * diffuseColor + emissiveColor) * baseColor.rgb;

    vec4 finalColor 		= vec4(
                                finalDiffuse * baseAmbientColor
                                + 
                                finalSpecular
                                +
                                refractionColor.rgb
                                + 
                                reflectionColor.rgb
                                ,
                                alpha
                            );

	finalColor				= max(finalColor, 0.0);

    // Premulty
    finalColor.rgb *= finalColor.a;

    gl_FragColor = finalColor;
