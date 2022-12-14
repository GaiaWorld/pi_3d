

    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;
    // gl_Position = position;
    
    v_UV = A_UV;