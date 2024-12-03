
    vec4 baseColor = v_color;
    float alpha = 1.0;

    vec4 mainTextureColor = mainTexture(v_uv, applyUVOffsetSpeed(matParam.uMainUVOS), matParam);
    baseColor.rgb *= mainTextureColor.rgb * mainStrength(matParam) * mainColor(matParam);
    alpha *= mainTextureColor.a;

    gl_FragColor = vec4(baseColor.rgb, alpha);
