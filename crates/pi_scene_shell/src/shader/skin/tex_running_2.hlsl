
    mat4 influence   = readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC2[0], bondTexSize.x, 0., bondTexSize.y)  * A_JOINT_WEG2[0];
    influence       += readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC2[1], bondTexSize.x, 0., bondTexSize.y) * A_JOINT_WEG2[1];
    PI_ObjectToWorld = PI_ObjectToWorld * influence;
