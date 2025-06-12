
    mat4 influence = readMatrixFromTex(_boneTex, sampler_boneTex, A_JOINT_INC1 * 1.0, bondTexSize.x, 0., bondTexSize.y);
    PI_ObjectToWorld = PI_ObjectToWorld * influence; 
