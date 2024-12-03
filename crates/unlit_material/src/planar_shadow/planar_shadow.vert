
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;

    // vec4 shadowPos = worldPos;
    // //灯光方向
    // vec3 lightDir = normalize(matParam.uLightDir.xyz);
    // //阴影的世界空间坐标（低于地面的部分不做改变）
    // shadowPos.y = min(worldPos.y , matParam.uLightDir.w);
    // shadowPos.xz = worldPos.xz - normalize(lightDir.xz) * max(0 , worldPos.y - matParam.uLightDir.w); 
    // gl_Position = PI_MATRIX_VP * shadowPos;

    mat4 planarMatrix = mat4(
        matParam.uPlanarRow1,
        matParam.uPlanarRow2,
        matParam.uPlanarRow3,
        vec4(0., 0., 0., 1.)
    );

    vec4 shadowPos = planarMatrix * worldPos;
    vec3 diff = shadowPos.xyz - worldPos.xyz;
    float flag = step(0., dot(diff, matParam.uPlanarDir.xyz));
    gl_Position = PI_MATRIX_VP * mix(worldPos, shadowPos, flag);
    gl_Position.z -= 0.001 * flag;

    v_color = matParam.uShadowColor;
