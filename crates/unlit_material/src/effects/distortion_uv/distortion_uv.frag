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

    vec2 maskValue          = (maskTexture(v_uv).rg * 2.0 - 1.0) + maskUVOffset * matParam.uFlowMode;

    vec4 mainTextureColor   = mainTexture(v_uv2 + maskValue * matParam.uStrength);
    baseColor.rgb           *= mainTextureColor.rgb * mainStrength() * mainColor();
    alpha                   *= mainTextureColor.a;
    
    vec4 opacityData        = opacityTexture(v_uv3);
    alpha                   *= opacityChannel(opacityData);

    vec4 finalColor = vec4(baseColor.rgb, alpha);
    finalColor.rgb *= finalColor.a;

    gl_FragColor = finalColor;
