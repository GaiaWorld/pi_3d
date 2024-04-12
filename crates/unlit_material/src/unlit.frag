
    vec4 baseColor = v_color;
    float alpha = 1.0;

    vec4 mainTextureColor = mainTexture(v_uv, applyUVOffsetSpeed(uMainUVOS), uMainAtlas);
    baseColor.rgb *= mainTextureColor.rgb * mainStrength() * mainColor();
    alpha *= mainTextureColor.a;

    gl_FragColor = vec4(baseColor.rgb, alpha);
