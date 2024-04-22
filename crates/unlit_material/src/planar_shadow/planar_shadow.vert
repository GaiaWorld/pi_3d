
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;

    vec4 shadowPos = worldPos;
    //灯光方向
    vec3 lightDir = normalize(uLightDir.xyz);
    //阴影的世界空间坐标（低于地面的部分不做改变）
    shadowPos.y = min(worldPos.y , uLightDir.w);
    shadowPos.xz = worldPos.xz - normalize(lightDir.xz) * max(0 , worldPos.y - uLightDir.w); 

    gl_Position = PI_MATRIX_VP * shadowPos;
    v_color = uShadowColor;