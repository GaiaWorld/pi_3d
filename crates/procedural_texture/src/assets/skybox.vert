
    mat4 finalWorld = PI_MATRIX_P * PI_MATRIX_V;

    vec4 positionUpdate = vec4(A_POSITION * 2.0, 1.);

    positionUpdate = PI_ObjectToWorld * positionUpdate;
    gl_Position = PI_MATRIX_VP * positionUpdate;

