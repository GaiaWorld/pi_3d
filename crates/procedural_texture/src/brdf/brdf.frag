
    float alpha = 1.0;

    vec4 mainTextureColor = texture(sampler2D(_MainTex, sampler_MainTex), v_UV);;

    gl_FragColor = vec4(fromRGBD(mainTextureColor), alpha);
