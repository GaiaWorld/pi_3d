
layout(location = 0) out vec4 gl_FragColor;
void shader() {
    vec3 baseColor = vec3(1., 1., 1.);
    float alpha = opacity();

	const float glossiness 			    = 0.;

	// ----------------------------------------------------------

    const vec4 mainTextureColor   = mainTexture(v_uv);
    baseColor               *= mainTextureColor.rgb * mainStrength() * mainColor();
    alpha                   *= mainTextureColor.a;

    const vec4 opacityData        = opacityTexture(v_uv2);
    const float opacityValue      = opacityChannel(opacityData);

    if (cutoff(opacityValue - 0.001)) {
        discard;
    }

    vec3 emissiveColor = emissiveColor();
    emissiveColor *= emissiveTexture(v_uv3).rgb * emissiveStrength();

	const vec3 specularBase 					= vec3(0., 0., 0.);
	const vec3 specularColor 		    = vec3(0., 0., 0.);
	const vec3 finalSpecular 		= specularBase * specularColor;
	const vec3 diffuseColor 		    = vec3(0., 0., 0.);
	const vec3 diffuseBase 					= vec3(0., 0., 0.);
    const vec3 finalDiffuse       = (diffuseBase * diffuseColor + emissiveColor) * baseColor;

    vec4 finalColor		    = max(vec4(
                                finalDiffuse
                                ,
                                alpha
                            ), 0.0);

    // Premulty
    // finalColor.rgb *= finalColor.a;

    gl_FragColor = finalColor;
}
