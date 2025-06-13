
layout(location = 0) out vec4 gl_FragColor;
void shader(const MatParam matParam) {
    vec4 baseColor = v_color;
    float alpha = opacity(matParam);

	const float glossiness 			= 0.;
	const vec3 specularColor 		    = vec3(0., 0., 0.);
	const vec3 diffuseColor 		    = vec3(0., 0., 0.);
	const vec3 baseAmbientColor		= vec3(1., 1., 1.);
	const vec4 refractionColor		= vec4(0., 0., 0., 1.);
    const vec4 reflectionColor 		= vec4(0., 0., 0., 1.);

	// ----------------------------------------------------------
	const vec3 diffuseBase 					= vec3(0., 0., 0.);
	const vec3 specularBase 					= vec3(0., 0., 0.);

    const vec4 mainTextureColor   = mainTexture(v_uv, applyUVOffsetSpeed(matParam.uMainUVOS), matParam);
    baseColor.rgb           *= mainTextureColor.rgb * mainStrength(matParam) * mainColor(matParam);
    alpha                   *= mainTextureColor.a;

    const vec4 opacityData        = opacityTexture(v_uv, applyUVOffsetSpeed(matParam.uOpacityUVOS), matParam);
    const float opacityValue      = opacityChannel(opacityData, matParam);

    if (cutoff(opacityValue - 0.001, matParam)) {
        discard;
    }

    vec3 emissiveColor = emissiveColor(matParam);
    emissiveColor *= emissiveTexture(v_uv, applyUVOffsetSpeed(matParam.uEmissiveUVOS), matParam).rgb * emissiveStrength(matParam);

	const vec3 finalSpecular 		= specularBase * specularColor;
    const vec3 finalDiffuse       = (diffuseBase * diffuseColor + emissiveColor) * baseColor.rgb;

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
    // finalColor.rgb *= finalColor.a;

    gl_FragColor = finalColor;
}
