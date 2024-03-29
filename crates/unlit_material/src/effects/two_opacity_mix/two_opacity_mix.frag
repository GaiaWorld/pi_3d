    vec4 baseColor = v_color;
    float alpha = opacity();

	float glossiness 			= 0.;
	vec3 specularColor 		    = vec3(0., 0., 0.);
	vec3 diffuseColor 		    = vec3(0., 0., 0.);
	vec3 baseAmbientColor		= vec3(1., 1., 1.);
	vec4 refractionColor		= vec4(0., 0., 0., 1.);
    vec4 reflectionColor 		= vec4(0., 0., 0., 1.);
    
    vec3 normalW 				= normalize(v_normal);
	vec3 viewDirectionW		    = WorldSpaceViewDir(v_pos);
    float NdotVunclamped 		= dot(normalW, viewDirectionW);
	float NdotV					= absEps(NdotVunclamped);
	vec3 V 					    = viewDirectionW;
	vec3 N 					    = normalW;
    float absNdV                = abs(NdotVunclamped);

	// ----------------------------------------------------------
	vec3 diffuseBase 					= vec3(0., 0., 0.);
	vec3 specularBase 					= vec3(0., 0., 0.);

    vec4 mainTextureColor   = mainTexture(v_uv, applyUVOffsetSpeed(uMainUVOS));
    baseColor.rgb           *= mainTextureColor.rgb * mainStrength() * mainColor();
    alpha                   *= mainTextureColor.a;

    vec4 opacityData        = opacityTexture(v_uv, applyUVOffsetSpeed(uOpacityUVOS));
    alpha                   *= opacityChannel(opacityData);

    vec4 staticOpacityData  = opacity2Texture(v_uv, applyUVOffsetSpeed(uOpacity2UVOS));
    float staticOpacity     = opacity2Channel(staticOpacityData);

    vec4 mixData            = mixTexture(v_uv, applyUVOffsetSpeed(uMixUVOS));
    float mixValue          = valueByChannel(mixData, uTwoOpacityMixChannel);
    float mixFactor 	    = smoothstep(mixValue - uTwoOpacityMixControl * 0.5, mixValue, uTwoOpacityMixControl);
    alpha 					= mix(1.0, mixValue * sqrt(alpha * staticOpacity), mixFactor);

    vec3 emissiveColor      = emissiveColor();
    emissiveColor           *= emissiveTexture(v_uv, applyUVOffsetSpeed(uEmissiveUVOS)).rgb * emissiveStrength();
    emissiveColor           = emissiveFresnel(absNdV, emissiveColor);

    alpha 					+= opacityFresnel(absNdV);

	vec3 finalSpecular 		= specularBase * specularColor;
    vec3 finalDiffuse       = (diffuseBase * diffuseColor + emissiveColor + PI_Ambient.rgb) * baseColor.rgb;

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
