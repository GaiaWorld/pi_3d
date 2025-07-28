
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;
     
    // gl_Position = position;

    v_pos = worldPos.xyz;

    mat3 normalWorld = mat3(finalWorld);
    v_normal = A_NORMAL;
    
    v_uv  = A_UV * matParam.uMainTilloff.xy + matParam.uMainTilloff.zw + applyUVOffsetSpeed(matParam.uMainUVOS);
    v_uv2 = A_UV * matParam.uOpacityTilloff.xy + matParam.uOpacityTilloff.zw + applyUVOffsetSpeed(matParam.uOpacityUVOS);
    v_uv3 = A_UV * matParam.uOpacity2Tilloff.xy + matParam.uOpacity2Tilloff.zw + applyUVOffsetSpeed(matParam.uOpacity2UVOS);
    v_uv4 = A_UV * matParam.uMixTilloff.xy + matParam.uMixTilloff.zw + applyUVOffsetSpeed(matParam.uMixUVOS);
    v_uv5 = A_UV * matParam.uEmissiveTilloff.xy + matParam.uEmissiveTilloff.zw + applyUVOffsetSpeed(matParam.uEmissiveUVOS);
    v_color = A_COLOR4;
