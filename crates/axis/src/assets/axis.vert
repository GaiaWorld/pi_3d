
    vec4 positionUpdate = vec4(A_POSITION, 1.);

    positionUpdate = PI_ObjectToWorld * positionUpdate;
    gl_Position = PI_MATRIX_VP * positionUpdate;
    v_color = A_COLOR4;
