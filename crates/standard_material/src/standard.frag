

    vec4 baseColor              = v_color;
    float alpha                 = 1.0;

    float Glossiness            = 0.;
    vec3 diffuseColor           = vec3(1., 1., 1.);
    vec3 specularColor          = vec3(1., 1., 1.);
    vec3 emissiveColor          = vec3(0., 0., 0.);
    vec3 baseAmbientColor       = vec3(1., 1., 1.);
    vec3 LightMap               = vec3(1., 1., 1.);
    vec4 refractionColor        = vec4(0., 0., 0., 1.);
    vec4 reflectionColor        = vec4(0., 0., 0., 1.);

    float depth                 = 0.;
    float dither                = 0.;
    
    vec3 diffuseBase        = vec3(0., 0., 0.);
    vec3 specularBase       = vec3(0., 0., 0.);

    vec3 V                  = WorldSpaceViewDir(P.xyz);
    float NdotV             = dot(N, V);

    float totalAttention;
    computeLighting(depth, dither, NdotV, N, V, P, Glossiness, LightMap, diffuseBase, specularBase, totalAttention);
    if (totalAttention < 0.001) {
        discard;
    }

    vec4 mainTextureColor = mainTexture(v_uv, applyUVOffsetSpeed(uMainUVOS));
    baseColor.rgb *= mainTextureColor.rgb * mainStrength() * mainColor();

    alpha *= mainTextureColor.a;

specularBase       = vec3(0., 0., 0.);
    vec3 finalSpecular      = specularBase * specularColor;
    vec3 finalDiffuse       = (diffuseBase * diffuseColor + emissiveColor) * baseColor.rgb;
    
    vec4 finalColor         = vec4(
        finalDiffuse * baseAmbientColor
        + finalSpecular
        + refractionColor.rgb
        + reflectionColor.rgb
        ,
        alpha
    );

    finalColor.rgb *= finalColor.a;
    // finalColor.rgb = v_color.rgb;

    gl_FragColor = finalColor;
