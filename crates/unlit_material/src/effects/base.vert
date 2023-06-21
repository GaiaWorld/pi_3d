
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;
    gl_Position.z = gl_Position.z * 0.5 + 0.5;
    // gl_Position = position;

    v_pos = worldPos.xyz;

    mat3 normalWorld = mat3(finalWorld);
    v_normal = A_NORMAL;
    
    v_uv = A_UV;
    v_color = A_COLOR4;
