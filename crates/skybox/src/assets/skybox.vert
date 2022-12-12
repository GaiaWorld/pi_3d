
    mat4 finalWorld = PI_MATRIX_P * PI_MATRIX_V;

    vec4 positionUpdate = vec4(A_POSITION * 2.0, 1.);

    positionUpdate = PI_ObjectToWorld * positionUpdate;
    gl_Position = PI_MATRIX_VP * positionUpdate;
    // gl_Position = vec4(a_position * 0.5, 1.);

    mat3 normalWorld = mat3(PI_ObjectToWorld);
    v_normal = normalize(normalWorld * A_NORMAL);
    v_normal = A_NORMAL;
    v_dist = applyFog(distance(positionUpdate.xyz, PI_CAMERA_POSITION.xyz), PI_CAMERA_POSITION.xyz, PI_CAMERA_POSITION.xyz - positionUpdate.xyz);
