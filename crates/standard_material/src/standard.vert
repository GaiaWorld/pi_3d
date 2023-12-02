
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;

    v_pos = worldPos;

    mat3 normalWorld = mat3(finalWorld);
    v_normal = A_NORMAL; // normalize(vec3(finalWorld * vec4(a_normal, 1.0)));
    
    v_color = A_COLOR4;

    v_uv = A_UV;
